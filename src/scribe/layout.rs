use unicode_normalization::UnicodeNormalization;
use rusttype::{Font, PositionedGlyph, Scale};
use super::glyph_writer::GlyphWriter;

pub fn fit_text<'a>(font: &'a Font, text: &str, mut scale: Scale, width: i32, placement: f32) -> Vec<PositionedGlyph<'a>>
{
    let mut some_glyphs: Option<Vec<PositionedGlyph>> = None;
    while let None = some_glyphs {
        let lines = break_text(font, text, scale, width, placement);
        let line_count = lines.len();
        if line_count <= 1 || scale.x <= 1.0 {
            let glyphs = lines.into_iter().fold(Vec::new(), |mut glyphs, (_, more)| {
                glyphs.extend(more);
                glyphs
            });
            some_glyphs = Some(glyphs);
        } else {
            scale.x = (scale.x * 0.95).max(1.0);
        }
    }
    some_glyphs.expect("glyphs")
}

pub fn break_text<'a>(font: &'a Font, text: &str, scale: Scale, max_width: i32, placement: f32) -> Vec<(f32, Vec<PositionedGlyph<'a>>)> {
    let mut glyph_writer = GlyphWriter::new(&font.v_metrics(scale));
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\r' => glyph_writer.feed_line(),
                '\n' => (),
                _ => (),
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        let kerning = if let Some(id) = last_glyph_id.take() {
            font.pair_kerning(scale, id, base_glyph.id())
        } else {
            0.0
        };
        last_glyph_id = Some(base_glyph.id());

        glyph_writer.feed_right(kerning);
        let mut glyph = base_glyph.scaled(scale).positioned(glyph_writer.position());
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            if bounding_box.max.x > max_width as i32 {
                glyph_writer.feed_right(-kerning);
                glyph_writer.feed_line();
                glyph = glyph.into_unpositioned().positioned(glyph_writer.position());
                last_glyph_id = None;
            }
        }
        glyph_writer.feed_right(glyph.unpositioned().h_metrics().advance_width);
        glyph_writer.add_glyph(glyph);
    }
    glyph_writer.take_lines(max_width as f32, placement)
}
