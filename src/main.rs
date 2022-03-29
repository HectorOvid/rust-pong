use tetra::{Context, ContextBuilder, State, TetraError};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;

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
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity { texture, position, velocity }
    }

}

struct GameState {
    background: Background,
    player1: Entity,
    player2: Entity,
    ball: Entity,
    rotation: f32,
    txt: Text,
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
            1 as f32,
            1 as f32,
        );

        let txt = Text::new("SiMiTo", Font::vector(ctx, "./res/lato/Lato-BoldItalic.ttf", 12.0)?);

        Ok(GameState { background: Background::new(),
            player1: Entity::new(player1_texture, player1_position, Vec2::zero()),
            player2: Entity::new(player2_texture, player2_position, Vec2::zero()),
            ball: Entity::new(ball_texture, ball_position, ball_velocity),
            rotation: 0f32,
            txt,
        })
    }

}

fn draw_entity(ctx: &mut Context, entity: &Entity) {
    entity.texture.draw(ctx, entity.position);
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        self.background.update();

        self.rotation = self.rotation + 0.01f32;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, self.background.color());

        draw_entity(ctx, &self.player1);
        draw_entity(ctx, &self.player2);
        draw_entity(ctx, &self.ball);

        self.txt.draw(ctx, DrawParams::new().position(Vec2::new(WINDOW_WIDTH / 2f32,50f32)).rotation(self.rotation));

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
