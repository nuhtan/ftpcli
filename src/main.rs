mod shared_state;
mod ui;

use std::io::Error;

fn main() -> Result<(), Error> {
    // shared_state::SharedState::new(String::from("nuhtan"), String::from("test"), [192, 168, 1, 7].into(), 22);
    ui::start_up();
    Ok(())
}
