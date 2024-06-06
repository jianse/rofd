use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;

use eyre::eyre;
use eyre::OptionExt;
use eyre::Result;
use skia_safe::path::ArcSize;
use skia_safe::Color;
use skia_safe::Color4f;
use skia_safe::Paint;
use skia_safe::Path;
use skia_safe::PathDirection;
use skia_safe::{Canvas, ImageInfo, Surface};

use crate::container::Container;
use crate::element::base::StArray;
use crate::element::file::document::CtPageArea;
use crate::element::file::document::DocumentXmlFile;
use crate::element::file::page::PageXmlFile;
use crate::element::file::page::PathObject;
use crate::error::MyError;

// fn render_template()

fn create_surface(size: (i32, i32)) -> Result<Surface> {
    let ii = ImageInfo::new_s32(size, skia_safe::AlphaType::Opaque);
    let surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
    // let canvas = Canvas::f
    // let canvas = surface.canvas();
    Ok(surface)
}

fn draw_path_object(canvas: &Canvas, path_object: &PathObject) -> Result<()> {
    let vis = path_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    let path = abbreviated_data_2_path(&path_object.abbreviated_data)?;
    let color: Color4f = Color::from_rgb(255, 0, 0).into();

    let paint = Paint::new(color, None);
    canvas.draw_path(&path, &paint);
    // canvas.
    // create_canvas(size)
    // todo!()
    Ok(())
}

fn abbreviated_data_2_path(abbr: &StArray<String>) -> Result<Path> {
    let mut path = Path::new();
    let mut iter = abbr.0.iter().enumerate();
    while let Some((idx, ele)) = iter.next() {
        let s: std::prelude::v1::Result<(), MyError> = match ele as &str {
            "S" => Ok(()),
            "M" => {
                // iter.nex
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
pub fn render_template(container: &mut Container, doc_index: usize, page_index: usize) -> Result<()> {
    let doc = container.document_by_index(doc_index)?;
    let doc_xml = &doc.content;

    let page = container.page_by_index(doc_index, page_index)?;
    let page_xml = &page.content;

    // let
    // let size = content.area.as_ref();
    let templates = container.templates_for_page(doc_index, page_index)?;
    let tpls = &templates
        .iter()
        .map(|i| &i.content)
        .collect::<Vec<&PageXmlFile>>();
    // let has_template = !;
    if templates.is_empty() {
        return Err(eyre!("no such template!"));
    }
    let pa = decide_size(page_xml, tpls, doc_xml);
    // create_canvas(size.physical_box);
    let size = (mm2px(pa.physical_box.w, 300), mm2px(pa.physical_box.h, 300));
    // let template_size =
    let mut sur = create_surface(size)?;
    let can = sur.canvas();
    for tpl in tpls {
        draw_page(can, tpl);
    }

    Ok(())
}

fn draw_page(can: &Canvas, tpl: &&PageXmlFile) -> () {
    if let Some(content) = tpl.content.as_ref() {
        for layer in &content.layer {
            draw_layer(can, layer);
        }
    }
    todo!()
}

fn draw_layer(can: &Canvas, layer: &crate::element::file::page::Layer) -> () {
    if let Some(objects) = layer.objects.as_ref() {
        for obj in objects {
            let _ = match obj {
                crate::element::file::page::CtPageBlock::TextObject(text) => todo!(),
                crate::element::file::page::CtPageBlock::PathObject(path) => {
                    draw_path_object(can, path)
                }
                crate::element::file::page::CtPageBlock::ImageObject {} => todo!(),
                crate::element::file::page::CtPageBlock::CompositeObject {} => todo!(),
                crate::element::file::page::CtPageBlock::PageBlock {} => todo!(),
            };
        }
    }
    todo!()
}
fn mm2px(mm: f32, dpi: i32) -> i32 {
    let f = mm * dpi as f32 / 25.4;
    let f = dbg!(f);
    f.round() as i32
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
        let x = mm2px(210.0, 72);
        assert_eq!(x, 595);
        let y = mm2px(297.0, 72);
        assert_eq!(y, 842);

        let x = mm2px(210.0, 150);
        assert_eq!(x, 1240);
        let y = mm2px(297.0, 150);
        assert_eq!(y, 1754);

        let x = mm2px(210.0, 300);
        assert_eq!(x, 2480);
        let y = mm2px(297.0, 300);
        assert_eq!(y, 3508);
    }
}
