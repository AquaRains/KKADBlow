mod gui;
mod err;
mod memory_lock;
mod structs;
mod funcs;

use std::sync::mpsc::Sender;
use crate::gui::{TrayApplication, IconSource};
use crate::gui::w32::traits::{Label, Menu, Separator};
use crate::structs::{AppData, ElapsedHandler, Message, TimerTrait};
pub use crate::funcs::options_from_console_args;

use windows::core::{PCWSTR, w};
use crate::gui::w32::find_area_and_shrink;

pub(crate) const CLASSNAME_APP_NAME: PCWSTR = w!("kkadblow_rs");

pub fn app_init(option: Option<AppData>) -> Result<(), err::ApplicationError> {
    let mut option = match option {
        None => { AppData::default() }
        Some(o) => { o }
    };

    let mut trayapp = TrayApplication::new(
        "test",
        IconSource::Resource("IDI_ICON1"),
    ).unwrap();

    let (sender, receiver) = std::sync::mpsc::channel();

    trayapp.set_options(option.clone());
    trayapp.add_label("label").unwrap();

    let work_sender = sender.clone();

    trayapp.add_menu_item("Find Window", move || { work_sender.send(Message::FeatureStart).unwrap(); }).unwrap();
    trayapp.add_separator().unwrap();

    let quit_sender = sender.clone();
    trayapp.add_menu_item("Quit", move || { quit_sender.send(Message::Quit).unwrap(); }).unwrap();

    let timer_sender = sender.clone();

    let timer = structs::Timer {
        interval: 0,
        repeat: true,
    };

    timer.start(ElapsedHandler::Func(move || {
        timer_sender.send(Message::TimerElapsed).unwrap();
    }));

    loop {
        match receiver.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::TimerElapsed) => {
               // println!("TimerElapsed");
                do_main_feature(option, sender.clone());
            }
            Ok(Message::TimerStop) => {
                println!("TimerStop");
                option.repeat = false;
            }
            Ok(Message::FeatureStart) => {
                do_main_feature(option, sender.clone());
            }
            _ => {}
        }
    }
    return Ok(());
}

fn do_main_feature(option: AppData, sender: Sender<Message>) {
    find_area_and_shrink(Some(sender.clone()), option).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
    });
}
