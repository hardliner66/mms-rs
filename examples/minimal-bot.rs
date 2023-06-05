use std::io::{stdin, stdout};

use mms_rs::MmsApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Running...");
    let mut api = MmsApi::new(stdin().lock(), stdout().lock());

    api.set_color(0, 0, &mms_rs::CellColor::DarkGreen)?;
    api.set_text(0, 0, "abc")?;
    loop {
        if !api.wall_left()? {
            api.turn_left()?;
        }
        while api.wall_front()? {
            api.turn_right()?;
        }
        _ = api.move_forward(None);
    }
}
