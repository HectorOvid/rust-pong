use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;

pub(super) struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub original: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    pub fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            original: position,
            velocity,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
    }

    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn height(&self) -> f32  {
        self.texture.height() as f32
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    pub fn intersects(&self, entity :&Entity) -> bool {
        self.bounds().intersects(&entity.bounds())
    }

    pub fn y_coordinate_centre(&self) -> f32 {
        self.position.y + (self.height() / 2.0)
    }

    pub fn reset(&mut self, velocity: Vec2<f32>) {
        self.position.x = self.original.x;
        self.position.y = self.original.y;
        self.velocity = velocity;
    }
}
