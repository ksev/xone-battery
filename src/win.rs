use std;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HWND};
use winapi::shared::ntdef::{LPCWSTR, WCHAR};

use winapi::um::winuser;
use winapi::um::libloaderapi;
use winapi::um::shellapi;

pub fn initialize() -> Option<HWND> {
    // "xboxone-battery-class"
    const CLASS_NAME: &[u16] = &[
        120, 98, 111, 120, 111, 110, 101, 45, 98, 97, 
        116, 116, 101, 114, 121, 45, 99, 108, 97, 115, 115, 0
    ];

    // "xboxone-battery-window"
    const WINDOW_NAME: &[u16] = &[
        120, 98, 111, 120, 111, 110, 101, 45, 98, 97, 116, 116, 
        101, 114, 121, 45, 119, 105, 110, 100, 111, 119, 0
    ];

    unsafe {
        let module = libloaderapi::GetModuleHandleW(std::ptr::null());

        let wnd = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            hInstance: module,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            lpszClassName: CLASS_NAME.as_ptr(),
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
            WINDOW_NAME.as_ptr(),
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

pub fn add_icon(hwnd: HWND, id: GUID, res_id: isize, tip: &'static [u16]) -> bool {
    let mut array = [0 as WCHAR; 128];
    for (x, p) in tip.iter().zip(array.iter_mut()) {
        *p = *x;
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

pub fn change_icon(hwnd: HWND, id: GUID, res_id: isize, tip: &'static [u16]) -> bool {
    let mut array = [0 as WCHAR; 128];
    for (x, p) in tip.iter().zip(array.iter_mut()) {
        *p = *x;
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
