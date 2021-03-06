use ggez::{
    conf::{WindowMode, WindowSetup},
    GameResult,
};
use state::MainState;

mod state;

fn main() -> GameResult<()> {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("nonogram", "icedynamix")
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .window_setup(WindowSetup::default().title("Nonogram"))
        .build()?;

    let main_state = &mut MainState::new("./nonograms/penguin.txt")?;
    ggez::event::run(ctx, event_loop, main_state)?;

    Ok(())
}
