use std::borrow::BorrowMut;
use std::io;
use std::io::Stdout;

use termion::raw::{IntoRawMode, RawTerminal};
use tui::{Frame, Terminal};
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Gauge, List, ListItem, ListState};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App {
    logs: StatefulList<String>,
    pub total: usize,
    pub finish: usize,
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
}

impl App {
    pub fn new(total: usize) -> Result<Self, io::Error> {
        let emtpy_logs = Vec::<String>::new();

        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("xycommand")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        Ok(App {
            logs: StatefulList::with_items(emtpy_logs),
            total,
            finish: 0,
            terminal,
        })
    }
}


impl App {
    pub fn display(&mut self) -> Result<(), io::Error> {
        let frame = self.terminal.borrow_mut();
        frame.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("xycommand")
                .borders(Borders::ALL);
            f.render_widget(block, size);
            Self::draw_progress(f, self.finish as f64 / self.total as f64);
            Self::draw_logs(f, &mut self.logs)
        })?;

        Ok(())
    }

    pub fn append_log(&mut self, log: String) {
        let logs = &mut self.logs;
        let logs = &mut logs.items;
        logs.push(log)
    }

    fn draw_progress<B: Backend>(f: &mut Frame<B>, ratio: f64) {
        let chunks = Self::spited_area(f);
        let gauge = Gauge::default()
            .block(Block::default().title("Progress:").borders(Borders::ALL))
            .gauge_style(
                Style::default()
                    .fg(Color::Magenta)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .ratio(ratio);

        f.render_widget(gauge, chunks[0]);
    }

    fn draw_logs<B: Backend>(f: &mut Frame<B>, logs: &mut StatefulList<String>) {
        let chunks = Self::spited_area(f);
        let items: Vec<ListItem> = logs
            .items
            .iter()
            .map(|i| {
                let spans = Spans::from(Span::styled(
                    i.as_str(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ));

                ListItem::new(spans).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Logs"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .start_corner(Corner::BottomLeft);

        f.render_stateful_widget(list, chunks[1], &mut logs.state);
    }

    fn spited_area<B: Backend>(f: &mut Frame<B>) -> Vec<tui::layout::Rect> {
        let rect = f.size();
        return Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20)
                ]
                    .as_ref(),
            )
            .split(rect);
    }
}
