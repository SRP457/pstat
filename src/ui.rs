use crate::terminal::TApp;
use std::{
    cmp::Ordering,
    io::{self, Stdout},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

fn git_branch(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &TApp) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let branches = app.branches.clone();
    let paragraph = Paragraph::new(branches)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Branches")
                .border_style(Style::default().fg(app.app_color)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let status = app.status.clone();
    let paragraph = Paragraph::new(status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Status")
                .border_style(Style::default().fg(app.app_color)),
        )
        .wrap(Wrap { trim: true })
        .scroll(app.status_scroll);
    f.render_widget(paragraph, chunks[1]);
}

fn git_log(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &TApp) {
    let log = if app.verbose {
        app.log_tree.clone()
    } else {
        app.log.clone()
    };
    let paragraph = Paragraph::new(log)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Log")
                .border_style(Style::default().fg(app.app_color)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_tree(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &TApp) {
    let tree = app.tree.clone();
    let paragraph = Paragraph::new(tree)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Project Tree")
                .border_style(Style::default().fg(app.app_color)),
        )
        .wrap(Wrap { trim: true })
        .scroll(app.scroll);
    f.render_widget(paragraph, area);
}

fn draw_table(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &TApp) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let chunks1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[1]);

    let mut count_time: Vec<(&String, &(u32, u32, u64))> = app.file_stats.iter().collect();
    count_time.sort_by(|a, b| b.1 .2.cmp(&a.1 .2));

    let rows = count_time.iter().map(|f| {
        let cells = vec![
            Cell::from(f.0.to_string().trim().to_owned()),
            Cell::from(f.1 .0.to_string()),
            Cell::from(f.1 .1.to_string()),
            Cell::from(f.1 .2.to_string()),
        ];
        Row::new(cells)
    });

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Language", "No.Files", "No.Lines", "Size(B)"])
                .style(Style::default().fg(app.app_color))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .title("File Stats")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.app_color)),
        )
        .widths(&[
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ]);
    f.render_widget(table, chunks1[0]);

    let mut files = String::new();
    for (ind, f) in app.file_time.clone().iter().enumerate() {
        let temp = (ind + 1).to_string() + ". " + f + "\n";
        files.push_str(&temp[..]);
    }
    // let file_time = app.file_time.clone().join("\n");
    let paragraph = Paragraph::new(files)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Recently Modified")
                .border_style(Style::default().fg(app.app_color)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks1[1]);
}

fn draw_gauge(f: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect, app: &TApp) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // let temp = &app.lang_stats;
    let mut temp: Vec<(&String, &f64)> = app.lang_stats.iter().collect();
    temp.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

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
                .title("Language Distribution")
                .border_style(Style::default().fg(app.app_color)),
        )
        .data(&data)
        .bar_width(6)
        .bar_gap(2)
        .value_style(Style::default().fg(Color::Black).bg(app.app_color))
        .label_style(Style::default().fg(Color::White))
        .bar_style(Style::default().fg(app.app_color));

    f.render_widget(barchart, chunks[0]);
}

pub fn home_tab(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect, app: &TApp) {
    let block = Block::default()
        .title("Project Stats")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.app_color));
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

pub fn git_tab(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect, app: &TApp) {
    let block = Block::default()
        .title("Git Stats")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.app_color));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    git_branch(f, chunks[0], app);
    git_log(f, chunks[1], app);
}
