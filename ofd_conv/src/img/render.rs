#[allow(dead_code)]
#[allow(unused_variables)]
mod font;
mod path;
mod text;

use std::io::{Cursor, Read, Seek};
use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;

use eyre::Result;
use eyre::{eyre, OptionExt};
use skia_safe::canvas::SrcRectConstraint;
use skia_safe::Color4f;
use skia_safe::Matrix;
use skia_safe::PaintCap;
use skia_safe::PaintJoin;
use skia_safe::Rect;
use skia_safe::{BlendMode, Data, Image, Paint};
use skia_safe::{Canvas, ImageInfo, Surface};
use skia_safe::{ClipOp, Color};
use tracing::{debug, error, warn};

use crate::error::MyError;
use crate::img::render::font::AggFontMgr;
use crate::img::render::path::draw_path_object;
use crate::img::render::text::draw_text_object;
use ofd_base::common::Cap;
use ofd_base::common::CtColor;
use ofd_base::common::Join;
use ofd_base::file::annotation::AnnotationXmlFile;
use ofd_base::file::document::CtPageArea;
use ofd_base::file::document::DocumentXmlFile;
use ofd_base::file::page::PageXmlFile;
use ofd_base::file::page::{ImageObject, VtGraphicUnit};
use ofd_base::file::res::DrawParam;
use ofd_base::file::res::SRGB;
use ofd_base::StArray;
use ofd_base::StBox;
use ofd_base::StRefId;
use ofd_rw::{from_bytes, Ofd, Resources};
use ofd_sign::decode_sign;

struct RenderCtx<'a, I> {
    _ofd: Ofd<I>,
    canvas: &'a Canvas,
    draw_param_stack: DrawParamStack,
    resources: &'a Resources,
    font_mgr: &'a mut AggFontMgr<I>,
}

#[allow(unused)]
pub struct Render<I> {
    ofd: Ofd<I>,
    dpi: i32,
    font_mgr: AggFontMgr<I>,
}

impl<I: Read + Seek> Render<I> {
    pub fn new(ofd: Ofd<I>, fallback_font: impl AsRef<str>) -> Result<Self> {
        // let o = ofd.0.clone()
        let font_mgr = AggFontMgr::builder(ofd.clone(), fallback_font).build()?;
        Ok(Render {
            ofd,
            dpi: 300,
            font_mgr,
        })
    }

    pub fn new_with_fm(ofd: Ofd<I>, font_mgr: AggFontMgr<I>) -> Self {
        Render {
            ofd,
            dpi: 300,
            font_mgr,
        }
    }

    pub fn render_page(&mut self, doc_index: usize, page_index: usize) -> Result<Surface> {
        let dpi = self.dpi;

        let doc = self.ofd.document_by_index(doc_index)?;
        let doc_xml = &doc.content;

        let page = self.ofd.page_by_index(doc_index, page_index)?;
        let page_xml = &page.content;

        let templates = self.ofd.templates_for_page(doc_index, page_index)?;
        let template_pages = &templates
            .iter()
            .map(|i| &i.content)
            .collect::<Vec<&PageXmlFile>>();
        let pa = decide_size(page_xml, template_pages, doc_xml);
        let size = (
            mm2px_i32(pa.physical_box.w, dpi),
            mm2px_i32(pa.physical_box.h, dpi),
        );
        let resources = self.ofd.resources_for_page(doc_index, page_index)?;

        self.font_mgr.load_page(doc_index, page_index);

        let mut sur = create_surface(size)?;
        let can = sur.canvas();

        // bgc
        can.draw_color(Color::WHITE, BlendMode::Color);
        let scale = calc_scale(dpi);
        can.scale((scale, scale));
        let mut ctx = RenderCtx {
            _ofd: self.ofd.clone(),
            canvas: can,
            draw_param_stack: DrawParamStack::new(),
            resources: &resources,
            font_mgr: &mut self.font_mgr,
        };

        debug!("drawing templates");
        for tpl in template_pages {
            draw_page(&mut ctx, tpl)?;
        }
        debug!("drawing page");
        draw_page(&mut ctx, &page.content)?;

        debug!("drawing annotations");
        let anno_vec = self.ofd.annotations_for_page(doc_index, page_index)?;
        for anno in anno_vec {
            draw_anno(&mut ctx, &anno)?;
        }

        #[cfg(feature = "sign")]
        {
            debug!("drawing e seals");
            let signs = self.ofd.signatures_for_page(doc_index, page_index)?;
            if let Some(sign_vec) = signs {
                for (sign_file, sign) in sign_vec {
                    ctx.canvas.save();
                    apply_boundary(ctx.canvas, sign.boundary);

                    // clip
                    if let Some(clip) = sign.clip {
                        let clip = Rect::from_xywh(clip.x, clip.y, clip.w, clip.h);
                        ctx.canvas.clip_rect(clip, ClipOp::Intersect, true);
                    }

                    // ctx.canvas.cl
                    let path = sign_file.resolve(&sign_file.signed_value);

                    let sign_bytes = self.ofd.bytes(path)?;
                    let s = decode_sign(&sign_bytes)?;
                    let appearance = s.appearance();
                    match appearance.r#type.to_lowercase().as_str() {
                        "ofd" => {
                            let ofd = from_bytes(&appearance.data)?;
                            let mut render = Render::new(ofd, "宋体")?;
                            let mut sur = render.render_stamp()?;
                            let image = sur.image_snapshot();
                            let src_bound = image.bounds().into();
                            let dst = Rect::from_wh(sign.boundary.w, sign.boundary.h);

                            let mut paint = Paint::default();
                            paint.set_anti_alias(true);

                            ctx.canvas.draw_image_rect(
                                image,
                                Some((&src_bound, SrcRectConstraint::Strict)),
                                dst,
                                &paint,
                            );
                        }
                        &_ => {
                            warn!("unrecognized appearance type: {}", appearance.r#type);
                        }
                    }
                    ctx.canvas.restore();
                }
            }
        }

        can.restore();
        Ok(sur)
    }

    pub fn render_stamp(&mut self) -> Result<Surface> {
        let doc_index = 0;
        let page_index = 0;
        let dpi = self.dpi;

        let doc = self.ofd.document_by_index(doc_index)?;
        let doc_xml = &doc.content;

        let page = self.ofd.page_by_index(doc_index, page_index)?;
        let page_xml = &page.content;

        let templates = self.ofd.templates_for_page(doc_index, page_index)?;
        let template_pages = &templates
            .iter()
            .map(|i| &i.content)
            .collect::<Vec<&PageXmlFile>>();
        let pa = decide_size(page_xml, template_pages, doc_xml);
        let size = (
            mm2px_i32(pa.physical_box.w, dpi),
            mm2px_i32(pa.physical_box.h, dpi),
        );
        let resources = self.ofd.resources_for_page(doc_index, page_index)?;

        self.font_mgr.load_page(doc_index, page_index);

        let mut sur = create_surface(size)?;
        let can = sur.canvas();

        // bgc
        // can.draw_color(Color::WHITE, BlendMode::Color);
        let scale = calc_scale(dpi);
        can.scale((scale, scale));
        let mut ctx = RenderCtx {
            _ofd: self.ofd.clone(),
            canvas: can,
            draw_param_stack: DrawParamStack::new(),
            resources: &resources,
            font_mgr: &mut self.font_mgr,
        };

        debug!("drawing templates");
        for tpl in template_pages {
            draw_page(&mut ctx, tpl)?;
        }
        debug!("drawing page");
        draw_page(&mut ctx, &page.content)?;

        debug!("drawing annotations");
        let anno_vec = self.ofd.annotations_for_page(doc_index, page_index)?;
        for anno in anno_vec {
            draw_anno(&mut ctx, &anno)?;
        }
        ctx.canvas.restore();
        Ok(sur)
    }

    pub fn render_template(&mut self, doc_index: usize, page_index: usize) -> Result<Surface> {
        let dpi = self.dpi;

        let doc = self.ofd.document_by_index(doc_index)?;
        let doc_xml = &doc.content;

        let page = self.ofd.page_by_index(doc_index, page_index)?;
        let page_xml = &page.content;

        let templates = self.ofd.templates_for_page(doc_index, page_index)?;
        let template_pages = &templates
            .iter()
            .map(|i| &i.content)
            .collect::<Vec<&PageXmlFile>>();
        let pa = decide_size(page_xml, template_pages, doc_xml);
        let size = (
            mm2px_i32(pa.physical_box.w, dpi),
            mm2px_i32(pa.physical_box.h, dpi),
        );
        let resources = self.ofd.resources_for_page(doc_index, page_index)?;

        self.font_mgr.load_page(doc_index, page_index);

        let mut sur = create_surface(size)?;
        let can = sur.canvas();

        // bgc
        can.draw_color(Color::WHITE, BlendMode::Color);
        let scale = calc_scale(dpi);
        can.scale((scale, scale));
        let mut ctx = RenderCtx {
            _ofd: self.ofd.clone(),
            canvas: can,
            draw_param_stack: DrawParamStack::new(),
            resources: &resources,
            font_mgr: &mut self.font_mgr,
        };

        debug!("drawing templates");
        for tpl in template_pages {
            draw_page(&mut ctx, tpl)?;
        }
        can.restore();
        Ok(sur)
    }
}

fn draw_anno<I: Read + Seek>(ctx: &mut RenderCtx<I>, anno: &AnnotationXmlFile) -> Result<()> {
    for annot in &anno.annot {
        if !annot.visible.unwrap_or(true) {
            continue;
        }
        if let Some(objects) = &annot.appearance.objects {
            draw_object(ctx, objects)
        }
    }
    Ok(())
}

struct DrawParamStack {
    draw_params: Vec<DrawParam>,
}

impl DrawParamStack {
    /// create a new stack
    fn new() -> Self {
        Self {
            draw_params: vec![],
        }
    }

    fn push(&mut self, draw_param: Option<DrawParam>) {
        if let Some(draw_param) = draw_param {
            self.draw_params.push(draw_param);
        }
    }
    fn pop(&mut self, draw_param: Option<DrawParam>) -> Option<DrawParam> {
        if draw_param.is_some() {
            self.draw_params.pop()
        } else {
            None
        }
    }

    /// is empty
    fn is_empty(&self) -> bool {
        self.draw_params.is_empty()
    }

    /// get stock color
    fn get_stroke_color(
        &self,
        element_stroke_color: Option<&CtColor>,
        resources: &Resources,
        fallback: Color4f,
    ) -> Color4f {
        // test element color
        if let Some(stroke_color) = element_stroke_color {
            return resolve_color(stroke_color, resources).unwrap();
        }

        // find in draw param
        let dp_stroke_color = self
            .draw_params
            .iter()
            .rev()
            .find_map(|dp| dp.stroke_color.clone());
        if let Some(ct_color) = dp_stroke_color {
            return resolve_color(&ct_color, resources).unwrap();
        }

        // use fallback
        fallback
    }

    /// get join
    fn get_join(&self, element_join: Option<&Join>, fallback: &Join) -> PaintJoin {
        // test element join
        let rj = if let Some(j) = element_join {
            j
        } else if let Some(x) = self
            .draw_params
            .iter()
            .rev()
            .find_map(|dp| dp.join.as_ref())
        {
            x
        } else {
            fallback
        };
        match rj {
            Join::Miter => PaintJoin::Miter,
            Join::Round => PaintJoin::Round,
            Join::Bevel => PaintJoin::Bevel,
        }
    }

    fn get_miter_limit(&self, element_miter_limit: Option<f32>, fallback: f32) -> f32 {
        // test element miter limit
        if let Some(ml) = element_miter_limit {
            ml
        } else if let Some(x) = self
            .draw_params
            .iter()
            .rev()
            .find_map(|dp| dp.miter_limit.as_ref())
        {
            *x
        } else {
            fallback
        }
    }

    fn get_cap(&self, element_cap: Option<&Cap>, fallback: &Cap) -> PaintCap {
        // test element cap
        let rc = if let Some(c) = element_cap {
            c
        } else if let Some(x) = self.draw_params.iter().rev().find_map(|dp| dp.cap.as_ref()) {
            x
        } else {
            fallback
        };
        match rc {
            Cap::Butt => PaintCap::Butt,
            Cap::Round => PaintCap::Round,
            Cap::Square => PaintCap::Square,
        }
    }

    fn get_fill_color(
        &self,
        element_fill_color: Option<&CtColor>,
        resources: &Resources,
        fallback: Color4f,
    ) -> Color4f {
        // test element color
        if let Some(fill_color) = element_fill_color {
            return resolve_color(fill_color, resources).unwrap();
        }

        // find in draw param
        let dp_fill_color = self
            .draw_params
            .iter()
            .rev()
            .find_map(|dp| dp.fill_color.clone());
        if let Some(ct_color) = dp_fill_color {
            return resolve_color(&ct_color, resources).unwrap();
        }

        // use fallback
        fallback
    }
}

fn create_surface(size: (i32, i32)) -> Result<Surface> {
    let ii = ImageInfo::new_s32(size, skia_safe::AlphaType::Unpremul);
    let surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
    Ok(surface)
}

fn apply_boundary(can: &Canvas, boundary: StBox) {
    let br = Rect::from_xywh(boundary.x, boundary.y, boundary.w, boundary.h);
    let matrix = Matrix::translate(boundary.get_tl());
    can.clip_rect(br, None, true);
    can.concat(&matrix);
}

fn apply_ctm(can: &Canvas, ctm: Option<&StArray<f32>>) {
    if ctm.is_none() {
        return;
    }
    let ctm = ctm.unwrap();
    assert_eq!(ctm.0.len(), 6, "ctm len must be 6");
    // dbg!(ctm);
    let mat = Matrix::new_all(
        ctm.0[0], ctm.0[2], ctm.0[4], ctm.0[1], ctm.0[3], ctm.0[5], 0.0, 0.0, 1.0,
    );
    can.concat(&mat);
}

fn resolve_color(ct_color: &CtColor, resources: &Resources) -> Result<Color4f> {
    // ct_color
    let cs = ct_color
        .color_space
        .map(|cs_id| {
            let cs = resources.get_color_space_by_id(cs_id);
            if cs.is_none() {
                warn!(
                    "can't find color space! colorspace_id = {}, fallback to default colorspace.",
                    cs_id
                );
            }
            cs.or_else(|| {
                if let Some(cs_id) = resources.default_cs {
                    // looking for default color space
                    let cs = resources.get_color_space_by_id(cs_id);
                    if cs.is_none() {
                        warn!(
                            "can not find default color space! colorspace_id = {},  fallback to SRGB.",
                            cs_id
                        );
                    }
                    cs
                } else {
                    None
                }
            })
            .unwrap_or(&SRGB)
        }).unwrap_or(&SRGB);

    let val = if let Some(val) = &ct_color.value {
        Ok(val)
    } else if let Some(idx) = ct_color.index {
        // plate color
        if let Some(pal) = &cs.palette {
            let c = pal.cv.get(idx).ok_or_eyre("plate color not found!")?;
            Ok(c)
        } else {
            Err(eyre!("color palette not found"))
        }
    } else {
        // TODO: may be it is a shadow
        Err(eyre!("invalid color!"))
    }?;

    // value color
    assert_eq!(
        cs.r#type.channel_count(),
        val.0.len(),
        "color and color space mismatch"
    );
    let bpc = cs.bits_per_component.unwrap_or(8);
    let max_val = (1 << bpc) - 1;
    let a = (ct_color.alpha.unwrap_or(255) / 255) as f32;
    let r = match &cs.r#type {
        ofd_base::file::res::Type::RGB => {
            let r = val.0[0] as f32 / max_val as f32;
            let g = val.0[1] as f32 / max_val as f32;
            let b = val.0[2] as f32 / max_val as f32;
            Color4f::new(r, g, b, a)
        }
        ofd_base::file::res::Type::GRAY => {
            let y = val.0[0] as f32 / max_val as f32;
            Color4f::new(y, y, y, a)
        }
        ofd_base::file::res::Type::CMYK => {
            // cmyk to rgb
            let c = val.0[0] as f32 / max_val as f32;
            let m = val.0[1] as f32 / max_val as f32;
            let y = val.0[2] as f32 / max_val as f32;
            let k = val.0[2] as f32 / max_val as f32;

            let x = 1.0 - k;
            let r = (1.0 - c) * x;
            let g = (1.0 - m) * c;
            let b = (1.0 - y) * c;

            Color4f::new(r, g, b, a)
        }
    };
    Ok(r)
}

fn decide_size(
    page: &PageXmlFile,
    templates: &Vec<&PageXmlFile>,
    doc: &DocumentXmlFile,
) -> CtPageArea {
    let mut pa = page.area.as_ref();
    for tpl in templates {
        pa = pa.or(tpl.area.as_ref());
    }
    pa = pa.or(Some(&doc.common_data.page_area));
    let size = pa.unwrap();
    CtPageArea { ..*size }
}

/// draw a page
fn draw_page<I: Read + Seek>(ctx: &mut RenderCtx<I>, tpl: &PageXmlFile) -> Result<()> {
    let init_sc = ctx.canvas.save_count();
    if let Some(content) = tpl.content.as_ref() {
        for layer in &content.layer {
            let dp_id = layer.draw_param;
            let dp = get_draw_param_by_id(ctx.resources, dp_id);
            ctx.draw_param_stack.push(dp.clone());
            draw_layer(ctx, layer);
            ctx.draw_param_stack.pop(dp);
            if !ctx.draw_param_stack.is_empty() {
                warn!("draw_param stack is imbalance!");
            }
        }
    }
    let after_sc = ctx.canvas.save_count();
    assert_eq!(init_sc, after_sc);
    Ok(())
}

fn get_draw_param_by_id(resources: &Resources, id: Option<StRefId>) -> Option<DrawParam> {
    if let Some(dp_id) = id {
        let dp = resources.get_draw_param_by_id(dp_id);
        if dp.is_none() {
            warn!("required DrawParam id = {dp_id} is not defined!");
        }
        dp
    } else {
        None
    }
}

fn draw_layer<I: Read + Seek>(ctx: &mut RenderCtx<I>, layer: &ofd_base::file::page::Layer) {
    let resources = ctx.resources;
    if let Some(dp_id) = layer.draw_param {
        let dp = resources.get_draw_param_by_id(dp_id);
        ctx.draw_param_stack.push(dp.clone());
        if let Some(objects) = layer.objects.as_ref() {
            draw_object(ctx, objects);
        }
        ctx.draw_param_stack.pop(dp);
    } else if let Some(objects) = layer.objects.as_ref() {
        draw_object(ctx, objects);
    }
}

fn draw_object<I: Read + Seek>(ctx: &mut RenderCtx<I>, objects: &Vec<VtGraphicUnit>) {
    let canvas = ctx.canvas;
    let resources = ctx.resources;

    for obj in objects {
        let dp_id = obj.draw_param();
        let dp = get_draw_param_by_id(resources, dp_id);
        ctx.draw_param_stack.push(dp.clone());
        ctx.canvas.save();
        let init_sc = canvas.save_count();
        assert!(init_sc > 0);
        let r = match obj {
            VtGraphicUnit::TextObject(text) => draw_text_object(ctx, text),
            VtGraphicUnit::PathObject(path) => draw_path_object(ctx, path),
            VtGraphicUnit::ImageObject(image) => draw_image_object(ctx, image),
            VtGraphicUnit::CompositeObject(_co) => todo!(),
            VtGraphicUnit::PageBlock(_pb) => todo!(),
        };
        ctx.draw_param_stack.pop(dp);
        if r.is_err() {
            error!("draw_layer_error: {:?}", r);
        }
        let after_sc = canvas.save_count();
        assert_eq!(
            init_sc, after_sc,
            "imbalanced skia save count. obj: {:?}",
            obj
        );
        ctx.canvas.restore();
    }
}
// trait ToMatrix {
//     fn to_matrix(&self) -> Matrix;
// }
// fn to_matrix(ctm: &StArray<f32>) -> Matrix {
//     Matrix::new_all(
//         ctm.0[0], ctm.0[2], ctm.0[4], ctm.0[1], ctm.0[3], ctm.0[5], 0.0, 0.0, 1.0,
//     )
// }

// impl ToMatrix for

fn draw_image_object<I: Read + Seek>(
    ctx: &mut RenderCtx<I>,
    image_object: &ImageObject,
) -> Result<()> {
    if !image_object.visible.unwrap_or(true) {
        return Ok(());
    }
    ctx.canvas.save();
    apply_boundary(ctx.canvas, image_object.boundary);
    // apply_ctm(ctx.canvas, image_object.ctm.as_ref());
    if let Some((ofd_item, image)) = ctx.resources.get_image_by_id(image_object.resource_id) {
        match &image.format {
            Some(format) => match format.to_lowercase().as_str() {
                "png" => {
                    let p = ofd_item.resolve(&ofd_item.base_loc.join(&image.media_file));
                    // dbg!(&p);
                    let bytes = ctx._ofd.bytes(p)?;
                    let len = bytes.len();
                    let data =
                        Data::from_stream(Cursor::new(bytes), len).ok_or(eyre!("data in None"))?;
                    let img = Image::from_encoded(data).ok_or(eyre!("image encoded in None"))?;
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    // img.
                    // let mut mat = Matrix::new_identity();
                    let b = img.bounds().into();

                    let dst = Rect::from_wh(
                        // image_object.boundary.x,
                        // image_object.boundary.y,
                        image_object.boundary.w,
                        image_object.boundary.h,
                    );
                    // dbg!(&img);
                    // if let Some(ctm) = image_object.ctm.as_ref() {
                    //     let image_filter =
                    //         matrix_transform(&to_matrix(ctm), SamplingOptions::default(), None)
                    //             .unwrap();
                    //     paint.set_image_filter(image_filter);
                    // }
                    ctx.canvas.draw_image_rect(
                        img,
                        Some((&b, SrcRectConstraint::Strict)),
                        dst,
                        &paint,
                    );
                    // ctx.canvas.draw_image_with_sampling_options(
                    //     img,
                    //     (0,0),
                    //     // SamplingOptions::from()
                    //     image_filter.into(),
                    //     // Some(&img.bounds()),
                    //     // Rect::from_wh(image_object.boundary.w, image_object.boundary.h),
                    //     None
                    // );
                }
                _ => {
                    // unsupported format
                    warn!("unsupported image format {}", format);
                }
            },
            None => {
                // detect format
            }
        }
        // dbg!(mm);
    } else {
        warn!(
            "image resource not found! id = {}",
            image_object.resource_id
        );
    }
    // ctx.canvas.draw_image()
    // warn!("draw_image_object is not implemented!");
    ctx.canvas.restore();
    Ok(())
}

fn mm2px_i32(mm: f32, dpi: i32) -> i32 {
    let f = mm * dpi as f32 / 25.4;
    f.round() as i32
}

fn calc_scale(dpi: i32) -> f32 {
    dpi as f32 / 25.4
}

#[inline]
fn next_val<T: FromStr>(iter: &mut Enumerate<Iter<String>>) -> Result<T> {
    let (_, val) = iter.next().ok_or_eyre("unexpected end")?;
    let r = val.parse::<T>().map_err(|_| MyError::ParseError)?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm2px() {
        let x = mm2px_i32(210.0, 72);
        assert_eq!(x, 595);
        let y = mm2px_i32(297.0, 72);
        assert_eq!(y, 842);

        let x = mm2px_i32(210.0, 150);
        assert_eq!(x, 1240);
        let y = mm2px_i32(297.0, 150);
        assert_eq!(y, 1754);

        let x = mm2px_i32(210.0, 300);
        assert_eq!(x, 2480);
        let y = mm2px_i32(297.0, 300);
        assert_eq!(y, 3508);
    }
}
