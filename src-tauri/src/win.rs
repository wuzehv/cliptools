use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct Win {}

impl Win {
    pub fn send_message(text: &str) {
        let hwnd = unsafe { GetForegroundWindow() };
        for c in text.chars() {
            unsafe {
                SendMessageW(
                    hwnd,
                    WM_CHAR,
                    Option::from(WPARAM(c as usize)),
                    Option::from(LPARAM(0)),
                );
            }
        }
    }
}
