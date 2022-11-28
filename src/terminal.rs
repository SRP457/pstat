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
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{BarChart, Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame, Terminal,
};

pub struct App {
    pub scroll: (u16, u16),
    pub tree: String,
    pub path: String,
    pub lang_stats: HashMap<String, f64>,
    pub file_stats: HashMap<String, (u32, u32, u64)>,
}

fn ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title("Project Stats")
        .borders(Borders::ALL);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

    draw_tree(f, chunks[0], app);
    draw_gauge(f, chunks[1], app);
    draw_table(f, chunks[1], app);
}

fn draw_tree(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let tree = app.tree.clone();
    let paragraph = Paragraph::new(tree)
        .block(Block::default().borders(Borders::ALL).title("Project Tree"))
        .wrap(Wrap { trim: true })
        .scroll(app.scroll);
    f.render_widget(paragraph, area);
}

fn draw_table(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let rows = app.file_stats.iter().map(|f| {
        let cells = vec![
            Cell::from(f.0.to_string()),
            Cell::from(f.1 .0.to_string()),
            Cell::from(f.1 .1.to_string()),
            Cell::from(f.1 .2.to_string()),
        ];
        Row::new(cells)
    });

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Language", "No.Files", "No.Lines", "Size(Bytes)"])
                .style(Style::default().fg(Color::LightBlue))
                .bottom_margin(1),
        )
        .block(Block::default().title("File Stats").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
        ]);
    f.render_widget(table, chunks[1]);
}

fn draw_gauge(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let temp = &app.lang_stats;
    let mut data = Vec::new();

    for lang in temp {
        if *lang.1 as u64 >= 1 {
            data.push((&lang.0[..], *lang.1 as u64));
        }
    }
    let data = data.as_slice();

    let barchart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Language Stats"),
        )
        .data(&data)
        .bar_width(6)
        .bar_gap(1)
        .bar_set(symbols::bar::THREE_LEVELS)
        .value_style(Style::default().fg(Color::Black).bg(Color::LightBlue))
        .label_style(Style::default().fg(Color::White))
        .bar_style(Style::default().fg(Color::LightBlue));

    f.render_widget(barchart, chunks[0]);
}

pub fn setup_terminal(app: &mut App) -> Result<(), io::Error> {
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
                    if lines - app.scroll.0 > 15 {
                        app.scroll.0 += 1;
                        terminal.draw(|f| ui(f, &app))?;
                    }
                }
                KeyCode::Up => {
                    if app.scroll.0 > 0 {
                        app.scroll.0 -= 1;
                        terminal.draw(|f| ui(f, &app))?;
                    } else {
                        app.scroll.0 = 0;
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
