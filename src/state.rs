use ggez::{graphics, GameResult};
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
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        nonogram::draw_nonogram(&self.solved, ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}
