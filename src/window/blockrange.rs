#[derive(Copy, Clone, Debug)]
pub struct BlockRange {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub approach: f32,
}

impl BlockRange {
    pub fn with_approach(&self, approach: f32) -> Self {
        let mut range = self.clone();
        range.approach = approach;
        range
    }

    pub fn with_more_approach(&self, more: f32) -> Self {
        self.with_approach(self.approach + more)
    }

    pub fn with_padding(&self, h_pad: f32, v_pad: f32) -> Self {
        BlockRange {
            left: self.left + h_pad,
            top: self.top + v_pad,
            width: (self.width - 2.0 * h_pad).max(0.0),
            height: (self.height - 2.0 * v_pad).max(0.0),
            approach: self.approach,
        }
    }
    pub fn split_width(&self, right_width: f32) -> (Self, Self) {
        let right_width = right_width.min(self.width);
        let left_width = self.width - right_width;
        let left_range = BlockRange {
            left: self.left,
            top: self.top,
            width: left_width,
            height: self.height,
            approach: self.approach,
        };
        let right_range = BlockRange {
            left: self.left + left_width,
            top: self.top,
            width: right_width,
            height: self.height,
            approach: self.approach,
        };
        (left_range, right_range)
    }
    pub fn split_height(&self, bottom_height: f32) -> (Self, Self) {
        let bottom_height = bottom_height.min(self.height).max(0.0);
        let top_height = self.height - bottom_height;
        let top_range = BlockRange {
            left: self.left,
            top: self.top,
            width: self.width,
            height: top_height,
            approach: self.approach,
        };
        let bottom_range = BlockRange {
            left: self.left,
            top: self.top + top_height,
            width: self.width,
            height: bottom_height,
            approach: self.approach,
        };
        (top_range, bottom_range)
    }
}
