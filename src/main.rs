mod comet;
mod memory;

use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let comet = comet::Comet::new();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(2)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)].as_ref())
            .split(f.size());

        let chunks2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)].as_ref())
            .split(chunks[0]);

        let chunks3 = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let items = [
            ListItem::new(format!("PR={:X}", *comet.get_pr())),
            ListItem::new(format!("SP={:X}", *comet.get_sp())),
            ListItem::new(format!(
                "FR={:X}{:X}{:X}",
                if *comet.get_fr(0) { 1 } else { 0 },
                if *comet.get_fr(1) { 1 } else { 0 },
                if *comet.get_fr(2) { 1 } else { 0 }
            )),
            ListItem::new(format!("IR={:X} {:X}", *comet.get_ir(0), *comet.get_ir(1))),
            ListItem::new(format!("GR0={:X}", *comet.get_gr(0))),
            ListItem::new(format!("GR1={:X}", *comet.get_gr(1))),
            ListItem::new(format!("GR2={:X}", *comet.get_gr(2))),
            ListItem::new(format!("GR3={:X}", *comet.get_gr(3))),
            ListItem::new(format!("GR4={:X}", *comet.get_gr(4))),
            ListItem::new(format!("GR5={:X}", *comet.get_gr(5))),
            ListItem::new(format!("GR6={:X}", *comet.get_gr(6))),
            ListItem::new(format!("GR7={:X}", *comet.get_gr(7))),
        ];
        let list = List::new(items).block(Block::default().title("Register").borders(Borders::ALL));
        let mem_wrap = Block::default().title("Main Memory").borders(Borders::ALL);

        let op = Paragraph::new("Operation=")
            .block(Block::default().title("Operation").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        let mm1 = List::new(get_mm_items(&comet.main_memory, 0, 15)).block(Block::default());
        let mm2 = List::new(get_mm_items(&comet.main_memory, 16, 31)).block(Block::default());
        let mm3 = List::new(get_mm_items(&comet.main_memory, 32, 47)).block(Block::default());
        let mm4 = List::new(get_mm_items(&comet.main_memory, 48, 63)).block(Block::default());
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

fn get_mm_items<'a>(main_memory: &'a memory::Memory, from: i32, to: i32) -> Vec<ListItem<'a>> {
    (from..=to)
        .map(|i| {
            ListItem::new(format!(
                "{:04X} {:04X}",
                i,
                *main_memory.read(i as i16).unwrap()
            ))
        })
        .collect::<Vec<ListItem>>()
}
