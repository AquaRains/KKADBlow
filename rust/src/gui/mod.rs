use windows::Win32::UI::WindowsAndMessaging::HICON;

pub(crate) mod w32;

pub type TrayApplication = w32::TrayItem;

#[allow(dead_code)]
pub enum IconSource {
    Resource(&'static str),
    RawIcon(HICON),
}

#[allow(dead_code)]
impl IconSource {
    pub fn as_str(&self) -> &str {
        match self {
            IconSource::Resource(res) => res,
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

