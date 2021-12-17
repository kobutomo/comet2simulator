use comet2simulator::comet;
use comet2simulator::tui;
use std::io;

fn main() -> Result<(), io::Error> {
    let comet = comet::Comet::new();
    let mut tui = tui::new(&comet)?;
    tui.draw_blocks()?;
    Ok(())
}
