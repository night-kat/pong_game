mod game;
use ggez::conf::{self};
use ggez::{glam, graphics, GameResult};
use ggez::graphics::Color;
use ggez::event;
use ggez::ContextBuilder;
use ggez::graphics::Canvas;
use ggez::GameError;
use ggez::Context;
use crate::game::paddle::Paddle;
use crate::conf::Conf;
use crate::graphics::Rect;
use crate::game::paddle::Location;

// I made it so 100.0 is the other end of the window.
// This makes it easier to draw shapes that are centered
// And it *should* also work with different scale factors.
// When drawing something, import these and use them instead of the
// height field in conf::window_mode
pub const WINDOW_HEIGHT:f32 = 100.0;
pub const WINDOW_WIDTH:f32 = 100.0;

struct MainState {
    left_paddle: Paddle,
    right_paddle: Paddle
}

impl MainState {
    fn new(ctx: &Context, c: &Conf, window_height:f32) -> MainState {
        // Appropriate size for paddle(i hope)
        let paddle_height = window_height/6.0;

        let left_paddle = Paddle::new(
            ctx,
            Location::Left,
            window_height,
            paddle_height,
        );

        let right_paddle = Paddle::new(
            ctx, 
            Location::Right, 
            window_height, 
            paddle_height
        );

        MainState {
            left_paddle,
            right_paddle,
        }
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(Rect::new(0.0,0.0,100.0,100.0));
        let my_dest = glam::vec2(0.0, 0.0);
        canvas.draw(&self.left_paddle.mesh, graphics::DrawParam::new().dest(my_dest));
        canvas.draw(&self.right_paddle.mesh, graphics::DrawParam::new().dest(my_dest));

        canvas.finish(ctx)
    }

}

fn main() -> GameResult {

    let c = ggez::conf::Conf {
        window_mode: ggez::conf::WindowMode::default().resize_on_scale_factor_change(true),
        window_setup: conf::WindowSetup::default(),
        backend: conf::Backend::default(),
    };

    dbg!(&c);

    let (ctx, event_loop) = ContextBuilder::new("Pong", "nightcat")
        .default_conf(c.clone())
        .build()
        .unwrap();
    
    let state = MainState::new(&ctx, &c, WINDOW_WIDTH);
    
    event::run(ctx, event_loop, state)
}
