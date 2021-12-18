use core::time;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::thread::sleep;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui;

use crate::comet;

pub enum Event {
    End,
    Proceed,
    Back,
}
pub struct Tui<'comet> {
    comet: &'comet Rc<RefCell<comet::Comet>>,
    pub terminal:
        tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
}

pub fn new(comet: &Rc<RefCell<comet::Comet>>) -> Result<Tui, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    terminal.clear()?;
    Ok(Tui { comet, terminal })
}

impl<'comet> Tui<'comet> {
    pub fn draw_blocks(&mut self) -> Result<(), io::Error> {
        let comet = self.comet.clone();
        let comet = comet.as_ref().borrow();
        self.terminal.draw(|f| {
            let chunks = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Horizontal)
                .vertical_margin(2)
                .constraints(
                    [
                        tui::layout::Constraint::Percentage(45),
                        tui::layout::Constraint::Percentage(55),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let chunks2 = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints(
                    [
                        tui::layout::Constraint::Min(10),
                        tui::layout::Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);

            let chunks3 = tui::layout::Layout::default()
                .direction(tui::layout::Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        tui::layout::Constraint::Percentage(25),
                        tui::layout::Constraint::Percentage(25),
                        tui::layout::Constraint::Percentage(25),
                        tui::layout::Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            let items = [
                tui::widgets::ListItem::new(format!("PR={:X}", comet.get_pr())),
                tui::widgets::ListItem::new(format!("SP={:X}", comet.get_sp())),
                tui::widgets::ListItem::new(format!(
                    "FR={:X}{:X}{:X}",
                    if *comet.get_fr(0) { 1 } else { 0 },
                    if *comet.get_fr(1) { 1 } else { 0 },
                    if *comet.get_fr(2) { 1 } else { 0 }
                )),
                tui::widgets::ListItem::new(format!(
                    "IR={:X} {:X}",
                    comet.get_ir(0),
                    comet.get_ir(1)
                )),
                tui::widgets::ListItem::new(format!("GR0={:X}", comet.get_gr(0))),
                tui::widgets::ListItem::new(format!("GR1={:X}", comet.get_gr(1))),
                tui::widgets::ListItem::new(format!("GR2={:X}", comet.get_gr(2))),
                tui::widgets::ListItem::new(format!("GR3={:X}", comet.get_gr(3))),
                tui::widgets::ListItem::new(format!("GR4={:X}", comet.get_gr(4))),
                tui::widgets::ListItem::new(format!("GR5={:X}", comet.get_gr(5))),
                tui::widgets::ListItem::new(format!("GR6={:X}", comet.get_gr(6))),
                tui::widgets::ListItem::new(format!("GR7={:X}", comet.get_gr(7))),
            ];
            let list = tui::widgets::List::new(items).block(
                tui::widgets::Block::default()
                    .title("Register")
                    .borders(tui::widgets::Borders::ALL),
            );
            let mem_wrap = tui::widgets::Block::default()
                .title("Main Memory")
                .borders(tui::widgets::Borders::ALL);

            let op = tui::widgets::Paragraph::new("Operation=")
                .block(
                    tui::widgets::Block::default()
                        .title("Operation")
                        .borders(tui::widgets::Borders::ALL),
                )
                .wrap(tui::widgets::Wrap { trim: true });

            let mm1 = tui::widgets::List::new(get_mm_items(&comet, 0, 15))
                .block(tui::widgets::Block::default());
            let mm2 = tui::widgets::List::new(get_mm_items(&comet, 16, 31))
                .block(tui::widgets::Block::default());
            let mm3 = tui::widgets::List::new(get_mm_items(&comet, 32, 47))
                .block(tui::widgets::Block::default());
            let mm4 = tui::widgets::List::new(get_mm_items(&comet, 48, 63))
                .block(tui::widgets::Block::default());
            f.render_widget(list, chunks2[0]);
            f.render_widget(op, chunks2[1]);
            f.render_widget(mem_wrap, chunks[1]);
            f.render_widget(mm1, chunks3[0]);
            f.render_widget(mm2, chunks3[1]);
            f.render_widget(mm3, chunks3[2]);
            f.render_widget(mm4, chunks3[3]);
        })?;
        Ok(())
    }
}

fn get_mm_items<'a>(
    comet: &'a comet::Comet,
    from: i32,
    to: i32,
) -> Vec<tui::widgets::ListItem<'a>> {
    (from..=to)
        .map(|i| {
            tui::widgets::ListItem::new(format!(
                "{:04X} {:04X}",
                i,
                comet.main_memory.read(i as i16).unwrap()
            ))
        })
        .collect::<Vec<tui::widgets::ListItem>>()
}

/// 実行中のスレッドをブロックする
pub fn read_event(reader: &mut termion::AsyncReader) -> Event {
    let e: Event;
    loop {
        let input = reader.keys().next();
        if let Some(Ok(key)) = input {
            if let termion::event::Key::Ctrl('c') = key {
                e = Event::End;
                break;
            }
            if let termion::event::Key::Char('n') = key {
                e = Event::Proceed;
                break;
            }
            if let termion::event::Key::Char('b') = key {
                e = Event::Back;
                break;
            }
        }
        sleep(time::Duration::from_millis(50));
    }
    e
}
