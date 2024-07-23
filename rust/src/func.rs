
use crate::structs::{AppData};

pub fn options_from_console_args(args: Vec<String>) -> Option<AppData> {
    let mut repeat_interval_second :Option<u64> = None;
    let mut repeat = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-i" => {
                repeat_interval_second = match args[i + 1].parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                i += 2;
            }
            "-r" => {
                repeat = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    let mut o = AppData::default();
    o.repeat_interval_second = repeat_interval_second.unwrap_or(AppData::DEFAULT_INTERVAL_SECOND);
    o.repeat = repeat;
    return Some(o);
}


