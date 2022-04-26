use tetra::{Context, State};
use tetra::graphics::{self, DrawParams, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use entity::Entity;
use background::Background;
use score::Score;

mod background;
mod score;
pub mod entity;

const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 1.05;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPEED: f32 = 8.0;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 960.0;
const SCORE_TO_WIN: i32 = 2;


fn draw_entity(ctx: &mut Context, entity: &Entity) {
    entity.texture.draw(ctx, entity.position);
}


pub struct GameState {
    background: Background,
    player1: Entity,
    player2: Entity,
    ball: Entity,
    rotation: f32,
    txt: Text,
    score: Score,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "res/bar-mini.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "res/fox-pixel.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "res/ball-pixel.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT / 2.0,
        );
        let ball_velocity = Vec2::new(
            -BALL_SPEED,
            0.0,
        );

        let branding = Text::new("SiMiTo", Font::vector(ctx, "res/lato/Lato-BoldItalic.ttf", 12.0)?);


        Ok(GameState { background: Background::new(),
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            rotation: 0f32,
            txt: branding,
            score: Score::new(ctx, 0, 0)?,
        })
    }

    fn ball_bounce(&mut self) {
        let ball = &self.ball;

        let paddle_hit = if ball.intersects(&self.player1) {
            Some(&self.player1)
        } else if ball.intersects(&self.player2) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(self.ball.velocity.x * BALL_ACC);

            let offset = (paddle.y_coordinate_centre() - self.ball.y_coordinate_centre()) / paddle.height();

            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        self.ball.update();
    }
}


impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        self.background.update();

        self.rotation = self.rotation + 0.01f32;

        self.ball_bounce();

        if self.ball.position.x <= 0.0 {
            self.score.goal_player_2();
            self.ball.reset(Vec2::new(-BALL_SPEED, 0.));
        }

        if self.ball.position.x >= WINDOW_WIDTH {
            self.score.goal_player_1();
            self.ball.reset(Vec2::new(BALL_SPEED, 0.));
        }

        if self.score.player1 >= SCORE_TO_WIN || self.score.player2 >= SCORE_TO_WIN {
            self.score.reset_score();
            println!("You should read a book or sth.");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, self.background.color());

        draw_entity(ctx, &self.player1);
        draw_entity(ctx, &self.player2);
        draw_entity(ctx, &self.ball);

        self.txt.draw(ctx, DrawParams::new().position(Vec2::new(WINDOW_WIDTH / 2f32,50f32)).rotation(self.rotation));

        self.score.draw(ctx);

        Ok(())
    }
}

