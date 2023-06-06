# mms-rs

A simple library to use rust for [mms](https://github.com/mackorone/mms).

## Using the example

- Check out the code and add a new mouse to mms.
- Point the `Directory` to the code you checked out.
- As `Build Command` use: `cargo build --release --example minimal-mouse`
- As `Run Command` use: `./target/release/examples/minimal-mouse`

After that you can run the example from within mms.

## Building your own

Add `mms-rs` to your dependencies with `cargo add mms-rs`. Then you can use the api in your own code.

## Example code
```rs
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
        Mouse::move_forward(None);
    }
}
```
