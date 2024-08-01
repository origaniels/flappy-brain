use std::{io::{stdout, Result}, time::Instant};
use backend::game::Game;
use frontend::interface::Drawable;
use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
    }, Terminal
};

fn main()->Result<()>{
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;
    let mut last_frame: Instant = Instant::now();
    let mut game = Game::new();

    loop {
        if game.player_is_hit() {
            break;
        }
        term.draw(|frame| {
            let area = frame.size();
            game.display(area, frame);
        })?;

        if last_frame.elapsed().as_millis()>10 {
            game.next_state();
            last_frame = Instant::now();
        }

        if event::poll(std::time::Duration::from_millis(5))? {
            match event::read()?{
                event::Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char(' ') {
                            game.jump();
                        }
                        if key.code == KeyCode::Esc {
                            break;
                        }
                    }
                },
                _=>()
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
