use ggez::GameResult;

mod state;

fn main() -> GameResult<()> {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("nonogram", "icedynamix")
        .window_mode(ggez::conf::WindowMode::default().dimensions(640.0, 480.0))
        .build()?;

    let main_state = &mut state::MainState::new("./nonograms/penguin.txt")?;
    ggez::event::run(ctx, event_loop, main_state)?;

    Ok(())
}
