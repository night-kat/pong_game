mod game;
use game::ball::Ball;
use ggez::input::keyboard::KeyCode;
use ggez::conf::{self};
use ggez::{graphics, GameResult};
use ggez::graphics::Color;
use ggez::event;
use ggez::ContextBuilder;
use ggez::graphics::Canvas;
use ggez::GameError;
use crate::game::paddle::Paddle;
use crate::graphics::Rect;
use crate::game::paddle::Location;

// I made it so 100.0 is the other end of the window.
// This makes it easier to draw shapes that are centered
// And it *should* also work with different scale factors.
// When drawing something, import these and use them instead of the
// height field in conf::window_mode
pub const WINDOW_HEIGHT:f32 = 150.0;
pub const WINDOW_WIDTH:f32 = 200.0;

pub struct MainState {
    pub ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    left_player_score: u32,
    right_player_score: u32,
}

impl MainState {
    fn new(window_height:f32) -> MainState {
        // Appropriate size for paddle(i hope)
        let paddle_height = window_height/6.0;

        let left_paddle = Paddle::new(
            Location::Left,
            window_height,
            paddle_height,
        );

        let right_paddle = Paddle::new(
            Location::Right, 
            window_height, 
            paddle_height
        );
        
        let ball = Ball::default();

        MainState {
            ball,
            left_paddle,
            right_paddle,
            left_player_score: 0,
            right_player_score: 0,
        }
    }

    /// Simply adds one to the left players score
    fn increment_left_score(&mut self) {
        self.left_player_score += 1;
        self.ball.reset();
    }
    
    /// Simply adds one to the right players score
    fn increment_right_score(&mut self) {
        self.right_player_score += 1;
        self.ball.reset();
    }
    
    fn check_and_increment_score(&mut self) {
        if self.ball.right() < 0.0 {
            self.increment_right_score();
            self.ball.velocity.invert_horizontal();
        }

        if self.ball.left() > WINDOW_WIDTH {
            self.increment_left_score();
            self.ball.velocity.invert_horizontal();
        }
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let k_ctx = &ctx.keyboard;

        if self.left_paddle.is_in_bounds_up() & k_ctx.is_key_pressed(KeyCode::W) {
            self.left_paddle.move_up();
        }

        if self.left_paddle.is_in_bounds_down() & k_ctx.is_key_pressed(KeyCode::S) {
                self.left_paddle.move_down()
        }

        if self.right_paddle.is_in_bounds_up() & k_ctx.is_key_pressed(KeyCode::Up) {
            self.right_paddle.move_up();
        }

        if self.right_paddle.is_in_bounds_down() & k_ctx.is_key_pressed(KeyCode::Down) {
            self.right_paddle.move_down()
        }

        self.ball.move_ball();
        self.ball.check_border_collisions();
        self.check_and_increment_score();
        if self.ball.check_collision(&self.left_paddle) {
            self.ball.velocity.invert_horizontal();
        } 

        if self.ball.check_collision(&self.right_paddle) {
            self.ball.velocity.invert_horizontal();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0,0.0,WINDOW_WIDTH,WINDOW_HEIGHT));

        self.ball.draw(ctx, &mut canvas)?;

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.left_paddle.hitbox.point())
                .scale(self.left_paddle.hitbox.size())
                .color(Color::WHITE),
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.right_paddle.hitbox.point())
                .scale(self.right_paddle.hitbox.size())
                .color(Color::WHITE),
        );
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let c = ggez::conf::Conf {
        window_mode: ggez::conf::WindowMode::default()
            .resize_on_scale_factor_change(true),
        window_setup: conf::WindowSetup::default(),
        backend: conf::Backend::default(),
    };

    let (ctx, event_loop) = ContextBuilder::new("Pong", "nightcat")
        .default_conf(c.clone())
        .build()
        .unwrap();
    
    let state = MainState::new(WINDOW_HEIGHT);
    
    event::run(ctx, event_loop, state)
}
