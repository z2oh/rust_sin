extern crate termion;

use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;

fn main() {
    // Initialize.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    write!(stdout, "{}", termion::clear::All,).unwrap();

    stdout.flush().unwrap();

    let cell_width = std::f64::consts::PI / 20.0;
    let cell_height = 1.0 / 12.0;

    for x in 0..80 {
        let adj_x = (x as f64) * cell_width;
        let adj_y = adj_x.sin();
        let y = ((adj_y / cell_height) + (1.0 / cell_height)).round() as u16;
        write!(stdout, "{}+", termion::cursor::Goto(x + 1, y + 1)).unwrap();
    }

    stdout.flush().unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            // Quit.
            b'q' => {
                write!(
                    stdout,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                ).unwrap();
                stdout.flush().unwrap();
                return;
            }
            _ => continue,
        };
    }
}
