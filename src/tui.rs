use crate::{db::Note, utils};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};
use std::{
    io,
    time::{Duration, Instant},
};

pub struct NotesTable {
    state: TableState,
    notes: Vec<Note>,
}

impl NotesTable {
    pub fn new(notes: Vec<Note>) -> Self {
        let mut state = TableState::default();
        if !notes.is_empty() {
            state.select(Some(0));
        }
        Self { state, notes }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.notes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.notes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_note(&self) -> Option<&Note> {
        if let Some(i) = self.state.selected() {
            self.notes.get(i)
        } else {
            None
        }
    }
}

pub fn run_notes_ui(notes: Vec<Note>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut table = NotesTable::new(notes);
    let res = run_app(&mut terminal, &mut table);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(Some(note_path)) => {
            utils::open_in_editor(&note_path)?;
        }
        Ok(None) => {
            // User quit without selecting
        }
        Err(err) => {
            println!("{err:?}");
        }
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    table: &mut NotesTable,
) -> Result<Option<String>> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, table))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Down | KeyCode::Char('j') => table.next(),
                    KeyCode::Up | KeyCode::Char('k') => table.previous(),
                    KeyCode::Enter => {
                        if let Some(note) = table.selected_note() {
                            return Ok(Some(note.path.clone()));
                        }
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, table: &mut NotesTable) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let selected_style = Style::default()
        .bg(Color::White)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);
    let normal_style = Style::default()
        .bg(Color::Black)
        .fg(Color::White);
    let header_cells = ["ID", "Title", "Created At", "Slug"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = table.notes.iter().map(|note| {
        let height = 1;
        let cells = vec![
            Cell::from(note.id.to_string()),
            Cell::from(note.title.clone()),
            Cell::from(note.created_at.clone()),
            Cell::from(note.slug.clone()),
        ];
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let table_widget = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Min(20),
            Constraint::Length(20),
            Constraint::Min(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Notes (q to quit, ↑/↓ or j/k to navigate, Enter to select)"),
    )
    .row_highlight_style(selected_style)
    .highlight_symbol(">> ");

    f.render_stateful_widget(table_widget, rects[0], &mut table.state);
}

