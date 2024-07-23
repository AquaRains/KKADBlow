use std::{mem, thread};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use windows::core::{PCWSTR, w};
use windows::Win32::Foundation::{BOOL, GetLastError, HINSTANCE, HWND, LPARAM, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW, Shell_NotifyIconW};
use windows::Win32::UI::WindowsAndMessaging::{CreatePopupMenu, CreateWindowExW, CW_USEDEFAULT, DispatchMessageW, GetMenuItemInfoW, GetMessageW, HICON, HMENU, IDI_APPLICATION, LoadIconW, MENUINFO, MENUITEMINFOW, MIM_APPLYTOSUBMENUS, MIM_STYLE, MNS_NOTIFYBYPOS, MSG, PostMessageW, RegisterClassW, SetMenuInfo, SetMenuItemInfoW, TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WM_QUIT, WM_USER, WNDCLASSW, WS_OVERLAPPEDWINDOW};
use crate::err::ApplicationError;
use crate::gui::{IconSource, w32};
use crate::gui::w32::structs::{WindowInfo, WindowsLoopData, WindowsTrayEvent};
use crate::gui::w32::{CallBackEntry, TrayItem, WININFO_STASH};
use crate::{memory_lock, zeroed};
use crate::gui::w32::string_extensions::ToPCWSTRWrapper;
use crate::gui::w32::traits::Icon;
use crate::structs::AppData;

pub(crate) fn get_win_os_error(msg: &str) -> ApplicationError {
    let win_os_error = unsafe { GetLastError().to_hresult() };

    ApplicationError::new_at(
        format!("{}: {}", &msg, win_os_error),
        file!(),
        line!(),
    )
}

pub(crate) unsafe fn run_loop() {
    let mut msg = MSG::default();
    loop {
        _ = GetMessageW(&mut msg, HWND::default(), 0, 0);
        if msg.message == WM_QUIT {
            break;
        }
        let _ = TranslateMessage(&msg);
        let _ = DispatchMessageW(&msg);
    }
}

pub(crate) unsafe fn init_window() -> Result<WindowInfo, ApplicationError> {
    let hmodule = match GetModuleHandleW(PCWSTR::null()) {
        Ok(v) => { v }
        Err(_) => { return Err(get_win_os_error("Error getting module handle")); }
    };

    let class_name = crate::CLASSNAME_APP_NAME;

    let wnd = WNDCLASSW {
        lpfnWndProc: Some(w32::extern_func::window_proc),
        lpszClassName: class_name.clone(),
        ..unsafe { mem::zeroed() }
    };

    RegisterClassW(&wnd);

    let hwnd = match CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        class_name,
        w!("rust_systray_window"),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        0,
        CW_USEDEFAULT,
        0,
        HWND::default(),
        HMENU::default(),
        HINSTANCE::default(),
        mem::zeroed(),
    ) {
        Ok(h) => { h }
        Err(_) => { return Err(get_win_os_error("Error creating window")); }
    };

    let icon: HICON = unsafe {
        let handle = match LoadIconW(GetModuleHandleW(zeroed!(PCWSTR)).unwrap(), w!("tray-default")) {
            Ok(h) => { h }
            Err(_) => {
                match LoadIconW(HINSTANCE::default(), IDI_APPLICATION) {
                    Ok(h) => { h }
                    Err(_) => {
                        return Err(get_win_os_error("Error setting icon from resource"));
                    }
                }
            }
        };
        handle
    };

    let nid = NOTIFYICONDATAW {
        cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        uFlags: NIF_MESSAGE | NIF_ICON,
        hIcon: icon,
        uCallbackMessage: WM_USER + 1,
        ..unsafe { mem::zeroed() }
    };

    if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
        return Err(get_win_os_error("Error adding menu icon"));
    }

    // Setup menu
    let info = MENUINFO {
        cbSize: mem::size_of::<MENUINFO>() as u32,
        fMask: MIM_APPLYTOSUBMENUS | MIM_STYLE,
        dwStyle: MNS_NOTIFYBYPOS,
        ..unsafe { mem::zeroed() }
    };

    let hmenu = match CreatePopupMenu() {
        Ok(h) => {
            match SetMenuInfo(h, &info) {
                Ok(..) => { h }
                Err(_) => { return Err(get_win_os_error("Error setting up menu")); }
            }
        }
        Err(_) => { return Err(get_win_os_error("Error creating popup menu")); }
    };

    Ok(WindowInfo {
        hwnd,
        hmenu,
        hmodule,
    })
}

#[allow(dead_code)]
impl TrayItem {
    pub fn new(title: &str, icon: IconSource) -> Result<Self, ApplicationError>
    {
        let entries = Arc::new(Mutex::new(Vec::new()));
        let (event_sender, event_receiver) = channel::<WindowsTrayEvent>();

        let entries_clone = Arc::clone(&entries);
        let event_loop = thread::spawn(move || loop {
            if let Ok(v) = event_receiver.recv() {
                if v.0 == u32::MAX {
                    break;
                }

                memory_lock::mutex_lock(&entries_clone, |entries: &mut Vec<CallBackEntry>| match &entries[v.0 as usize] {
                    Some(f) => f(),
                    None => {}
                })
            }
        });

        let (window_sender, window_receiver) = channel();

        let event_sender_clone = event_sender.clone();
        let windows_loop = thread::spawn(move || unsafe {
            let info = match init_window() {
                Ok(info) => {
                    window_sender.send(Ok(info.clone())).ok();
                    info
                }
                Err(e) => {
                    window_sender.send(Err(e)).ok();
                    return;
                }
            };

            WININFO_STASH.with(|stash| {
                let data = WindowsLoopData {
                    info,
                    event_sender: event_sender_clone,
                };

                *stash.borrow_mut() = Some(data);
            });

            run_loop();
        });

        let info = match window_receiver.recv().unwrap() {
            Ok(info) => info,
            Err(e) => return Err(e),
        };

        let w = Self {
            entries,
            info,
            windows_loop: Some(windows_loop),
            event_loop: Some(event_loop),
            event_sender,
            app_options: AppData::default(),
        };

        let _ = w.set_tooltip(title);
        let _ = w.set_icon(icon);

        Ok(w)
    }

    pub fn set_tooltip(&self, tooltip: &str) -> Result<(), ApplicationError> {
        let wide_tooltip = tooltip.to_vec_u16();

        if wide_tooltip.len() > 128 {
            return Err(ApplicationError::new("The tooltip may not exceed 127 wide bytes"));
        }

        let mut nid = NOTIFYICONDATAW {
            cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.info.hwnd,
            uID: 1,
            uFlags: NIF_TIP,
            ..unsafe { mem::zeroed() }
        };

        #[cfg(not(target_arch = "x86"))]
        nid.szTip[..wide_tooltip.len()].copy_from_slice(&wide_tooltip);

        unsafe {
            if !Shell_NotifyIconW(NIM_MODIFY, &nid).as_bool() {
                return Err(get_win_os_error("Error setting tooltip"));
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        unsafe {
            PostMessageW(self.info.hwnd, WM_DESTROY, WPARAM(0), LPARAM(0)).unwrap();
        }

        if let Some(t) = self.windows_loop.take() {
            t.join().ok();
        }

        if let Some(t) = self.event_loop.take() {
            self.event_sender.send(WindowsTrayEvent(u32::MAX)).ok();
            t.join().ok();
        }
    }

    pub fn shutdown(&self) -> Result<(), ApplicationError> {
        let nid = NOTIFYICONDATAW {
            cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.info.hwnd,
            uID: 1,
            uFlags: NIF_ICON,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            return if Shell_NotifyIconW(NIM_DELETE, &nid).as_bool()
            {
                Ok(())
            } else {
                Err(get_win_os_error("Error deleting icon from menu"))
            };
        }
    }

    pub(crate) fn _get_menu_item_info(&self, id: u32) -> Result<MENUITEMINFOW, ApplicationError> {
        let mut item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            return match GetMenuItemInfoW(self.info.hmenu, id, BOOL(0), &mut item) {
                Ok(_) => Ok(item),
                Err(_) => Err(get_win_os_error("Error getting menuitem info")),
            };
        }
    }
    pub(crate) fn _set_menu_item_info(&self, item: &MENUITEMINFOW) -> Result<(), ApplicationError> {
        unsafe {
            return match SetMenuItemInfoW(self.info.hmenu, item.wID, BOOL(1), item) {
                Ok(_) => Ok(()),
                Err(_) => Err(get_win_os_error("Error setting menu item")),
            };
        }
    }
}

impl Drop for TrayItem {
    fn drop(&mut self) {
        self.shutdown().ok();
        self.quit();
    }
}