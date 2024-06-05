use std::iter::Enumerate;
use std::slice::Iter;
use std::str::FromStr;

// use eyre::Ok;
use eyre::OptionExt;
use eyre::Result;
use skia_safe::path::ArcSize;
use skia_safe::Path;
use skia_safe::PathDirection;
use skia_safe::{Canvas, ImageInfo, Surface};

use crate::container::Container;
use crate::element::base::StArray;
use crate::element::file::page::PathObject;
use crate::error::MyError;

// fn render_template()

fn create_canvas(size: (i32, i32)) -> Result<Surface> {
    let ii = ImageInfo::new_s32(size, skia_safe::AlphaType::Opaque);
    let surface = skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("message")?;
    // let canvas = Canvas::f
    // let canvas = surface.canvas();
    Ok(surface)
}

fn draw_path_object(canvas: &mut Canvas, path_object: &PathObject) {
    let vis = path_object.visible.unwrap_or(true);
    if !vis {
        return;
    }

    // canvas.
    // create_canvas(size)
    todo!()
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
fn render_template(container: &mut Container, doc_index: usize, page_index: usize) -> Result<()> {
    let mut doc = container.document_by_index(doc_index)?;
    let page = doc.get_page(page_index)?;
    let content = page.content;
    let size = content.area;
    let mut has_template = false;
    if let Some(templates) = content.template {
        if templates.is_empty() {
            has_template = false;
        } else {
            has_template = true;
            let tpl = doc.get_template_by_id(5);
        }
        
    }

    let xml = container.template_by_index(doc_index, page_index)?.content;
    Ok(())
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
}
