use std::sync::mpsc::Sender;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::HMENU;

#[derive(Clone)]
pub(crate) struct WindowInfo {
    pub hwnd: HWND,
    pub hmodule: HMODULE,
    pub hmenu: HMENU,
}

unsafe impl Send for WindowInfo {}
unsafe impl Sync for WindowInfo {}

#[derive(Clone)]
pub(crate) struct WindowsLoopData {
    pub info: WindowInfo,
    pub event_sender: Sender<WindowsTrayEvent>,
}

pub(crate) struct WindowsTrayEvent(pub(crate) u32);
