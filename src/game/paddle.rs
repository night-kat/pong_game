use ggez::*;
use graphics::Rect;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug)]
pub struct Paddle{
    pub hitbox: Rect,
    pub speed: f32,
}

pub enum Location{
    Left,
    Right,
}

/// Creates new paddle for the left side, the width is fixed.
fn left_paddle(screen_height:f32, paddle_height:f32) -> Paddle {
    let left_border = 0.0;
    let paddle_x = left_border + 5.0;
    let paddle_y = screen_height/2.0 - paddle_height / 2.0;
    let paddle_width = 2.0;

    let hitbox = Rect::new(
        paddle_x,
        paddle_y,
        paddle_width,
        paddle_height,
    );

    Paddle { 
        hitbox,
        speed: 1.0 
    }
}

/// Creates new paddle for the right side, the width is fixed.
fn right_paddle(screen_height:f32, paddle_height:f32) -> Paddle {
    let right_border = WINDOW_WIDTH;
    let paddle_width = 2.0;
    // Might look weird, but is needed to compensate for the paddles width
    // and the fact that it is on the right side, which is why 5 is subtracted
    let paddle_x = right_border - 5.0 - paddle_width;
    let paddle_y = screen_height/2.0 - paddle_height / 2.0;

    let hitbox = Rect::new(
        paddle_x,
        paddle_y,
        paddle_width,
        paddle_height,
    );

    Paddle { 
        hitbox,
        speed: 1.0 
    }
}

impl Paddle {
    pub fn new(
        location: Location, 
        screen_height:f32, 
        paddle_height:f32) -> Paddle {

        match location {
            Location::Left => left_paddle(screen_height, paddle_height),
            Location::Right => right_paddle(screen_height, paddle_height),
        }
    }

    pub fn move_up(&mut self) {
        self.hitbox.y -= self.speed;
    }

    pub fn move_down(&mut self) {
        self.hitbox.y += self.speed
    }

    pub fn is_in_bounds_up(&self) -> bool {
        self.hitbox.top() > 0.0
    }
    
    pub fn is_in_bounds_down(&self) -> bool {
        self.hitbox.bottom() < WINDOW_HEIGHT
    }
    
}

