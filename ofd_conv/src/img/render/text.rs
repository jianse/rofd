use crate::img::render::{apply_boundary, apply_ctm, next_val, RenderCtx};
use eyre::OptionExt;
use ofd_base::file::page::TextObject;
use ofd_base::StArray;
use skia_safe::{Color, Font, FontStyle, Paint, Point, TextBlob};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, warn};

pub(super) fn draw_text_object(ctx: &mut RenderCtx, text_object: &TextObject) -> eyre::Result<()> {
    let canvas = ctx.canvas;
    let resources = ctx.resources;
    let draw_param_stack = &ctx.draw_param_stack;
    let vis = text_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    canvas.save();
    let boundary = text_object.boundary;
    apply_boundary(canvas, boundary);

    // debug!("??: {:?}", text_object.f);
    debug!("boundary: {:?}", text_object.boundary);
    debug!("bounds: {:?}", canvas.local_clip_bounds());
    // debug
    // canvas.draw_color(Color::from_argb(0xcc, 0, 0xcc, 0), None);

    let ctm = text_object.ctm.as_ref();
    apply_ctm(canvas, ctm);

    let text_vals = &text_object.text_vals;
    assert!(!text_vals.is_empty(), "text_vals must not be empty!");
    let tc0 = &text_vals[0].text_code;
    let mut last_pos = (tc0.x.unwrap(), tc0.y.unwrap());
    let font_id = text_object.font;
    let file_and_font = resources.get_font_by_id(font_id);
    // let typeface = ctx.font_mgr.typeface_by_resource_id(font_id);
    let typeface = if let Some((file, font)) = file_and_font {
        if let Some(font_file) = font.font_file.as_ref() {
            debug!("embedded font file: {}", font_file.display());
            let p = file.resolve(&PathBuf::from(&file.content.base_loc).join(font_file));
            warn!("embedded font file: {}", p);
            // todo!()
            ctx.font_mgr.load_embed_font(p)?
        } else {
            ctx.font_mgr
                .match_family_style(&font.font_name, FontStyle::normal())
                .unwrap_or_else(|| {
                    warn!("font {} not found! using fallback font.", &font.font_name);
                    ctx.font_mgr.fallback_typeface()
                })
        }
    } else {
        warn!("required font id = {font_id} is not defined!");
        // fallback font
        ctx.font_mgr.fallback_typeface()
    };
    debug!("font: {}", typeface.family_name());

    let font = Font::from_typeface(typeface, Some(text_object.size));
    for text_val in &text_object.text_vals {
        let text_code = &text_val.text_code;
        if let Some(cgf) = &text_val.cg_transform {
            warn!("text transform not implemented. {:?}", cgf);
        }
        if text_code.val.is_empty() {
            warn!("skipped an empty text code!");
            continue;
        }
        let origin = (
            text_code.x.unwrap_or(last_pos.0),
            text_code.y.unwrap_or(last_pos.1),
        );
        let blob = from_text_code(text_code, &font)?;

        if text_object.stroke.unwrap_or(false) {
            let stroke_color = draw_param_stack.get_stroke_color(
                text_object.stroke_color.as_ref(),
                resources,
                Color::TRANSPARENT.into(),
            );
            let mut paint = Paint::new(stroke_color, None);
            paint.set_stroke(true);
            canvas.draw_text_blob(blob.clone(), origin, &paint);
        }

        if text_object.fill.unwrap_or(true) {
            let fill_color = draw_param_stack.get_fill_color(
                text_object.fill_color.as_ref(),
                resources,
                Color::BLACK.into(),
            );
            let mut paint = Paint::new(fill_color, None);
            paint.set_stroke(false);
            canvas.draw_text_blob(blob, origin, &paint);
        }
        last_pos = origin;
    }
    // canvas.draw_text_align(text, p, font, paint, align);

    canvas.restore();
    Ok(())
}

/// make TextBlob from TextCode
fn from_text_code(
    // origin: (f32, f32),
    text_code: &ofd_base::file::page::TextCode,
    font: &Font,
) -> eyre::Result<TextBlob> {
    let origin = (0.0, 0.0);
    let text = &text_code.val;

    let pos = decode_dx_dy(
        origin,
        text_code.delta_x.as_ref(),
        text_code.delta_y.as_ref(),
        text.chars().count(),
    )?;
    TextBlob::from_pos_text(text, &pos, font).ok_or_eyre("message")
}

/// decode dx dy into points
fn decode_dx_dy(
    origin: (f32, f32),
    delta_x: Option<&StArray<String>>,
    delta_y: Option<&StArray<String>>,
    len: usize,
) -> eyre::Result<Vec<Point>> {
    let mut res = vec![];
    let mut dxs = delta_x
        .map(flat_g)
        .transpose()?
        .unwrap_or(vec![0_f32; len - 1]);
    let mut dys = delta_y
        .map(flat_g)
        .transpose()?
        .unwrap_or(vec![0_f32; len - 1]);
    assert!(
        dxs.len() >= (len - 1),
        "dx for textCode is not enough! required: {}, got: {}, dxs: {:?}",
        len - 1,
        dxs.len(),
        dxs
    );
    if dxs.len() > len - 1 {
        debug!("dx for textCode is longer than text! truncating!");
        dxs.truncate(len - 1);
    }
    assert!(
        dys.len() >= (len - 1),
        "dy for textCode is not enough! required: {}, got: {}, dys: {:?}",
        len - 1,
        dys.len(),
        dys
    );
    if dys.len() > len - 1 {
        debug!("dy for textCode is longer than text! truncating!");
        dys.truncate(len - 1);
    }
    let mut last_pos = origin;
    res.push(Point::new(last_pos.0, last_pos.1));
    for (dx, dy) in std::iter::zip(dxs, dys) {
        last_pos = (last_pos.0 + dx, last_pos.1 + dy);
        res.push(Point::new(last_pos.0, last_pos.1));
    }
    // todo!();
    assert_eq!(res.len(), len);
    Ok(res)
}

/// flat sparse format (include g command) dx or dy into dense format (only numbers)
fn flat_g(d: &StArray<String>) -> eyre::Result<Vec<f32>> {
    debug!("flatting {}", d);
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
