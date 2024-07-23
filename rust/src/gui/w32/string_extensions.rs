use windows::core::{PCWSTR, PWSTR};

#[allow(dead_code)]
pub trait ToPCWSTRWrapper {
    fn to_pcwstr(&self) -> PCWSTR;

    fn to_vec_u16(&self) -> Vec<u16>;
}

#[allow(dead_code)]
pub trait ToPWSTRWrapper {
    fn to_pwstr(&self) -> PWSTR;
}


struct StrWrapper(Vec<u16>);
impl StrWrapper
{
    fn new<T: AsRef<str>>(text: T) -> Self
    {
        let text = text.as_ref();
        let mut text = text.encode_utf16().collect::<Vec<_>>();
        text.push(0);

        Self(text)
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String
    {
        String::from_utf16_lossy(&self.0)
    }
}


struct PCWSTRWrapper(Vec<u16>);

impl PCWSTRWrapper {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.0.as_ptr())
    }

    fn new<T: AsRef<str>>(text: T) -> Self {
        let text = StrWrapper::new(text);

        Self(text.0)
    }

    fn from_string(text: String) -> Self {
        let mut text = text.encode_utf16().collect::<Vec<_>>();
        text.push(0);

        Self(text)
    }

    fn to_vec_u16(&self) -> Vec<u16> {
        self.0.clone()
    }
}

impl ToPCWSTRWrapper for &str {
    fn to_pcwstr(&self) -> PCWSTR {
        PCWSTRWrapper::new(self).as_pcwstr()
    }

    fn to_vec_u16(&self) -> Vec<u16> {
        PCWSTRWrapper::new(self).to_vec_u16()
    }
}
impl ToPCWSTRWrapper for String {
    fn to_pcwstr(&self) -> PCWSTR {
        PCWSTRWrapper::from_string(self.clone()).as_pcwstr()
    }

    fn to_vec_u16(&self) -> Vec<u16> {
        PCWSTRWrapper::from_string(self.clone()).to_vec_u16()
    }
}

struct PWSTRWrapper(Vec<u16>);


impl PWSTRWrapper
{
    fn new<T: AsRef<str> + AsRef<str>>(text: T) -> Self
    {
        let text = StrWrapper::new(text);
        Self(text.0)
    }

    fn as_pwstr(&self) -> PWSTR
    {
        PWSTR::from_raw(self.0.as_ptr() as *mut u16)
    }
}

impl ToPWSTRWrapper for PCWSTR {
    fn to_pwstr(&self) -> PWSTR {
        PWSTR::from_raw(self.as_ptr() as *mut u16)
    }
}

impl ToPWSTRWrapper for &str {
    fn to_pwstr(&self) -> PWSTR {
        PWSTRWrapper::new(self).as_pwstr()
    }
}

