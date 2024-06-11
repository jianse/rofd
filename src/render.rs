use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;

use eyre::eyre;
use eyre::OptionExt;
use eyre::Result;
// use skia_safe::named_transfer_fn::SRGB;
use skia_safe::path::ArcSize;
use skia_safe::Color;
use skia_safe::Color4f;
use skia_safe::Image;
use skia_safe::Matrix;
use skia_safe::Paint;
use skia_safe::Path;
use skia_safe::PathDirection;
use skia_safe::Rect;
use skia_safe::{Canvas, ImageInfo, Surface};

use crate::container::Container;
use crate::container::Resources;
use crate::element::base::StArray;
use crate::element::base::StBox;
use crate::element::common::CtColor;
use crate::element::file::document::CtPageArea;
use crate::element::file::document::DocumentXmlFile;
use crate::element::file::page::PageXmlFile;
use crate::element::file::page::PathObject;
use crate::element::file::res::DrawParam;
use crate::element::file::res::SRGB;
// use crate::element
use crate::error::MyError;

// fn render_template()

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
    // todo!()
}

fn resolve_color(ct_color: &CtColor, resources: &Resources) -> Result<()> {
    // ct_color
    let cs = if let Some(cs_id) = ct_color.color_space {
        // have a color space refence
        let cs = resources
            .get_color_space_by_id(cs_id)
            .ok_or_eyre("message")?;
        cs
    } else {
        if let Some(cs_id) = resources.default_cs {
            // looking for default color space
            let cs = resources
                .get_color_space_by_id(cs_id)
                .ok_or_eyre("message")?;
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
            "color and colorspace mismatch"
        );
        // let color = Color::from_rgb(val.0, g, b);

        Ok(())
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
    draw_param: Option<&DrawParam>,
) -> Result<()> {
    let vis = path_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    canvas.save();
    let boundary = path_object.boundary;
    apply_boundary(canvas, boundary);
    let path = abbreviated_data_2_path(&path_object.abbreviated_data)?;
    if path_object.stroke.unwrap_or(true) {
        let color: Color4f;
        if let Some(ct_color) = &path_object.stroke_color {
            if let Some(color_values) = &ct_color.value {}
            todo!();
        } else {
            color = Color::BLACK.into();
        }
        //  = Color::from_rgb(r, g, b);
        let mut paint = Paint::new(color, None);
        paint.set_stroke(true);
        let lw = path_object.line_width.unwrap_or(0.353);
        paint.set_stroke_width(lw);
        canvas.draw_path(&path, &paint);
    }

    canvas.restore();
    Ok(())
}

fn abbreviated_data_2_path(abbr: &StArray<String>) -> Result<Path> {
    let mut path = Path::new();
    let mut iter = abbr.0.iter().enumerate();
    while let Some((idx, ele)) = iter.next() {
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
            "" => {
                path.close();
                Ok(())
            }
            _ => { Err(MyError::UnknownPathCommnad(ele.into())) }?,
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
    if let Some(content) = tpl.content.as_ref() {
        for layer in &content.layer {
            // TODO: get draw_param
            let dp = if let Some(dp_id) = layer.draw_param {
                let dp = resources
                    .get_draw_param_by_id(dp_id)
                    .ok_or_eyre(format!("required DrawParam id = {dp_id} is not defined!"))?;
                Some(dp)
            } else {
                None
            };
            draw_layer(can, layer, resources, dp);
        }
    }
    Ok(())
}

fn draw_layer(
    canvas: &Canvas,
    layer: &crate::element::file::page::Layer,
    resources: &Resources,
    draw_param: Option<&DrawParam>,
) -> () {
    if let Some(objects) = layer.objects.as_ref() {
        for obj in objects {
            let _ = match obj {
                crate::element::file::page::CtPageBlock::TextObject(_text) => Ok(()),
                crate::element::file::page::CtPageBlock::PathObject(path) => {
                    draw_path_object(canvas, path, resources, draw_param)
                }
                crate::element::file::page::CtPageBlock::ImageObject {} => todo!(),
                crate::element::file::page::CtPageBlock::CompositeObject {} => todo!(),
                crate::element::file::page::CtPageBlock::PageBlock {} => todo!(),
            };
        }
    }
    // todo!()
}

fn mm2px_i32(mm: f32, dpi: i32) -> i32 {
    let f = mm * dpi as f32 / 25.4;
    let f = dbg!(f);
    f.round() as i32
}

fn calc_scale(dpi: i32) -> f32 {
    dpi as f32 / 25.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() -> Result<()> {
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
