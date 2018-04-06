use rusttype::{PositionedGlyph, VMetrics};
use rusttype::{Point, point};

pub struct GlyphWriter<'a> {
    line_stride: f32,
    caret: Point<f32>,
    line: Vec<PositionedGlyph<'a>>,
    page: Vec<(f32, Vec<PositionedGlyph<'a>>)>,
}

impl<'a> GlyphWriter<'a> {
    pub fn new(v_metrics: &VMetrics) -> Self {
        let line_stride = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let caret = point(0.0, v_metrics.ascent);
        GlyphWriter { line_stride, caret, line: Vec::new(), page: Vec::new() }
    }
    pub fn position(&self) -> Point<f32> {
        self.caret
    }
    pub fn feed_right(&mut self, amount: f32) {
        self.caret.x += amount;
    }
    pub fn add_glyph(&mut self, glyph: PositionedGlyph<'a>) {
        self.line.push(glyph);
    }
    pub fn feed_line(&mut self) {
        let line_width = self.caret.x;
        self.caret = point(0.0, self.caret.y + self.line_stride);
        let mut line = Vec::new();
        line.append(&mut self.line);
        self.page.push((line_width, line));
    }
    pub fn take_lines(&mut self, max_width: f32, placement: f32) -> Vec<(f32, Vec<PositionedGlyph<'a>>)> {
        let mut lines = Vec::new();
        lines.append(&mut self.page);
        if !self.line.is_empty() {
            let mut line = Vec::new();
            line.append(&mut self.line);
            lines.push((self.caret.x, line));
        }
        lines.into_iter()
             .map(|(line_width, line_glyphs)| {
                 let extra = (max_width - line_width) * placement;
                 let placed_glyphs = line_glyphs.into_iter()
                                                .map(|glyph| {
                                                    let mut position = glyph.position();
                                                    position.x += extra;
                                                    glyph.into_unpositioned().positioned(position)
                                                })
                                                .collect::<Vec<_>>();
                 (max_width, placed_glyphs)
             })
             .collect::<Vec<_>>()
    }
}
