use mms_rs::MmsApi as Mouse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Running...");
    Mouse::set_color(0, 0, &mms_rs::CellColor::DarkGreen)?;
    Mouse::set_text(0, 0, "abc")?;
    loop {
        if !Mouse::wall_left()? {
            Mouse::turn_left()?;
        }
        while Mouse::wall_front()? {
            Mouse::turn_right()?;
        }
        Mouse::move_forward(None)?;
    }
}
