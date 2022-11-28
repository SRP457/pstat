use crate::terminal::App;
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{BarChart, Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

fn git_branch(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let branches = app.branches.clone();
    let paragraph = Paragraph::new(branches)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Branches")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let status = app.status.clone();
    let paragraph = Paragraph::new(status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Status")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
}

fn git_log(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let log = app.log.clone();
    let paragraph = Paragraph::new(log)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Log")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_tree(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &App) {
    let tree = app.tree.clone();
    let paragraph = Paragraph::new(tree)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Project Tree")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
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
        .block(
            Block::default()
                .title("File Stats")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightBlue)),
        )
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
                .title("Language Stats")
                .border_style(Style::default().fg(Color::LightBlue)),
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

pub fn home_tab(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect, app: &App) {
    let block = Block::default()
        .title("Project Stats")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_tree(f, chunks[0], app);
    draw_gauge(f, chunks[1], app);
    draw_table(f, chunks[1], app);
}

pub fn git_tab(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect, app: &App) {
    let block = Block::default()
        .title("Git Stats")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    git_branch(f, chunks[0], app);
    git_log(f, chunks[1], app);
}
