use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;

use eyre::eyre;
use eyre::OptionExt;
use eyre::Result;
use skia_safe::path::ArcSize;
use skia_safe::BlendMode;
use skia_safe::ClipOp;
use skia_safe::Color;
use skia_safe::Color4f;
use skia_safe::Font;
use skia_safe::FontMgr;
use skia_safe::FontStyle;
use skia_safe::Image;
use skia_safe::Matrix;
use skia_safe::Paint;
use skia_safe::PaintCap;
use skia_safe::PaintJoin;
use skia_safe::Path;
use skia_safe::PathDirection;
use skia_safe::Point;
use skia_safe::Rect;
use skia_safe::TextBlob;
use skia_safe::{Canvas, ImageInfo, Surface};

use crate::container::Container;
use crate::container::Resources;
use crate::element::base::StArray;
use crate::element::base::StBox;
use crate::element::base::StRefId;
use crate::element::common::Cap;
use crate::element::common::CtColor;
use crate::element::common::Join;
use crate::element::file::document::CtPageArea;
use crate::element::file::document::DocumentXmlFile;
use crate::element::file::page::PageXmlFile;
use crate::element::file::page::PathObject;
use crate::element::file::page::TextObject;
use crate::element::file::res::DrawParam;
use crate::element::file::res::SRGB;
// use crate::element
use crate::error::MyError;

// fn render_template()

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
        if element_stroke_color.is_some() {
            return resolve_color(element_stroke_color.unwrap(), resources).unwrap();
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
        return fallback;
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
        let join = match rj {
            Join::Miter => PaintJoin::Miter,
            Join::Round => PaintJoin::Round,
            Join::Bevel => PaintJoin::Bevel,
        };
        return join;
    }

    fn get_miter_limit(&self, element_miter_limit: Option<f32>, fallback: f32) -> f32 {
        // test element miter limit
        let rml = if let Some(ml) = element_miter_limit {
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
        };
        return rml;
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
        let cap = match rc {
            Cap::Butt => PaintCap::Butt,
            Cap::Round => PaintCap::Round,
            Cap::Square => PaintCap::Square,
        };
        cap
    }

    fn get_fill_color(
        &self,
        element_fill_color: Option<&CtColor>,
        resources: &Resources,
        fallback: Color4f,
    ) -> Color4f {
        // test element color
        if element_fill_color.is_some() {
            return resolve_color(element_fill_color.unwrap(), resources).unwrap();
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
        return fallback;
    }
}

fn create_surface(size: (i32, i32)) -> Result<Surface> {
    let ii = ImageInfo::new_s32(size, skia_safe::AlphaType::Unpremul);
    let surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
    // let canvas = Canvas::f
    // let canvas = surface.canvas();
    Ok(surface)
}

fn apply_boundary(can: &Canvas, boundary: StBox) {
    let br = Rect::from_xywh(boundary.x, boundary.y, boundary.w, boundary.h);
    let matrix = Matrix::translate(boundary.get_tl());
    can.clip_rect(br, None, true);
    can.concat(&matrix);
}

fn resolve_color(ct_color: &CtColor, resources: &Resources) -> Result<Color4f> {
    // ct_color
    let cs = if let Some(cs_id) = ct_color.color_space {
        // have a color space reference
        let cs = resources
            .get_color_space_by_id(cs_id)
            .ok_or_eyre(format!("color space not found id: {cs_id}"))?;
        cs
    } else {
        if let Some(cs_id) = resources.default_cs {
            // looking for default color space
            let cs = resources
                .get_color_space_by_id(cs_id)
                .ok_or_eyre(format!("default cs not found id {cs_id}"))?;
            cs
        } else {
            // default srgb
            &SRGB
        }
    };
    if let Some(val) = &ct_color.value {
        // value color
        assert_eq!(
            cs.r#type.channel_count(),
            val.0.len(),
            "color and color space mismatch"
        );
        let bpc = cs.bits_per_component.unwrap_or(8);
        let max_val = (1 << bpc) - 1;
        let a = (ct_color.alpha.unwrap_or(255) / 255) as f32;
        let r = match cs.r#type {
            crate::element::file::res::Type::RGB => {
                let r = val.0[0] as f32 / max_val as f32;
                let g = val.0[1] as f32 / max_val as f32;
                let b = val.0[2] as f32 / max_val as f32;
                Color4f::new(r, g, b, a)
            }
            crate::element::file::res::Type::GRAY => {
                let y = val.0[0] as f32 / max_val as f32;
                Color4f::new(y, y, y, a)
            }
            crate::element::file::res::Type::CMYK => {
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
    } else {
        // plate color
        todo!()
    }
    // ct_color.
}

fn draw_path_object(
    canvas: &Canvas,
    path_object: &PathObject,
    resources: &Resources,
    draw_param_stack: &DrawParamStack,
) -> Result<()> {
    let vis = path_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    canvas.save();
    let boundary = path_object.boundary;
    apply_boundary(canvas, boundary);
    let mut path = abbreviated_data_2_path(&path_object.abbreviated_data)?;

    // draw stroke
    if path_object.stroke.unwrap_or(true) {
        let color: Color4f = draw_param_stack.get_stroke_color(
            path_object.stroke_color.as_ref(),
            resources,
            Color::BLACK.into(),
        );
        let mut paint = Paint::new(color, None);

        paint.set_stroke(true);

        let join = draw_param_stack.get_join(path_object.join.as_ref(), &Join::Miter);
        paint.set_stroke_join(join);

        if join == PaintJoin::Miter {
            let miter = draw_param_stack.get_miter_limit(path_object.miter_limit, 3.528);
            paint.set_stroke_miter(miter);
        }

        let cap = draw_param_stack.get_cap(path_object.cap.as_ref(), &Cap::Butt);
        paint.set_stroke_cap(cap);
        let lw = path_object.line_width.unwrap_or(0.353);
        paint.set_stroke_width(lw);
        canvas.draw_path(&path, &paint);
    }

    // fill
    if path_object.fill.unwrap_or(false) {
        let color: Color4f = draw_param_stack.get_fill_color(
            path_object.fill_color.as_ref(),
            resources,
            Color::TRANSPARENT.into(),
        );
        let mut paint = Paint::new(color, None);
        paint.set_stroke(false);
        let rule = path_object
            .rule
            .as_ref()
            .unwrap_or(&crate::element::file::page::FillRule::NoneZero);
        let ft = match rule {
            crate::element::file::page::FillRule::NoneZero => skia_safe::PathFillType::Winding,
            crate::element::file::page::FillRule::EvenOdd => skia_safe::PathFillType::EvenOdd,
        };
        path.set_fill_type(ft);
        // paint.set_
        canvas.draw_path(&path, &paint);
    }

    canvas.restore();
    Ok(())
}

fn abbreviated_data_2_path(abbr: &StArray<String>) -> Result<Path> {
    let mut path = Path::new();
    let mut iter = abbr.0.iter().enumerate();

    // TODO: make error contains _idx
    while let Some((_idx, ele)) = iter.next() {
        let s: Result<()> = match ele as &str {
            "S" => Ok(()),
            "M" => {
                let x = next_val::<f32>(&mut iter)?;
                let y = next_val::<f32>(&mut iter)?;
                path.move_to((x, y));
                Ok(())
            }
            "L" => {
                let x = next_val::<f32>(&mut iter)?;
                let y = next_val::<f32>(&mut iter)?;
                path.line_to((x, y));
                Ok(())
            }
            "Q" => {
                let x1 = next_val::<f32>(&mut iter)?;
                let y1 = next_val::<f32>(&mut iter)?;
                let x2 = next_val::<f32>(&mut iter)?;
                let y2 = next_val::<f32>(&mut iter)?;
                path.quad_to((x1, y1), (x2, y2));
                Ok(())
            }
            "B" => {
                let x1 = next_val::<f32>(&mut iter)?;
                let y1 = next_val::<f32>(&mut iter)?;
                let x2 = next_val::<f32>(&mut iter)?;
                let y2 = next_val::<f32>(&mut iter)?;
                let x3 = next_val::<f32>(&mut iter)?;
                let y3 = next_val::<f32>(&mut iter)?;
                path.cubic_to((x1, y1), (x2, y2), (x3, y3));
                Ok(())
            }
            "A" => {
                let rx = next_val::<f32>(&mut iter)?;
                let ry = next_val::<f32>(&mut iter)?;
                let x_axis_rotate = next_val::<f32>(&mut iter)?;
                let large_arc = next_arc_size(&mut iter)?;
                let sweep = next_path_direction(&mut iter)?;

                let end_x = next_val::<f32>(&mut iter)?;
                let end_y = next_val::<f32>(&mut iter)?;
                path.arc_to_rotated((rx, ry), x_axis_rotate, large_arc, sweep, (end_x, end_y));
                Ok(())
            }
            "C" => {
                path.close();
                Ok(())
            }
            _ => { Err(MyError::UnknownPathCommand(ele.into())) }?,
        };
        s?;
    }
    Ok(path)
}

#[inline]
fn next_val<T: FromStr>(iter: &mut Enumerate<Iter<String>>) -> Result<T> {
    let (_, val) = iter.next().ok_or_eyre("unexpected end")?;
    let r = val.parse::<T>().map_err(|_| MyError::ParseError)?;
    Ok(r)
}

fn next_arc_size(iter: &mut Enumerate<Iter<String>>) -> Result<ArcSize> {
    let val = next_val::<u8>(iter)?;
    let r = match val {
        1 => Ok(ArcSize::Large),
        0 => Ok(ArcSize::Small),
        _ => Err(MyError::Invalid),
    }?;
    Ok(r)
}

fn next_path_direction(iter: &mut Enumerate<Iter<String>>) -> Result<PathDirection> {
    let val = next_val::<u8>(iter)?;
    let r = match val {
        1 => Ok(PathDirection::CW),
        0 => Ok(PathDirection::CCW),
        _ => Err(MyError::Invalid),
    }?;
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
    // todo!()
}
pub fn render_template(
    container: &mut Container,
    doc_index: usize,
    page_index: usize,
) -> Result<Image> {
    let dpi = 300;

    let doc = container.document_by_index(doc_index)?;
    let doc_xml = &doc.content;

    let page = container.page_by_index(doc_index, page_index)?;
    let page_xml = &page.content;

    let templates = container.templates_for_page(doc_index, page_index)?;
    let tpls = &templates
        .iter()
        .map(|i| &i.content)
        .collect::<Vec<&PageXmlFile>>();

    if templates.is_empty() {
        return Err(eyre!("no such template!"));
    }
    let pa = decide_size(page_xml, tpls, doc_xml);

    let size = (
        mm2px_i32(pa.physical_box.w, dpi),
        mm2px_i32(pa.physical_box.h, dpi),
    );
    let resources = container.resources_for_page(doc_index, page_index)?;

    let mut sur = create_surface(size)?;
    let can = sur.canvas();
    can.save();
    can.clip_rect(Rect::from_size(size), ClipOp::Intersect, true);
    can.draw_color(Color::WHITE, BlendMode::Color);
    can.restore();
    let scale = calc_scale(dpi);
    can.scale((scale, scale));
    for tpl in tpls {
        draw_page(can, tpl, &resources)?;
    }
    can.restore();
    let snap = sur.image_snapshot();
    Ok(snap)
}

fn draw_page(can: &Canvas, tpl: &&PageXmlFile, resources: &Resources) -> Result<()> {
    let mut draw_param_stack = DrawParamStack::new();
    if let Some(content) = tpl.content.as_ref() {
        for layer in &content.layer {
            let dp_id = layer.draw_param;
            let dp = get_draw_param_by_id(resources, dp_id);
            draw_param_stack.push(dp.clone());
            draw_layer(can, layer, resources, &mut draw_param_stack);
            draw_param_stack.pop(dp);
            if !draw_param_stack.is_empty() {
                println!("warn! draw_param stack is imbalance!");
            }
        }
    }
    Ok(())
}

fn get_draw_param_by_id(resources: &Resources, id: Option<StRefId>) -> Option<DrawParam> {
    if let Some(dp_id) = id {
        let dp = resources.get_draw_param_by_id(dp_id);
        if dp.is_none() {
            println!("warn! required DrawParam id = {dp_id} is not defined!");
        }
        dp
    } else {
        None
    }
}

fn draw_layer(
    canvas: &Canvas,
    layer: &crate::element::file::page::Layer,
    resources: &Resources,
    draw_param_stack: &mut DrawParamStack,
) -> () {
    if let Some(objects) = layer.objects.as_ref() {
        for obj in objects {
            let _ = match obj {
                crate::element::file::page::CtPageBlock::TextObject(text) => {
                    let dp_id = text.draw_param;
                    let dp = get_draw_param_by_id(resources, dp_id);
                    draw_param_stack.push(dp.clone());
                    let _ = draw_text_object(canvas, text, resources, draw_param_stack);
                    draw_param_stack.pop(dp);
                }
                crate::element::file::page::CtPageBlock::PathObject(path) => {
                    let dp_id = path.draw_param;
                    let dp = get_draw_param_by_id(resources, dp_id);
                    draw_param_stack.push(dp.clone());
                    let _ = draw_path_object(canvas, path, resources, draw_param_stack);
                    draw_param_stack.pop(dp);
                }
                crate::element::file::page::CtPageBlock::ImageObject {} => todo!(),
                crate::element::file::page::CtPageBlock::CompositeObject {} => todo!(),
                crate::element::file::page::CtPageBlock::PageBlock {} => todo!(),
            };
        }
    }
    // todo!()
}

fn draw_text_object(
    canvas: &Canvas,
    text_object: &TextObject,
    resources: &Resources,
    draw_param: &DrawParamStack,
) -> Result<()> {
    let vis = text_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    canvas.save();
    let boundary = text_object.boundary;
    apply_boundary(canvas, boundary);
    let color: Color4f = Color::RED.into();
    let paint = Paint::new(&color, None);
    // paint
    let text_codes = &text_object.text_codes;
    assert!(text_codes.len() > 0, "textCode length must grater than 0!");
    let mut last_pos = (text_codes[0].x.unwrap(), text_codes[0].y.unwrap());
    let fm = FontMgr::new();
    let typeface = fm
        .match_family_style("楷体", FontStyle::normal())
        .ok_or_eyre("no font found!")?;

    let mut font = Font::from_typeface(typeface, Some(text_object.size));
    // font.set_w

    // get
    // font.set_scale_x(0.5);
    // font.set
    for text_code in &text_object.text_codes {
        let origin = (
            text_code.x.unwrap_or(last_pos.0),
            text_code.y.unwrap_or(last_pos.1),
        );
        let blob = from_text_code(text_code, &font)?;

        canvas.draw_text_blob(blob, origin, &paint);
        last_pos = origin;
    }
    // canvas.draw_text_align(text, p, font, paint, align);

    canvas.restore();
    Ok(())
}

/// make TextBlob from TextCode
fn from_text_code(
    // origin: (f32, f32),
    text_code: &crate::element::file::page::TextCode,
    font: &Font,
) -> Result<TextBlob> {
    let origin = (0.0, 0.0);
    let text = &text_code.val;
    let pos = decode_dx_dy(
        origin.into(),
        text_code.delta_x.as_ref(),
        text_code.delta_y.as_ref(),
        text.chars().count(),
    )?;
    let tb = TextBlob::from_pos_text(text, &pos, font).ok_or_eyre("message");
    return tb;
}

/// decode dx dy into points
fn decode_dx_dy(
    origin: (f32, f32),
    delta_x: Option<&StArray<String>>,
    delta_y: Option<&StArray<String>>,
    len: usize,
) -> Result<Vec<Point>> {
    let mut res = vec![];
    let dxs = delta_x
        .map(flat_g)
        .transpose()?
        .unwrap_or(vec![0_f32; len - 1]);
    let dys = delta_y
        .map(flat_g)
        .transpose()?
        .unwrap_or(vec![0_f32; len - 1]);
    assert_eq!(dxs.len(), len - 1, "dxs: {:?},dys:{:?}", delta_x, delta_y);
    // if let Some(dx)
    assert_eq!(dys.len(), len - 1, "dxs: {:?},dys:{:?}", delta_x, delta_y);
    let mut last_pos = origin;
    res.push(Point::new(last_pos.0, last_pos.1));
    for (dx, dy) in std::iter::zip(dxs, dys) {
        last_pos = (last_pos.0 + dx, last_pos.1 + dy);
        res.push(Point::new(last_pos.0, last_pos.1));
    }
    // todo!();
    assert_eq!(res.len(), len);
    return Ok(res);
}

/// flat sparse format (include g command) dx or dy into dense format (only numbers)
fn flat_g(d: &StArray<String>) -> Result<Vec<f32>> {
    let mut res = vec![];
    let mut iter = d.0.iter().enumerate();

    while let Some((_idx, ele)) = iter.next() {
        match ele as &str {
            "g" => {
                let rep = next_val::<usize>(&mut iter)?;
                let val = next_val::<f32>(&mut iter)?;
                res.append(&mut vec![val; rep]);
            }
            _ => {
                let val = f32::from_str(ele)?;
                res.push(val);
            }
        }
    }
    Ok(res)
}

fn mm2px_i32(mm: f32, dpi: i32) -> i32 {
    let f = mm * dpi as f32 / 25.4;
    f.round() as i32
}

fn calc_scale(dpi: i32) -> f32 {
    dpi as f32 / 25.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbr_2_path() -> Result<()> {
        let src = StArray::<String>::from_str("M 10.07 5.54 B 10.07 3.04 8.04 1 5.53 1 B 3.03 1 1 3.04 1 5.54 B 1 8.04 3.03 10.08 5.53 10.08 B 8.04 10.08 10.07 8.04 10.07 5.54 M 2.3 2.3 L 8.7 8.7 M 2.3 8.7 L 8.7 2.3 ")?;
        let p = abbreviated_data_2_path(&src)?;
        dbg!(p);
        Ok(())
    }

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
