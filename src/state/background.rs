use tetra::graphics::Color;

pub(super) struct Background {
    blue: f32,
    color_direction: bool,
}

impl Background {
    pub fn new() -> Background {
        Background {blue: 0f32, color_direction: true}
    }

    pub fn color(& self) -> Color {
        Color::rgb(0.2, 0.7, self.blue)
    }

    pub fn update(&mut self) -> f32 {
        let mut b;
        if self.color_direction {
            b = self.blue + 0.001;
            if b > 1f32 {
                self.color_direction = !self.color_direction;
                b = 1f32
            }
        } else {
            b = self.blue - 0.001;
            if b < 0f32 {
                self.color_direction = !self.color_direction;
                b = 0f32
            }
        }
        self.blue = b;
        b
    }
}
