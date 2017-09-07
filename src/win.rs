use std;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use uuid;

use winapi;
use kernel32;
use user32;
use shell32;

pub trait ToWin {
    type Out;
    fn to_win(&self) -> Self::Out;
}

impl<'a> ToWin for &'a str {
    type Out = Vec<u16>;

    fn to_win(&self) -> Self::Out {
        OsStr::new(self)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
}

impl ToWin for uuid::Uuid {
    type Out = winapi::GUID;

    fn to_win(&self) -> Self::Out {
        let bytes = self.as_bytes();
        let end = [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                   bytes[15]];

        winapi::GUID {
            Data1: ((bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 |
                    (bytes[3] as u32)),
            Data2: ((bytes[4] as u16) << 8 | (bytes[5] as u16)),
            Data3: ((bytes[6] as u16) << 8 | (bytes[7] as u16)),
            Data4: end,
        }
    }
}

pub fn initialize() -> Option<winapi::HWND> {
    let class_name = "xboxone-battery-class".to_win();
    let window_name = "xboxone-battery-window".to_win();

    unsafe {
        let module = kernel32::GetModuleHandleW(std::ptr::null());

        let wnd = winapi::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            hInstance: module,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hbrBackground: winapi::COLOR_WINDOW as winapi::HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
            cbClsExtra: 0,
            cbWndExtra: 0,
        };

        let atom = user32::RegisterClassW(&wnd);

        if atom == 0 {
            return None;
        }

        let hwnd = user32::CreateWindowExW(0,
                                           atom as winapi::LPCWSTR,
                                           window_name.as_ptr(),
                                           winapi::WS_DISABLED,
                                           0,
                                           0,
                                           0,
                                           0,
                                           user32::GetDesktopWindow(),
                                           std::ptr::null_mut(),
                                           std::ptr::null_mut(),
                                           std::ptr::null_mut());

        if hwnd.is_null() {
            return None;
        }

        return Some(hwnd);
    }
}

pub fn add_icon(hwnd: winapi::HWND, id: winapi::GUID, res_id: isize, tip: &str) -> bool {
    let mut array = [0 as winapi::WCHAR; 128];
    for (&x, p) in tip.to_win().iter().zip(array.iter_mut()) {
        *p = x;
    }

    unsafe {
        let module = kernel32::GetModuleHandleW(std::ptr::null());
        let icon = user32::LoadIconW(module, std::mem::transmute(res_id));

        let mut nid = winapi::NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<winapi::NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uFlags: winapi::NIF_ICON | winapi::NIF_GUID | winapi::NIF_TIP,
            guidItem: id,
            hIcon: icon,
            uID: 0,
            uCallbackMessage: 0,
            szTip: array,
            dwState: 0,
            dwStateMask: 0,
            szInfo: [0; 256],
            uTimeout: 0,
            szInfoTitle: [0; 64],
            dwInfoFlags: 0,
            hBalloonIcon: std::ptr::null_mut(),
        };

        shell32::Shell_NotifyIconW(winapi::NIM_ADD, &mut nid as winapi::PNOTIFYICONDATAW) == 1
    }
}

pub fn change_icon(hwnd: winapi::HWND, id: winapi::GUID, res_id: isize, tip: &str) -> bool {
    let mut array = [0 as winapi::WCHAR; 128];
    for (&x, p) in tip.to_win().iter().zip(array.iter_mut()) {
        *p = x;
    }

    unsafe {
        let module = kernel32::GetModuleHandleW(std::ptr::null());
        let icon = user32::LoadIconW(module, std::mem::transmute(res_id));

        let mut nid = winapi::NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<winapi::NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uFlags: winapi::NIF_ICON | winapi::NIF_GUID | winapi::NIF_TIP,
            guidItem: id,
            hIcon: icon,
            uID: 0,
            uCallbackMessage: 0,
            szTip: array,
            dwState: 0,
            dwStateMask: 0,
            szInfo: [0; 256],
            uTimeout: 0,
            szInfoTitle: [0; 64],
            dwInfoFlags: 0,
            hBalloonIcon: std::ptr::null_mut(),
        };

        shell32::Shell_NotifyIconW(winapi::NIM_MODIFY, &mut nid as winapi::PNOTIFYICONDATAW) == 1
    }
}

pub unsafe extern "system" fn window_proc(h_wnd: winapi::HWND,
                                          msg: winapi::UINT,
                                          w_param: winapi::WPARAM,
                                          l_param: winapi::LPARAM)
                                          -> winapi::LRESULT {
    if msg == winapi::winuser::WM_DESTROY {
        user32::PostQuitMessage(0);
    }
    return user32::DefWindowProcW(h_wnd, msg, w_param, l_param);
}