use tetra::{Context, ContextBuilder, State, TetraError};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::window::WindowPosition::Positioned;

const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 1.05;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPEED: f32 = 8.0;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 960.0;

struct Background {
    blue: f32,
    color_direction: bool,
}

impl Background {
    fn new() -> Background {
        Background {blue: 0f32, color_direction: true}
    }

    fn color(& self) -> Color {
        Color::rgb(0.2, 0.7, self.blue)
    }

    fn update(&mut self) -> f32 {
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

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    original: Vec2<f32>,
    velocity: Vec2<f32>,
}

struct Score {
    text: Text,
    player1: i32,
    player2: i32,
    position: Vec2<f32>,
}

impl Score {
    fn draw(&mut self, ctx: &mut Context) {
        self.text.draw(ctx, DrawParams::new().position(self.position));
    }

    fn goal_player_1(&mut self) {
        self.player1 += 1;
        self.update_score_text()
    }

    fn goal_player_2(&mut self) {
        self.player2 += 1;
        self.update_score_text()
    }

    fn update_score_text(&mut self) {
        self.text.set_content(format!("{} : {}", self.player1, self.player2));
    }

    fn format(a: i32, b: i32) -> String {
        format!("{} : {}", a, b)
    }
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            original: position,
            velocity,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32  {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn intersects(&self, entity :&Entity) -> bool {
        self.bounds().intersects(&entity.bounds())
    }

    fn y_coordinate_centre(&self) -> f32 {
        self.position.y + (self.height() / 2.0)
    }

    fn reset(&mut self, velocity: Vec2<f32>) {
        self.position.x = self.original.x;
        self.position.y = self.original.y;
        self.velocity = velocity;
    }
}

struct GameState {
    background: Background,
    player1: Entity,
    player2: Entity,
    ball: Entity,
    rotation: f32,
    txt: Text,
    score: Score,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./res/bar-mini.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./res/fox-pixel.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./res/ball-pixel.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT / 2.0,
        );
        let ball_velocity = Vec2::new(
            -BALL_SPEED,
            0.0,
        );

        let branding = Text::new("SiMiTo", Font::vector(ctx, "./res/lato/Lato-BoldItalic.ttf", 12.0)?);
        let score = Text::new(Score::format(0, 0), Font::vector(ctx, "./res/lato/Lato-Regular.ttf", 24.0)?);

        Ok(GameState { background: Background::new(),
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            rotation: 0f32,
            txt: branding,
            score: Score { text: score, player1: 0, player2: 0, position: Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT - 40.) },
        })
    }

}

fn draw_entity(ctx: &mut Context, entity: &Entity) {
    entity.texture.draw(ctx, entity.position);
}


impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.process_user_input(ctx);
        self.background.update();
        self.update_logo();
        self.ball_bounce();
        self.update_score();

        if self.score.player1 + self.score.player2 >= 10 {
            window::quit(ctx);
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

impl GameState {
    fn process_user_input(&mut self, ctx: &mut Context) {
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

            self.ball.velocity.y = (self.ball.velocity.y + PADDLE_SPIN * -offset) * BALL_ACC;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        self.ball.update();
    }

    fn update_logo(&mut self) {
        self.rotation = self.rotation + 0.01f32;
    }

    fn update_score(&mut self) {
        if self.ball.position.x <= 0.0 {
            self.score.goal_player_2();
            self.ball.reset(Vec2::new(-BALL_SPEED, 0.));
        }

        if self.ball.position.x >= WINDOW_WIDTH {
            self.score.goal_player_1();
            self.ball.reset(Vec2::new(BALL_SPEED, 0.));
        }
    }
}


fn main() -> tetra::Result {
    println!("Hi Simon chill");

    let ran = ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new);

    // debugging / error / or whatever
    match ran {
        Ok(_) => {
            println!("Hurra");
        }
        Err(_) => {
            println!("Flark frigging dragons frigged it up again");
        }
    }

    ran
}
