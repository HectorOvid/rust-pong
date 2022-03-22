use tetra::{Context, ContextBuilder, State, TetraError};
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 960.0;

struct GameState {
    blue: f32,
    color_direction: bool,
    // NOTE: this is only an ID, quasi :)
    paddle_texture: Texture,
    other_paddle_texture: Texture,
    ball: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "./res/bar-mini.png")?;
        let other_paddle_texture = Texture::new(ctx, "./res/fox-pixel.png")?;
        let ball = Texture::new(ctx, "./res/ball-pixel.png")?;
        Ok(GameState { blue: 0f32, color_direction: true, paddle_texture, other_paddle_texture, ball })
    }

    fn update_background(&mut self) -> f32 {
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

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.2, 0.7, self.update_background()));

        self.paddle_texture.draw(ctx, Vec2::new(16.0, 16.0));
        self.other_paddle_texture.draw(ctx, Vec2::new(1000.0, 740.0));
        self.ball.draw(ctx, Vec2::new(640.0, 400.0));

        Ok(())
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
