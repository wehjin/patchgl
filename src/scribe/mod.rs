use rusttype::{Font, PositionedGlyph, Scale};
use self::glyph_writer::GlyphWriter;
use unicode_normalization::UnicodeNormalization;


mod glyph_writer;

pub fn layout_fitted_glyphs<'a>(font: &'a Font, text: &str, mut scale: Scale, width: u32, placement: f32) -> Vec<PositionedGlyph<'a>>
{
    let mut some_glyphs: Option<Vec<PositionedGlyph>> = None;
    while let None = some_glyphs {
        let (glyphs, line_count) = layout_glyphs(font, text, scale, width, placement);
        if line_count <= 1 || scale.x <= 1.0 {
            some_glyphs = Some(glyphs);
        } else {
            scale.x = (scale.x * 0.95).max(1.0);
        }
    }
    some_glyphs.expect("glyphs")
}

fn layout_glyphs<'a>(font: &'a Font, text: &str, scale: Scale, width: u32, placement: f32) -> (Vec<PositionedGlyph<'a>>, usize) {
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
            if bounding_box.max.x > width as i32 {
                glyph_writer.feed_right(-kerning);
                glyph_writer.feed_line();
                glyph = glyph.into_unpositioned().positioned(glyph_writer.position());
                last_glyph_id = None;
            }
        }
        glyph_writer.feed_right(glyph.unpositioned().h_metrics().advance_width);
        glyph_writer.add_glyph(glyph);
    }
    glyph_writer.take_glyphs(width as f32, placement)
}
