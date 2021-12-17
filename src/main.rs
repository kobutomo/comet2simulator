use comet2simulator::comet;
use comet2simulator::tui;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

fn main() -> Result<(), io::Error> {
    let comet = Rc::new(RefCell::new(comet::Comet::new()));
    let mut tui = tui::new(&comet)?;
    let stdin = &mut termion::async_stdin();
    loop {
        tui.draw_blocks()?;
        match tui::read_event(stdin) {
            tui::Event::Proceed => comet.clone().borrow_mut().proceed(),
            tui::Event::Back => (),
            tui::Event::End => break,
        }
    }
    tui.terminal.clear()?;
    Ok(())
}
