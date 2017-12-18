use std;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use uuid;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::shared::ntdef::{LPCWSTR, WCHAR};

use winapi::um::winuser;
use winapi::um::libloaderapi;
use winapi::um::shellapi;

pub trait ToWin {
    type Out;
    fn to_win(&self) -> Self::Out;
}

fn win_str<'a>(data: &'a str) -> impl Iterator<Item = u16> + 'a {
    OsStr::new(data).encode_wide().chain(std::iter::once(0))
}

impl ToWin for uuid::Uuid {
    type Out = GUID;

    fn to_win(&self) -> Self::Out {
        let bytes = self.as_bytes();
        let end = [
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ];

        GUID {
            Data1: ((bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8
                | (bytes[3] as u32)),
            Data2: ((bytes[4] as u16) << 8 | (bytes[5] as u16)),
            Data3: ((bytes[6] as u16) << 8 | (bytes[7] as u16)),
            Data4: end,
        }
    }
}

pub fn initialize() -> Option<HWND> {
    let class_name = win_str("xboxone-battery-class").collect::<Vec<_>>();
    let window_name = win_str("xboxone-battery-window").collect::<Vec<_>>();

    unsafe {
        let module = libloaderapi::GetModuleHandleW(std::ptr::null());

        let wnd = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            hInstance: module,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hbrBackground: winuser::COLOR_WINDOW as HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
            cbClsExtra: 0,
            cbWndExtra: 0,
        };

        let atom = winuser::RegisterClassW(&wnd);

        if atom == 0 {
            return None;
        }

        let hwnd = winuser::CreateWindowExW(
            0,
            atom as LPCWSTR,
            window_name.as_ptr(),
            winuser::WS_DISABLED,
            0,
            0,
            0,
            0,
            winuser::GetDesktopWindow(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            return None;
        }

        return Some(hwnd);
    }
}

pub fn add_icon(hwnd: HWND, id: GUID, res_id: isize, tip: &str) -> bool {
    let mut array = [0 as WCHAR; 128];
    for (x, p) in win_str(tip).zip(array.iter_mut()) {
        *p = x;
    }

    unsafe {
        let module = libloaderapi::GetModuleHandleW(std::ptr::null());
        let icon = winuser::LoadIconW(module, std::mem::transmute(res_id));

        let mut nid = shellapi::NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uFlags: shellapi::NIF_ICON | shellapi::NIF_GUID | shellapi::NIF_TIP,
            guidItem: id,
            hIcon: icon,
            uID: 0,
            uCallbackMessage: 0,
            szTip: array,
            dwState: 0,
            dwStateMask: 0,
            szInfo: [0; 256],
            u: std::mem::transmute([0; 1]),
            szInfoTitle: [0; 64],
            dwInfoFlags: 0,
            hBalloonIcon: std::ptr::null_mut(),
        };

        shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &mut nid as shellapi::PNOTIFYICONDATAW) == 1
    }
}

pub fn change_icon(hwnd: HWND, id: GUID, res_id: isize, tip: &str) -> bool {
    let mut array = [0 as WCHAR; 128];
    for (x, p) in win_str(tip).zip(array.iter_mut()) {
        *p = x;
    }

    unsafe {
        let module = libloaderapi::GetModuleHandleW(std::ptr::null());
        let icon = winuser::LoadIconW(module, std::mem::transmute(res_id));

        let mut nid = shellapi::NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uFlags: shellapi::NIF_ICON | shellapi::NIF_GUID | shellapi::NIF_TIP,
            guidItem: id,
            hIcon: icon,
            uID: 0,
            uCallbackMessage: 0,
            szTip: array,
            dwState: 0,
            dwStateMask: 0,
            szInfo: [0; 256],
            u: std::mem::transmute([0; 1]),
            szInfoTitle: [0; 64],
            dwInfoFlags: 0,
            hBalloonIcon: std::ptr::null_mut(),
        };

        shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &mut nid as shellapi::PNOTIFYICONDATAW)
            == 1
    }
}

pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if msg == winuser::WM_DESTROY {
        winuser::PostQuitMessage(0);
    }
    return winuser::DefWindowProcW(h_wnd, msg, w_param, l_param);
}
