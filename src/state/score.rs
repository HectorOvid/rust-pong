use tetra::Context;
use tetra::graphics::DrawParams;
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use crate::state::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub(super) struct Score {
    text: Text,
    pub player1: i32,
    pub player2: i32,
    position: Vec2<f32>,
}

impl Score {
    pub fn new(ctx: &mut Context, player1: i32, player2: i32) -> tetra::Result<Score> {

        let score = Text::new(Score::format(0, 0), Font::vector(ctx, "res/lato/Lato-Regular.ttf", 24.0)?);
        Ok(Score { text: score, player1, player2, position: Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT - 40.) })
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.text.draw(ctx, DrawParams::new().position(self.position));
    }

    pub fn goal_player_1(&mut self) {
        self.player1 += 1;
        self.update_score_text()
    }

    pub fn goal_player_2(&mut self) {
        self.player2 += 1;
        self.update_score_text()
    }

    pub fn reset_score(&mut self) {
        self.player1 = 0;
        self.player2 = 0;
        self.update_score_text()
    }

    fn update_score_text(&mut self) {
        self.text.set_content(format!("{} : {}", self.player1, self.player2));
    }

    fn format(a: i32, b: i32) -> String {
        format!("{} : {}", a, b)
    }

}
