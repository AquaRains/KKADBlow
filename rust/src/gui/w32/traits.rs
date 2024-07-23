use std::mem;
use std::ops::BitOrAssign;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{RDW_ALLCHILDREN, RDW_ERASE, RDW_ERASENOW, RDW_INVALIDATE, RedrawWindow};
use windows::Win32::UI::Shell::{NIF_ICON, NIM_MODIFY, NOTIFYICONDATAW, Shell_NotifyIconW};
use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, EnumWindows, FindWindowExW, FindWindowW, GetClassNameW, GetParent, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId, HICON, HWND_BOTTOM, IMAGE_ICON, InsertMenuItemW, LoadImageW, LR_DEFAULTCOLOR, MENUITEMINFOW, MFS_CHECKED, MFS_DISABLED, MFS_UNCHECKED, MFS_UNHILITE, MFT_SEPARATOR, MFT_STRING, MIIM_CHECKMARKS, MIIM_FTYPE, MIIM_ID, MIIM_STATE, MIIM_STRING, SetMenuItemInfoW, SetWindowPos, ShowWindow, SW_HIDE, SWP_ASYNCWINDOWPOS, SWP_HIDEWINDOW, SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOMOVE, SWP_NOREDRAW, SWP_NOSENDCHANGING, WNDENUMPROC};
use crate::err::ApplicationError;
use crate::gui::IconSource;
use crate::gui::w32::string_extensions::{ToPCWSTRWrapper, ToPWSTRWrapper};
use crate::gui::w32::TrayItem;
use crate::gui::w32::ui::get_win_os_error;
use crate::memory_lock;
use crate::structs::{AppData};

#[allow(dead_code)]
pub trait Label {
    fn add_label(&mut self, label: &str) -> Result<(), ApplicationError>;
    fn add_label_with_id(&mut self, label: &str) -> Result<u32, ApplicationError>;
    fn set_label(&mut self, label: &str, id: u32) -> Result<(), ApplicationError>;
}

impl Label for TrayItem {
    fn add_label(&mut self, label: &str) -> Result<(), ApplicationError> {
        self.add_label_with_id(label)?;
        Ok(())
    }

    fn add_label_with_id(&mut self, label: &str) -> Result<u32, ApplicationError> {
        let item_idx = memory_lock::mutex_lock(&self.entries, |entries| {
            let len = entries.len();
            entries.push(None);
            len
        }) as u32;

        let st = label.to_pwstr();

        let item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE,
            fType: MFT_STRING,
            fState: MFS_DISABLED | MFS_UNHILITE,
            wID: item_idx,
            dwTypeData: st,
            cch: (label.len() * 2) as u32,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            match InsertMenuItemW(self.info.hmenu, item_idx, BOOL(1), &item)
            {
                Ok(_) => Ok(item_idx),
                Err(_) => Err(get_win_os_error("Error inserting menu item")),
            }
        }
    }

    fn set_label(&mut self, label: &str, id: u32) -> Result<(), ApplicationError> {
        let item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE,
            fType: MFT_STRING,
            fState: MFS_DISABLED | MFS_UNHILITE,
            wID: id,
            dwTypeData: label.to_pwstr(),
            cch: (label.len() * 2) as u32,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            match SetMenuItemInfoW(self.info.hmenu, id, BOOL(1), &item) {
                Ok(r) => { Ok(r) }
                Err(_) => { return Err(get_win_os_error("Error setting menu item")); }
            }
        }
    }
}

#[allow(dead_code)]
pub trait Icon {
    fn set_icon_from_resource(&self, resource_name: &str) -> Result<(), ApplicationError>;
    fn set_icon(&self, icon: IconSource) -> Result<(), ApplicationError>;
    fn set_icon_by_iconhandle(&self, icon: HICON) -> Result<(), ApplicationError>;
}

impl Icon for TrayItem {
    fn set_icon_from_resource(&self, resource_name: &str) -> Result<(), ApplicationError> {
        let icon = unsafe {
            let handle = LoadImageW(
                self.info.hmodule,
                resource_name.to_pcwstr(),
                IMAGE_ICON,
                64,
                64,
                LR_DEFAULTCOLOR,
            );

            match handle {
                Ok(h) => { HICON(h.0) }
                Err(_) => {
                    return Err(get_win_os_error("Error setting icon from resource"));
                }
            }
        };

        self.set_icon_by_iconhandle(icon)
    }

    fn set_icon(&self, icon: IconSource) -> Result<(), ApplicationError> {
        match icon {
            IconSource::Resource(icon_str) => return self.set_icon_from_resource(icon_str),
            IconSource::RawIcon(raw_icon) => self.set_icon_by_iconhandle(raw_icon),
        }
    }

    fn set_icon_by_iconhandle(&self, icon: HICON) -> Result<(), ApplicationError> {
        let nid = NOTIFYICONDATAW {
            cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.info.hwnd,
            uID: 1,
            uFlags: NIF_ICON,
            hIcon: icon,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            if !Shell_NotifyIconW(NIM_MODIFY, &nid).as_bool() {
                return Err(get_win_os_error("Error setting icon"));
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub trait Menu {
    fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), ApplicationError>
    where
        F: Fn() + Send + 'static;

    fn add_menu_item_with_id<F>(&mut self, label: &str, cb: F) -> Result<u32, ApplicationError>
    where
        F: Fn() + Send + 'static;

    fn set_menu_item_label(&mut self, label: &str, id: u32) -> Result<(), ApplicationError>;

    fn set_menu_item_chackable(&mut self, id: u32, checkable: bool) -> Result<(), ApplicationError>;

    fn set_menu_item_checked_by_id(&mut self, id: u32, checked: bool) -> Result<(), ApplicationError>;
}

impl Menu for TrayItem {
    fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), ApplicationError>
    where
        F: Fn() + Send + 'static,
    {
        self.add_menu_item_with_id(label, cb)?;
        Ok(())
    }
    fn add_menu_item_with_id<F>(&mut self, label: &str, cb: F) -> Result<u32, ApplicationError>
    where
        F: Fn() + Send + 'static,
    {
        let item_idx = memory_lock::mutex_lock(&self.entries, |entries| {
            let len = entries.len();
            entries.push(Some(Box::new(cb)));
            len
        }) as u32;

        let st = label.to_pwstr();
        let item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE,
            fType: MFT_STRING,
            wID: item_idx,
            dwTypeData: st,
            cch: (label.len() * 2) as u32,
            ..unsafe { mem::zeroed() }
        };

        unsafe
        {
            return match InsertMenuItemW(self.info.hmenu, item_idx, BOOL(1), &item) {
                Ok(_) => Ok(item_idx),
                Err(_) => Err(get_win_os_error("Error inserting menu item")),
            };
        }
    }

    fn set_menu_item_label(&mut self, label: &str, id: u32) -> Result<(), ApplicationError> {
        let st = label.to_pwstr();

        let item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE,
            fType: MFT_STRING,
            wID: id,
            dwTypeData: st,
            cch: (label.len() * 2) as u32,
            ..unsafe { mem::zeroed() }
        };

        return self._set_menu_item_info(&item);
    }

    fn set_menu_item_chackable(&mut self, id: u32, checkable: bool) -> Result<(), ApplicationError>
    {
        let mut item = self._get_menu_item_info(id)?;

        item.fMask.bitor_assign(MIIM_CHECKMARKS);
        item.fState.bitor_assign(if checkable { MFS_CHECKED } else { MFS_UNCHECKED });

        return self._set_menu_item_info(&item);
    }

    fn set_menu_item_checked_by_id(&mut self, id: u32, checked: bool) -> Result<(), ApplicationError>
    {
        let mut item = self._get_menu_item_info(id)?;

        if !item.fMask.contains(MIIM_CHECKMARKS) { return Err(ApplicationError::new("Menu item is not checkable")); }
        item.fState.bitor_assign(if checked { MFS_CHECKED } else { MFS_UNCHECKED });

        return self._set_menu_item_info(&item);
    }
}

#[allow(dead_code)]
pub trait Separator {
    fn add_separator(&mut self) -> Result<(), ApplicationError>;
    fn add_separator_with_id(&mut self) -> Result<u32, ApplicationError>;
}

impl Separator for TrayItem {
    fn add_separator(&mut self) -> Result<(), ApplicationError> {
        self.add_separator_with_id()?;
        Ok(())
    }

    fn add_separator_with_id(&mut self) -> Result<u32, ApplicationError> {
        let item_idx = memory_lock::mutex_lock(&self.entries, |entries| {
            let len = entries.len();
            entries.push(None);
            len
        }) as u32;

        let item = MENUITEMINFOW {
            cbSize: mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_FTYPE | MIIM_ID | MIIM_STATE,
            fType: MFT_SEPARATOR,
            wID: item_idx,
            ..unsafe { mem::zeroed() }
        };

        unsafe {
            return match InsertMenuItemW(self.info.hmenu, item_idx, BOOL(1), &item) {
                Ok(_) => Ok(item_idx),
                Err(_) => Err(get_win_os_error("Error inserting menu separator")),
            };
        }
    }
}

#[allow(dead_code)]
pub trait MainFeature {
    fn find_window(window_class: Option<&str>, window_name: Option<&str>) -> Result<HWND, ApplicationError>;
    fn find_window_ex(handle_parent: HWND, handle_child: HWND, window_class: Option<&str>, window_name: Option<&str>) -> HWND;
    fn find_child_window_enum(handle_parent: HWND, proc: WNDENUMPROC, option: AppData) -> bool;
    fn find_window_enum(proc: WNDENUMPROC, option: AppData);
    fn hide_item(handle_area: HWND) -> Result<(), ApplicationError>;
    fn get_area_rect(handle_area: HWND) -> Result<RECT, ApplicationError>;
    fn set_windows_position(handle_area: HWND, insert_after: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<(), ApplicationError>;
    fn get_class_name(handle: HWND) -> Result<String, ApplicationError>;
    fn get_window_text(handle: HWND) -> Result<String, ApplicationError>
    ;
    fn get_parent_handle(handle: HWND) -> Result<HWND, ApplicationError>;
    fn get_window_thread_process_id(handle: HWND) -> Option<u32>;
}

impl MainFeature for TrayItem {
    fn find_window(window_class: Option<&str>, window_name: Option<&str>) -> Result<HWND, ApplicationError> {
        let handle = unsafe {
            match FindWindowW(
                window_class.unwrap_or("").to_pcwstr(),
                window_name.unwrap_or("").to_pcwstr(),
            ) {
                Ok(h) => h,
                Err(_) => { return Err(ApplicationError::new("Cannot Find Existing Window.")); }
            }
        };
        Ok(handle)
    }

    fn find_window_ex(handle_parent: HWND, handle_child: HWND, window_class: Option<&str>, window_name: Option<&str>) -> HWND {
        let handle = unsafe {
            FindWindowExW(
                handle_parent,
                handle_child,
                window_class.unwrap_or("").to_pcwstr(),
                window_name.unwrap_or("").to_pcwstr(),
            )
        };

        let h = match handle {
            Ok(h) => h,
            Err(err) => {
                let _message = format!("returned {} : {}, at {}", err.code(), err.message(), chrono::Local::now());

                #[cfg(debug_assertions)]
                println!("FindWindowExW : {}", _message);
                return HWND::default();
            }
        };

        return h;
    }

    fn find_child_window_enum(handle_parent: HWND, proc: WNDENUMPROC, option: AppData) -> bool
    {
        let input_ptr = Box::into_raw(Box::new(option));

        let result = unsafe {
            EnumChildWindows(
                handle_parent,
                proc,
                LPARAM(input_ptr as isize),
            )
        };
        result.as_bool()
    }

    fn find_window_enum(proc: WNDENUMPROC, option: AppData)
    {
        let input_ptr = Box::into_raw(Box::new(option));

        let _result = unsafe {
            EnumWindows(
                proc,
                LPARAM(input_ptr as isize),
            )
        };
    }

    fn hide_item(handle_area: HWND) -> Result<(), ApplicationError> {
        unsafe {
            let handle_parent = GetParent(handle_area).unwrap_or_else(|_| { HWND_BOTTOM });

            SetWindowPos(handle_area, handle_parent, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOACTIVATE | SWP_NOCOPYBITS | SWP_NOREDRAW | SWP_NOSENDCHANGING | SWP_ASYNCWINDOWPOS).unwrap_or_default();

            _ = RedrawWindow(handle_area, None, None, RDW_ERASE | RDW_INVALIDATE | RDW_ERASENOW | RDW_ALLCHILDREN);
            Ok(())
        }
    }

    fn get_area_rect(handle_area: HWND) -> Result<RECT, ApplicationError>
    {
        let mut rect = RECT::default();
        unsafe {
            match GetWindowRect(handle_area, &mut rect) {
                Ok(_) => { Ok(rect) }
                Err(_) => { Err(ApplicationError::new("Cannot Get Window Rect.")) }
            }
        }
    }

    fn set_windows_position(handle_area: HWND, insert_after: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<(), ApplicationError>
    {
        unsafe {
            match SetWindowPos(handle_area, insert_after, x, y, width, height, SWP_NOMOVE) {
                Ok(_) => { Ok(()) }
                Err(_) => { Err(ApplicationError::new("Cannot Set Window Position.")) }
            }
        }
    }

    fn get_class_name(handle: HWND) -> Result<String, ApplicationError>
    {
        let mut class_name_buffer = [0u16; 256];
        unsafe {
            if GetClassNameW(handle, &mut class_name_buffer) == 0 {
                return Err(ApplicationError::new("Cannot Get Class Name."));
            } else {
                let class_name = String::from_utf16_lossy(&class_name_buffer);
                Ok(class_name.trim_matches('\0').trim_matches(char::REPLACEMENT_CHARACTER).to_string())
            }
        }
    }

    fn get_window_text(handle: HWND) -> Result<String, ApplicationError>
    {
        let mut text_buffer = [0u16; 256];
        unsafe {
            if GetWindowTextW(handle, &mut text_buffer) == 0 {
                return Err(ApplicationError::new("Cannot Get Window Text."));
            } else {
                let text = String::from_utf16_lossy(&text_buffer);
                Ok(text.trim_matches('\0').trim_matches(char::REPLACEMENT_CHARACTER).to_string())
            }
        }
    }

    fn get_parent_handle(handle: HWND) -> Result<HWND, ApplicationError>
    {
        let parent_handle = unsafe {
            match GetParent(handle) {
                Ok(h) => h,
                Err(_) => { return Err(ApplicationError::new("Cannot Get Parent Handle.")); }
            }
        };
        Ok(parent_handle)
    }

    fn get_window_thread_process_id(handle: HWND) -> Option<u32> {
        let mut pid = 0;
        unsafe {
            GetWindowThreadProcessId(handle, Some(&mut pid));
        }
        return if pid > 0 { Some(pid) } else { None };
    }
}


