use crate::img::render::{apply_boundary, apply_ctm, next_val, RenderCtx};
use eyre::OptionExt;
use ofd_base::file::page::TextObject;
use ofd_base::StArray;
use ofd_rw::Resources;
use skia_safe::{Color, Font, FontStyle, Paint, Point, TextBlob, TextBlobBuilder};
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Index;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, warn};

pub(super) fn draw_text_object(ctx: &mut RenderCtx, text_object: &TextObject) -> eyre::Result<()> {
    let canvas = ctx.canvas;
    let resources = ctx.resources;
    let vis = text_object.visible.unwrap_or(true);
    if !vis {
        return Ok(());
    }
    canvas.save();
    let boundary = text_object.boundary;
    apply_boundary(canvas, boundary);

    debug!("boundary: {:?}", text_object.boundary);
    debug!("bounds: {:?}", canvas.local_clip_bounds());

    let ctm = text_object.ctm.as_ref();
    apply_ctm(canvas, ctm);

    let text_vals = &text_object.text_vals;
    assert!(!text_vals.is_empty(), "text_vals must not be empty!");
    let tc0 = &text_vals[0].text_code;
    let mut last_pos = (
        tc0.x.expect("x in first TextCode must be set"),
        tc0.y.expect("y in first TextCode must be set"),
    );

    let font = get_font(ctx, text_object, resources)?;
    for text_val in &text_object.text_vals {
        let text_code = &text_val.text_code;
        if text_code.val.is_empty() {
            warn!("skipped an empty text code!");
            continue;
        }

        let origin = (
            text_code.x.unwrap_or(last_pos.0),
            text_code.y.unwrap_or(last_pos.1),
        );
        let blob = from_text_val(text_val, &font)?;
        debug!("blob: {:?}", blob);

        if text_object.stroke.unwrap_or(false) {
            let stroke_color = ctx.draw_param_stack.get_stroke_color(
                text_object.stroke_color.as_ref(),
                resources,
                Color::TRANSPARENT.into(),
            );
            let mut paint = Paint::new(stroke_color, None);
            paint.set_stroke(true);
            if let Some(alpha) = text_object.alpha {
                paint.set_alpha(alpha);
            }
            canvas.draw_text_blob(&blob, origin, &paint);
        }

        if text_object.fill.unwrap_or(true) {
            let fill_color = ctx.draw_param_stack.get_fill_color(
                text_object.fill_color.as_ref(),
                resources,
                Color::BLACK.into(),
            );
            let mut paint = Paint::new(fill_color, None);
            paint.set_stroke(false);
            if let Some(alpha) = text_object.alpha {
                paint.set_alpha(alpha);
            }
            canvas.draw_text_blob(&blob, origin, &paint);
        }
        last_pos = origin;
    }

    canvas.restore();
    Ok(())
}

fn get_font(
    ctx: &mut RenderCtx,
    text_object: &TextObject,
    resources: &Resources,
) -> eyre::Result<Font> {
    let font_id = text_object.font;
    let file_and_font = resources.get_font_by_id(font_id);
    // let typeface = ctx.font_mgr.typeface_by_resource_id(font_id);
    let typeface = if let Some((file, font)) = file_and_font {
        if let Some(font_file) = font.font_file.as_ref() {
            debug!("embedded font file: {}", font_file.display());
            let p = file.resolve(&PathBuf::from(&file.content.base_loc).join(font_file));
            debug!("embedded font file: {}", p);
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
    debug!("using font: {}", typeface.family_name());

    let font = Font::from_typeface(typeface, Some(text_object.size));
    Ok(font)
}

/// make TextBlob from TextCode
fn from_text_val(
    // origin: (f32, f32),
    text_val: &ofd_base::file::page::TextVal,
    font: &Font,
) -> eyre::Result<TextBlob> {
    let tv = text_val.clone();
    let tc = tv.text_code;
    let text = tc.val;
    let d_points = Deltas::from_dx_dy(tc.delta_x.as_ref(), tc.delta_y.as_ref())?;

    let cgt_map = if let Some(cgt_vec) = tv.cg_transform {
        cgt_vec
            .into_iter()
            .map(|c| (c.code_position as usize, c))
            .collect::<HashMap<_, _>>()
    } else {
        // without cgt just return
        // TODO: use more proper error
        // or maybe this should not happened
        let glyph_len = font.count_text(&text);
        return TextBlob::from_pos_text(&text, &d_points.slice(0, glyph_len), font)
            .ok_or_eyre("can not create TextBlob from text");
    };

    // textblob
    let mut tb = TextBlobBuilder::new();

    let mut point_i: usize = 0;
    let mut skip = 0;
    // TODO: replace this with a more efficient way. doing parse part by part not one by one
    text.chars().enumerate().for_each(|(char_pos, c)| {
        if skip > 0 {
            skip -= 1;
            return;
        }
        let cgt = cgt_map.get(&char_pos);
        if let Some(cgt) = cgt {
            let cc = cgt.code_count.unwrap_or(1) as usize;
            if cc < 1 {
                warn!("invalid cgt.code_count: {}, {:#?}", cc, cgt);
                return;
            }
            skip = cc - 1;
            let gc = cgt.glyph_count.unwrap_or(1) as usize;
            assert!(
                cgt.glyphs.len() >= gc,
                "cgt glyphs not enough expected {} got  {}",
                gc,
                cgt.glyphs.len()
            );
            let (glyphs, points) = tb.alloc_run_pos(font, gc, None);
            glyphs.copy_from_slice(&cgt.glyphs[0..gc]);
            points.copy_from_slice(&d_points.slice(point_i, point_i + cc));
            point_i += cc;
        } else {
            let (glyph, point) = tb.alloc_run_pos(font, 1, None);

            let gid = font.unichar_to_glyph(c as i32);
            glyph[0] = gid;
            point[0] = d_points[point_i];
            point_i += 1;
        }
    });

    let res = tb.make().ok_or_eyre("")?;
    Ok(res)
}

#[derive(Debug)]
struct Deltas {
    points: Vec<Point>,
}

impl Deltas {
    fn from_dx_dy(
        // origin: (f32, f32),
        delta_x: Option<&StArray<String>>,
        delta_y: Option<&StArray<String>>,
    ) -> eyre::Result<Self> {
        let origin = (0.0, 0.0);
        let dxs = delta_x.map(flat_g).transpose()?.unwrap_or(Vec::new());
        let dys = delta_y.map(flat_g).transpose()?.unwrap_or(Vec::new());
        let longest_length = max(dxs.len(), dys.len());

        // first point is origin
        let mut points = Vec::new();
        points.push(Point::from(origin));

        let mut last_point = origin;
        for i in 0..longest_length {
            let dx = dxs.get(i).unwrap_or(&0_f32);
            let dy = dys.get(i).unwrap_or(&0_f32);

            last_point = (last_point.0 + dx, last_point.1 + dy);
            points.push(Point::from(last_point));
        }

        Ok(Self { points })
    }

    fn slice(&self, start: usize, end: usize) -> Vec<Point> {
        assert!(start <= end, "index error");
        if end < self.points.len() {
            self.points[start..end].to_vec()
        } else if self.points.len() <= start {
            let len = end - start;
            let last_e = self.points.last().unwrap();
            [*last_e].repeat(len)
        } else {
            let mut p1 = self.points[start..].to_vec();
            let len = end - start;
            let p2_len = len - p1.len();
            let last_e = self.points.last().unwrap();

            let mut p2 = [*last_e].repeat(p2_len);
            p1.append(&mut p2);
            p1
        }
    }
}

impl Index<usize> for Deltas {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        // self.points.set_len(index);
        // Index::index(&self.points, index)
        if index >= self.points.len() {
            self.points.last().unwrap()
        } else {
            &self.points[index]
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use ofd_base::file::page::{CGTransform, TextCode, TextVal};
    use ofd_base::StBox;
    use skia_safe::{AlphaType, Color4f, EncodedImageFormat, FontMgr, ISize, ImageInfo};
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn test_text_blob_from_text_val() -> Result<()> {
        // init_test_logger()

        // value
        let text = TextObject {
            id: 794,
            ctm: Some(vec![0.3528, 0.0, 0.0, 0.3528, 0.0, 0.0].into()),
            draw_param: None,
            line_width: None,
            cap: None,
            join: None,
            miter_limit: None,
            dash_offset: None,
            dash_pattern: None,
            alpha: None,
            boundary: StBox {
                x: 40.2237,
                y: 248.3684,
                w: 15.2292,
                h: 3.4608,
            },
            name: None,
            font: 115,
            size: 10.56,

            stroke: None,
            fill: None,
            h_scale: None,
            read_direction: None,
            char_direction: None,
            weight: None,
            italic: None,
            fill_color: None,
            stroke_color: None,
            text_vals: vec![TextVal {
                cg_transform: Some(vec![CGTransform {
                    code_position: 0,
                    code_count: Some(4),
                    glyph_count: Some(4),
                    glyphs: StArray(vec![16, 224, 210, 225]),
                }]),
                text_code: TextCode {
                    x: Some(0.0),
                    y: Some(8.7437),
                    delta_x: Some(
                        ["g", "2", "10.56", "10.4438"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                            .into(),
                    ),
                    delta_y: None,
                    val: "".to_string(),
                },
            }],
            visible: None,
            actions: None,
        };
        let tv = text.text_vals[0].clone();

        // font
        let fm = FontMgr::new();
        let mut data = vec![];
        let mut file = File::open("../samples/002/Doc_0/Res/font_13132_0.ttf")?;
        let _ = file.read_to_end(&mut data)?;
        let typeface = fm.new_from_data(&data, 0);
        if typeface.is_none() {
            println!("no match");
            return Ok(());
        }
        let typeface = typeface.unwrap();
        let font = Font::from_typeface(typeface, text.size);

        let res = from_text_val(&tv, &font)?;
        let tb = res;
        let ii = ImageInfo::new_s32(ISize::new(210, 297), AlphaType::Unpremul);
        let mut sur =
            skia_safe::surfaces::raster(&ii, None, None).ok_or_eyre("create surface error")?;
        let can = sur.canvas();
        let color = Color::RED;
        let c4f: Color4f = color.into();
        can.draw_text_blob(tb, (10, 10), &Paint::new(c4f, None));
        can.save();
        let img = sur.image_snapshot();
        let data = img.encode(None, EncodedImageFormat::PNG, 100).unwrap();
        let mut file = File::create("../output/tb_test.png")?;
        let _ = file.write(&data)?;
        Ok(())
    }

    #[test]
    fn test_char_to_i32() {
        let s = "你好！";
        s.chars().for_each(|c| println!("{}", c as i32))
    }

    #[test]
    fn test_repeat() {
        let v = [0].repeat(0);
        assert_eq!(v.len(), 0);
    }
}
