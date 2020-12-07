use ggez::{event::MouseButton, graphics, Context, GameResult};
use nonogram::{Block, Nonogram};

#[path = "./nonogram.rs"]
mod nonogram;

pub struct MainState {
    solved: Nonogram,
    current: Nonogram,
}

impl MainState {
    pub fn new(filename: &str) -> GameResult<Self> {
        let solved = nonogram::load_nonogram_from_file(filename)?;
        let current = vec![vec![Block::Empty; solved[0].len()]; solved.len()];
        Ok(MainState { solved, current })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        nonogram::draw_nonogram(&self.solved, ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if let Some((r, c)) = nonogram::translate_xy_to_rc(&self.solved, x, y, ctx) {
            self.solved[r][c] = self.solved[r][c].from_mouse_button(button);
        }
    }
}
