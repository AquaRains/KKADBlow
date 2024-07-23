use std::cmp::PartialEq;
use windows::Win32::Foundation::RECT;


#[allow(dead_code)]
pub(crate) enum Message {
    Quit,
    TimerElapsed,
    TimerStop,
    FeatureStart,
}

#[derive(Clone, Copy)]
pub struct AppData
{
    pub target_window: WindowInfo,
    pub target_child_window: WindowInfo,
    pub target_ad_area: WindowInfo,
    pub target_ad_area_child: WindowInfo,
    pub target_expandable_area: WindowInfo,
    pub target_shrinkable_area: WindowInfo,
    pub repeat_interval_second: u64,
    pub repeat: bool,
}

#[derive(Clone, Copy)]
pub struct WindowInfo {
    pub process_id: Option<u32>,
    pub handle: Option<windows::Win32::Foundation::HWND>,
    pub class_name: Option<&'static str>,
    pub window_name: Option<&'static str>,
    pub window_area: Option<RECT>,
}


impl Default for WindowInfo {
    fn default() -> Self {
        WindowInfo {
            class_name: None,
            window_name: None,
            handle: None,
            process_id: None,
            window_area: None,
        }
    }
}

impl WindowInfo {
    pub fn deconstruct(&self) -> (&'static str, &'static str, Option<windows::Win32::Foundation::HWND>) {
        (self.class_name.unwrap_or_else(|| { "" }), self.window_name.unwrap_or_else(|| { "" }), self.handle)
    }
}

impl AppData {
    pub(crate) const DEFAULT_INTERVAL_SECOND: u64 = 1;
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            target_window: WindowInfo {
                window_name: Some("카카오톡"),
                class_name: Some("EVA_Window_Dblclk"),
                ..Default::default()
            },
            target_child_window: WindowInfo
            {
                window_name: Some("OnlineMainView"),
                class_name: Some("EVA_Window_Dblclk"),
                ..Default::default()
            },
            target_ad_area: WindowInfo {
                class_name: Some("EVA_Window_Dblclk"),
                window_name: Some(""),
                ..Default::default()
            },
            target_ad_area_child: WindowInfo {
                class_name: Some("BannerAdContainer"),
                ..Default::default()
            },
            target_expandable_area: WindowInfo {
                class_name: Some("EVA_ChildWindow"),
                window_name: Some("OnlineMainView"),
                ..Default::default()
            },
            target_shrinkable_area: WindowInfo {
                class_name: Some("EVA_ChildWindow"),
                window_name: Some(""),
                ..Default::default()
            },
            repeat_interval_second: Self::DEFAULT_INTERVAL_SECOND,
            repeat: true,
        }
    }
}

#[allow(dead_code)]
pub struct ArgInfo {
    pub option: String,
    pub value: String,
    pub path: Option<String>,
}

#[allow(dead_code)]
impl ArgInfo {
    pub fn new(option: String, value: String, path: Option<String>) -> Self {
        ArgInfo {
            option,
            value,
            path,
        }
    }
}

pub struct Timer {
    pub interval: u64,
    pub repeat: bool,
}

#[allow(dead_code)]
pub trait TimerTrait {
    fn start<F>(&self, elapsed: ElapsedHandler<F>)
    where
        F: 'static + FnMut() + Send;
    fn stop(&mut self);
}
impl Default for Timer {
    fn default() -> Self {
        Timer {
            interval: 5,
            repeat: false,
        }
    }
}

#[allow(dead_code)]
pub enum ElapsedHandler<F>
where
    F: 'static + FnMut() + Send,
{
    Func(F),
    Nil,
}

impl<F> PartialEq for ElapsedHandler<F>
where
    F: 'static + FnMut() + Send,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ElapsedHandler::Nil, ElapsedHandler::Nil) => true,
            _ => false,
        }
    }
}

impl TimerTrait for Timer {
    fn start<F>(&self, elapsed: ElapsedHandler<F>)
    where
        F: 'static + FnMut() + Send,
    {
        let repeat = self.repeat;
        let interval = self.interval;

        let mut elapsed_func = match elapsed {
            ElapsedHandler::Func(f) => f,
            ElapsedHandler::Nil => return,
        };

        std::thread::spawn(move || loop {
            if interval > 0 { std::thread::sleep(std::time::Duration::from_secs(interval)); }
            else { std::thread::sleep(std::time::Duration::from_millis(20)); }

            elapsed_func();

            if !repeat { break; }
        });
    }

    fn stop(&mut self) {
        self.repeat = false;
    }
}