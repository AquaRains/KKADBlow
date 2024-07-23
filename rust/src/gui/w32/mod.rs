pub(crate) mod structs;
pub(crate) mod ui;
pub(super) mod extern_func;
mod string_extensions;
pub(crate) mod traits;

use std::{cell::RefCell, sync::{
    mpsc::Sender,
    Arc,
    Mutex,
}, thread};
use windows::Win32::Foundation::{HWND};
use structs::*;
use crate::err::ApplicationError;
use crate::gui::w32::extern_func::{enumerate_child_windows, enumerate_windows};
use crate::gui::w32::traits::MainFeature;
use crate::structs::{AppData, Message};

thread_local!(static WININFO_STASH: RefCell<Option<WindowsLoopData>> = RefCell::new(None));
type CallBackEntry = Option<Box<dyn Fn() + Send + 'static>>;

pub struct TrayItem {
    entries: Arc<Mutex<Vec<CallBackEntry>>>,
    info: WindowInfo,
    windows_loop: Option<thread::JoinHandle<()>>,
    event_loop: Option<thread::JoinHandle<()>>,
    event_sender: Sender<WindowsTrayEvent>,
    app_options: AppData,
}

impl TrayItem {
    pub(crate) fn set_options(&mut self, options: AppData) {
        self.app_options = options;
    }
}


#[macro_export]
macro_rules! zeroed {
    ($type:ty) => {
        #[allow(unused_unsafe)]
        unsafe {std::mem::zeroed::<$type>()}
    };
}

pub fn find_area_and_shrink(_: Option<Sender<Message>>, mut option: AppData) -> Result<(), ApplicationError> {
    let (target_class, target_window, _) = option.target_window.deconstruct();
    let parent = match TrayItem::find_window(Some(target_class), Some(target_window)) {
        Ok(p) => p,
        Err(e) => return Err(e)
    };

    option.target_window.handle = Some(parent);

    let target_pid = match TrayItem::get_window_thread_process_id(parent) {
        Some(pid) => pid,
        None => return Err(ApplicationError::new("Cannot Find Target Window"))
    };

    option.target_window.process_id = Some(target_pid);
    let mut ad_target_founded = false;
    let mut main_target_founded = false;

    //먼저 광고창 찾아서 조진다음
    enumerate_windows(|h| {
        let parent_pid = option.target_window.process_id;

        //지금 훑고있는 창 hwnd의 프로세스 id
        let current_pid = TrayItem::get_window_thread_process_id(h);

        //찾아낼 창의 클래스명
        let (ad_class, ad_windowname, _) = option.target_ad_area.deconstruct();

        //내가 찾을 타겟의 프로세스를 확인
        if parent_pid == current_pid {
            let current_class = TrayItem::get_class_name(h).unwrap_or_default();
            let current_window = TrayItem::get_window_text(h).unwrap_or_default();

            let (target_class, target_window, _) = option.target_window.deconstruct();


            //광고부분일경우
            if ad_class == current_class && current_window == ad_windowname {
                ad_target_founded = {
                    //현재 창의 핸들로 child 핸들명도 찾아서 체크 해야 한다.
                    enumerate_child_windows(h, |h2| {
                        //지금 창은 광고창인지 확인하기 위함이다.
                        let (ad_child_class_from_option, _, _) = option.target_ad_area_child.deconstruct();
                        //현재 창의 클래스명도 비교
                        return match TrayItem::get_class_name(h2) {
                            Ok(current_class) => {
                                if ad_child_class_from_option == current_class {
                                    option.target_ad_area_child.handle = Some(h2);
                                    option.target_ad_area.handle = Some(h);
                                    return false;
                                }
                                true
                            }
                            Err(_) => {
                                option.target_ad_area_child.handle = None;
                                option.target_ad_area.handle = None;
                                true
                            }
                        };
                    });

                    // 작업이 완료되고 저장된 핸들이 있는지 확인
                    match option.target_ad_area_child.handle {
                        None => { false }
                        Some(_) => {
                            option.target_ad_area.handle = Some(h);
                            //제거하기 전에 해당 창 크기 저장
                            let rect = match TrayItem::get_area_rect(h) {
                                Ok(rect) => { rect }
                                Err(_) => { return false }
                            };
                            option.target_ad_area.window_area = Some(rect);
                            //여기까지 찾았으면 빼박이니까 제거, 제거하기전에 0으로 만들고 제거해버리자
                            match TrayItem::set_windows_position(h, HWND::default(), 0, 0, 0, 0) {
                                Ok(_) => {  }
                                Err(_) => {  }
                            }

                            match TrayItem::hide_item(h) {
                                Ok(_) => { true }
                                Err(_) => { false }
                            }
                        }
                    }
                }
            } else if current_class == target_class && current_window == target_window {
                option.target_window.handle = Some(h); //찾기만하고 enumerate는 continue
                main_target_founded = true;
            }
        }
        //찾았으면 FALSE로 리턴하여 루프 종료
        return !(ad_target_founded && main_target_founded);
    });

    let rect = option.target_ad_area.window_area;
    let main_handle = option.target_window.handle;
    let mut shrinkable_found: bool = false;
    let mut expandable_found: bool = false;
    match (main_handle, rect) {
        (Some(h), Some(_)) => {
            //광고창을 제거하고 남은 빈칸을 땡겨서 맞춰준다
            enumerate_child_windows(h, |h2| {
                //원래 예전 광고 핸들이 있던 위치를 shrinkable_area로 지정
                let (shrinkable_class, shrinkable_name, _) = option.target_shrinkable_area.deconstruct();
                let (expandable_class, expandable_name, _) = option.target_expandable_area.deconstruct();
                let current_class = TrayItem::get_class_name(h2).unwrap_or_default();
                let current_window = TrayItem::get_window_text(h2).unwrap_or_default();

                //얘네들 웃긴게 window_name뒤에 hwnd 박아놓음. 쌩까야지.
                if expandable_class == current_class && current_window.starts_with(expandable_name) {
                    expandable_found = true;
                    option.target_expandable_area.handle = Some(h2);
                    return true;
                }

                if shrinkable_class == current_class && current_window.starts_with(shrinkable_name) {
                    shrinkable_found = true;
                    option.target_shrinkable_area.handle = Some(h2);
                    return true;
                }

                //모두 찾았으면 루프 종료
                return !shrinkable_found || !expandable_found;
            });
        }
        _ => {}
    }

    //expandeable_area를 shrinkable_area의 크기만큼 늘려준다
    match (option.target_shrinkable_area.handle, option.target_expandable_area.handle,option.target_window.handle) {
        (Some(_shr), Some(exp),Some(mhd)) =>
            {
                let main_rect = match TrayItem::get_area_rect(mhd) {
                    Ok(r) => { r }
                    Err(_) => { Err(ApplicationError::new("Cannot Find Main Area"))? }
                };

                let expandable_rect = match TrayItem::get_area_rect(exp) {
                    Ok(r) => { r }
                    Err(_) => { Err(ApplicationError::new("Cannot Find Expandable Area"))? }
                };

                let expandable_width = expandable_rect.right - expandable_rect.left;

                match TrayItem::set_windows_position(exp, HWND::default(),expandable_rect.left, expandable_rect.top, expandable_width, main_rect.bottom - expandable_rect.top) {
                    Ok(_) => {}
                    Err(_) => { Err(ApplicationError::new("Cannot Set Expandable Area"))? }
                }
            }
        (_, _,_) => {}
    }

    Ok(())
}






