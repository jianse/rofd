use crate::error::MyError;
use crate::img::render::{apply_boundary, apply_ctm, next_val, RenderCtx};
use ofd_base::common::{Cap, Join};
use ofd_base::file::page::PathObject;
use ofd_base::StArray;
use skia_safe::path::ArcSize;
use skia_safe::{Color, Color4f, Paint, PaintJoin, Path, PathDirection};
use std::iter::Enumerate;
use std::slice::Iter;

fn next_arc_size(iter: &mut Enumerate<Iter<String>>) -> eyre::Result<ArcSize> {
    let val = next_val::<u8>(iter)?;
    let r = match val {
        1 => Ok(ArcSize::Large),
        0 => Ok(ArcSize::Small),
        _ => Err(MyError::Invalid),
    }?;
    Ok(r)
}

fn next_path_direction(iter: &mut Enumerate<Iter<String>>) -> eyre::Result<PathDirection> {
    let val = next_val::<u8>(iter)?;
    let r = match val {
        1 => Ok(PathDirection::CW),
        0 => Ok(PathDirection::CCW),
        _ => Err(MyError::Invalid),
    }?;
    Ok(r)
}
pub(super) fn draw_path_object<I>(
    ctx: &mut RenderCtx<I>,
    path_object: &PathObject,
) -> eyre::Result<()> {
    let vis = path_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    ctx.canvas.save();
    let boundary = path_object.boundary;
    apply_boundary(ctx.canvas, boundary);

    let ctm = path_object.ctm.as_ref();
    apply_ctm(ctx.canvas, ctm);

    let mut path = abbreviated_data_2_path(&path_object.abbreviated_data)?;

    // draw stroke
    if path_object.stroke.unwrap_or(true) {
        let color: Color4f = ctx.draw_param_stack.get_stroke_color(
            path_object.stroke_color.as_ref(),
            ctx.resources,
            Color::BLACK.into(),
        );
        let mut paint = Paint::new(color, None);

        paint.set_stroke(true);

        let join = ctx
            .draw_param_stack
            .get_join(path_object.join.as_ref(), &Join::Miter);
        paint.set_stroke_join(join);

        if join == PaintJoin::Miter {
            let miter = ctx
                .draw_param_stack
                .get_miter_limit(path_object.miter_limit, 3.528);
            paint.set_stroke_miter(miter);
        }

        let cap = ctx
            .draw_param_stack
            .get_cap(path_object.cap.as_ref(), &Cap::Butt);
        paint.set_stroke_cap(cap);
        let lw = path_object.line_width.unwrap_or(0.353);
        paint.set_stroke_width(lw);
        if let Some(alpha) = path_object.alpha {
            paint.set_alpha(alpha);
        }
        ctx.canvas.draw_path(&path, &paint);
    }

    // fill
    if path_object.fill.unwrap_or(false) {
        let color: Color4f = ctx.draw_param_stack.get_fill_color(
            path_object.fill_color.as_ref(),
            ctx.resources,
            Color::TRANSPARENT.into(),
        );
        let mut paint = Paint::new(color, None);
        paint.set_stroke(false);
        let rule = path_object
            .rule
            .as_ref()
            .unwrap_or(&ofd_base::file::page::FillRule::NoneZero);
        let ft = match rule {
            ofd_base::file::page::FillRule::NoneZero => skia_safe::PathFillType::Winding,
            ofd_base::file::page::FillRule::EvenOdd => skia_safe::PathFillType::EvenOdd,
        };
        path.set_fill_type(ft);
        if let Some(alpha) = path_object.alpha {
            paint.set_alpha(alpha);
        }
        ctx.canvas.draw_path(&path, &paint);
    }

    ctx.canvas.restore();
    Ok(())
}

fn abbreviated_data_2_path(abbr: &StArray<String>) -> eyre::Result<Path> {
    let mut path = Path::new();
    let mut iter = abbr.iter().enumerate();

    while let Some((idx, ele)) = iter.next() {
        let s: eyre::Result<()> = match ele as &str {
            "S" | "M" => {
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
            _ => { Err(MyError::UnknownPathCommand(ele.into(), idx)) }?,
        };
        s?;
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use std::str::FromStr;
    #[test]
    fn test_abbr_2_path() -> Result<()> {
        let src = StArray::<String>::from_str("M 10.07 5.54 B 10.07 3.04 8.04 1 5.53 1 B 3.03 1 1 3.04 1 5.54 B 1 8.04 3.03 10.08 5.53 10.08 B 8.04 10.08 10.07 8.04 10.07 5.54 M 2.3 2.3 L 8.7 8.7 M 2.3 8.7 L 8.7 2.3 ")?;
        let p = abbreviated_data_2_path(&src)?;
        dbg!(p);
        Ok(())
    }
}
