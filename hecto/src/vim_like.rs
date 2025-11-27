use crossterm::cursor::MoveTo;
use crossterm::{
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::io::stdout;

pub fn draw_rows() {
    enable_raw_mode_panic();
    match size() {
        Ok((_, height)) => {
            for _curr_height in 0..height {
                println!("~\r");
            }

            let _ = stdout().execute(MoveTo(0, 0));
        }
        _ => {}
    }
    disable_raw_mode_panic();
}

fn enable_raw_mode_panic() {
    if let Err(err) = enable_raw_mode() {
        panic!("error!:{err:#?}")
    }
}

fn disable_raw_mode_panic() {
    if let Err(err) = disable_raw_mode() {
        panic!("error!:{err:#?}")
    }
}
