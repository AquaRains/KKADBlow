use std::{mem};
use std::os::raw::c_void;
use windows::core::{PCWSTR, w};

use windows::Win32::Foundation::{BOOL, FALSE, HINSTANCE, HWND, LPARAM, LRESULT, POINT, TRUE, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{Shell_NotifyIconW, NOTIFYICONDATAW, NIF_ICON, NIF_MESSAGE, NIM_ADD};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, GetCursorPos, GetMenuItemID, LoadIconW, PostQuitMessage, RegisterWindowMessageW, SetForegroundWindow, TrackPopupMenu, HICON, IDI_APPLICATION, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_LEFTBUTTON, WM_CREATE, WM_DESTROY, WM_LBUTTONUP, WM_MENUCOMMAND, WM_RBUTTONUP, WM_USER, EnumChildWindows, EnumWindows};
use crate::gui::w32::{structs::WindowsTrayEvent, WININFO_STASH};

pub(crate) unsafe extern "system" fn window_proc(h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    static mut U_TASKBAR_RESTART: u32 = 0;

    if msg == WM_MENUCOMMAND {
        WININFO_STASH.with(|stash| {
            let stash = stash.borrow();
            let stash = stash.as_ref();
            if let Some(stash) = stash {
                let menu_id = GetMenuItemID(stash.info.hmenu, w_param.0 as i32) as i32;
                if menu_id != -1 {
                    stash.event_sender.send(WindowsTrayEvent(menu_id as u32)).ok();
                }
            }
        });
    }

    if msg == WM_USER + 1 && (l_param.0 as u32 == WM_LBUTTONUP || l_param.0 as u32 == WM_RBUTTONUP) {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point) != Ok(()) {
            return LRESULT::default();
        }

        let _ = SetForegroundWindow(h_wnd);

        WININFO_STASH.with(|stash| {
            let stash = stash.borrow();
            let stash = stash.as_ref();
            if let Some(stash) = stash {
                let _ = TrackPopupMenu(
                    stash.info.hmenu,
                    TPM_LEFTBUTTON | TPM_BOTTOMALIGN | TPM_LEFTALIGN,
                    point.x,
                    point.y,
                    0,
                    h_wnd,
                    None,
                );
            }
        });
    }

    if msg == WM_CREATE {
        U_TASKBAR_RESTART = RegisterWindowMessageW(w!("TaskbarCreated"));
    }

    if msg == U_TASKBAR_RESTART {
        let icon: HICON = unsafe {
            match LoadIconW(HINSTANCE::from(GetModuleHandleW(PCWSTR::null()).unwrap()), w!("tray-default")) {
                Ok(v) => { v }
                Err(_) => {
                    match LoadIconW(HINSTANCE::default(), IDI_APPLICATION) {
                        Ok(v) => { v }
                        Err(_) => {
                            println!("Error setting icon from resource");
                            PostQuitMessage(0);
                            return LRESULT::default();
                        }
                    }
                }
            }
        };
        let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
        nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = h_wnd;
        nid.uID = 1;
        nid.uFlags = NIF_MESSAGE | NIF_ICON;
        nid.hIcon = icon;
        nid.uCallbackMessage = WM_USER + 1;
        if !Shell_NotifyIconW(NIM_ADD, &nid).as_bool() {
            println!("Error adding menu icon");
            PostQuitMessage(0);
        }
    }

    if msg == WM_DESTROY {
        PostQuitMessage(0);
    }

    DefWindowProcW(h_wnd, msg, w_param, l_param)
}


pub fn enumerate_windows<F>(mut callback: F)
where
    F: FnMut(HWND) -> bool,
{
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = LPARAM(closure_pointer_pointer as isize);
    let _ = unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

pub fn enumerate_child_windows<F>(hwnd: HWND, mut callback: F)
where
    F: FnMut(HWND) -> bool,
{
    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = LPARAM(closure_pointer_pointer as isize);
    let _ = unsafe { EnumChildWindows(hwnd, Some(enumerate_callback), lparam) };
}

unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam.0 as *mut c_void);
    if closure(hwnd) { TRUE } else { FALSE }
}
