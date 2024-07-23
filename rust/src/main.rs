#![windows_subsystem = "windows"]
use kkadblow_rs::{app_init, options_from_console_args};

fn main() -> Result<(), ()>{
    let args = std::env::args().collect();
    let option = options_from_console_args(args);

    match app_init(option) {
        Ok(_) => {
            println!("App Quitted successfully");
            return Ok(());
        }
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(());
        }
    }
}
