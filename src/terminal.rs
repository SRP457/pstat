use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::HashMap,
    io::{self, Stdout},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

use crate::ui;

pub struct TApp {
    pub scroll: (u16, u16),
    pub tree: String,
    pub path: String,
    pub branches: String,
    pub log: String,
    pub status: String,
    pub lang_stats: HashMap<String, f64>,
    pub file_stats: HashMap<String, (u32, u32, u64)>,
    pub file_time: Vec<String>,
    pub tab: u32,
}

fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &TApp) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tabs_list = vec![
        Spans::from(Span::styled("Home", Style::default().fg(Color::White))),
        Spans::from(Span::styled("Git", Style::default().fg(Color::White))),
    ];

    let tabs = Tabs::new(tabs_list)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tabs")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .highlight_style(Style::default().fg(Color::LightBlue))
        .select(app.tab as usize);
    f.render_widget(tabs, chunks[0]);

    if app.tab == 0 {
        ui::home_tab(f, chunks[1], &app);
    } else {
        ui::git_tab(f, chunks[1], &app);
    }
}

pub fn setup_terminal(app: &mut TApp) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| ui(f, &app))?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    let lines: u16 = app.tree.lines().count().try_into().unwrap();
                    if lines - app.scroll.0 > 15 && app.tab == 0 {
                        app.scroll.0 += 1;
                        terminal.draw(|f| ui(f, &app))?;
                    }
                }
                KeyCode::Up => {
                    if app.scroll.0 > 0 && app.tab == 0 {
                        app.scroll.0 -= 1;
                        terminal.draw(|f| ui(f, &app))?;
                    }
                }
                KeyCode::Right => {
                    if app.tab == 0 {
                        app.tab = 1;
                        terminal.draw(|f| ui(f, &app))?;
                    }
                }
                KeyCode::Left => {
                    if app.tab == 1 {
                        app.tab = 0;
                        terminal.draw(|f| ui(f, &app))?;
                    }
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
