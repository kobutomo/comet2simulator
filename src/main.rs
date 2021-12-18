use comet2simulator::comet;
use comet2simulator::tui;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::rc::Rc;

fn main() -> Result<(), io::Error> {
    let comet = Rc::new(RefCell::new(comet::Comet::new()));
    let mut tui = tui::new(&comet)?;
    let stdin = &mut termion::async_stdin();
    // ファイル読み込み
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid args");
    }
    let mut file = File::open(&args[1]).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // 初期化
    comet.borrow_mut().init(contents);
    // メインループ
    loop {
        tui.draw_blocks()?;
        match tui::read_event(stdin) {
            tui::Event::Proceed => comet.clone().borrow_mut().execute_step(),
            tui::Event::Back => (),
            tui::Event::End => break,
        }
    }
    tui.terminal.clear()?;
    Ok(())
}

