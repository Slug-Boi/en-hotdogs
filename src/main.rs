use std::{
    env, io::{self, stdout},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,

    layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}
};


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Hello, world!");

    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen
    )?;

    terminal.clear()?;

    run(&mut terminal)?;

    disable_raw_mode()?;


    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    Ok(())
}


fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>>  {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let widget = Block::default().borders(Borders::ALL).title("En Hotdogs").title_bottom("Quit with CTRL+Q");
            frame.render_widget(widget, area);
        })?;

        if handle_events()? {
            break;
        }
    }

    Ok(())
}

fn handle_events() -> Result<bool, Box<dyn std::error::Error>> {
    let e = event::read()?;
    match e {
        Event::Key(key) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    Ok(false)
}