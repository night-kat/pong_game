use ggez::{graphics::{self, Canvas, Color, DrawParam}, mint::Point2, GameResult};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::Paddle;

pub struct Velocity {
    vertical: f32,
    horizontal: f32,
}

impl Velocity {
    pub fn new(vertical: f32, horizontal: f32) -> Velocity {
        Velocity {
            vertical,
            horizontal,
        }
    }

    pub fn invert_horizontal(&mut self) {
        self.horizontal *= -1.0
    }
    
    pub fn invert_vertical(&mut self) {
        self.vertical *= -1.0
    }
}

pub struct Ball {
    pub position: Point2<f32>,
    pub radius: f32,
    /// Velocity of the ball: (horizontal velocity, vertical velocity)
    pub velocity: Velocity,
}

impl Ball {
    pub fn new(position: Point2<f32>, radius: f32, velocity: Velocity) -> Ball {
        Ball {
            position,
            radius,
            velocity,
        }
    }
    
    pub fn default() -> Ball {
        Ball{
            position: Point2{x: WINDOW_WIDTH/2.0, y: WINDOW_HEIGHT/2.0},
            radius: 3.0,
            velocity: Velocity { vertical: 1.0, horizontal: 1.0 } 
        }
    }

    pub fn check_collision(&self, paddle: &Paddle) -> bool {
        // Calculate the closest point on the paddle to the ball
        let closest_x = self.position.x
            .clamp(paddle.hitbox.x, paddle.hitbox.x + paddle.hitbox.h);
        let closest_y = self.position.y
            .clamp(paddle.hitbox.y, paddle.hitbox.y + paddle.hitbox.h);

        // Calculate the distance from the ball's center to this closest point
        let distance_x = self.position.x - closest_x;
        let distance_y = self.position.y - closest_y;

        // Check if the distance is less than the ball's radius
        (distance_x * distance_x + distance_y * distance_y) < (self.radius * self.radius)
}

    /// Returns the location of the right edge of the circle
    pub fn right(&self) -> f32 {
        self.position.x - self.radius
    }
    
    /// Returns the location of the left edge of the circle
    pub fn left(&self) -> f32 {
        self.position.x + self.radius
    }

    /// Returns the location of the bottom edge of the cirle
    pub fn bottom(&self) -> f32 {
        self.position.y + self.radius
    }

    /// Returns the location of the top edge of the circle
    pub fn top(&self) -> f32 {
        self.position.y - self.radius
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut Canvas) -> GameResult {
        let circle = graphics::Mesh::new_circle(ctx,
            graphics::DrawMode::fill(), 
            self.position, 
            self.radius, 
            0.1, 
            Color::WHITE,
            );
        
        let _ = &canvas.draw(&circle.unwrap(), DrawParam::default());
        Ok(())
    }
    /// Will check for collisions
    /// Will then invert vertical speed, should it hit a border. 
    pub fn check_border_collisions(&mut self) {

        // upper border of window
        if self.top() < 0.0 {
            self.velocity.invert_vertical();
        } 

        // lower border of window
        if  self.bottom() > WINDOW_HEIGHT {
            self.velocity.invert_vertical();
        }

    }

    pub fn move_ball(&mut self) {
        self.position.x += self.velocity.horizontal;
        self.position.y += self.velocity.vertical;
    }

    pub fn reset(&mut self) {
        self.position = Point2{ x: WINDOW_WIDTH/2.0, y: WINDOW_HEIGHT/2.0};
    }

}
