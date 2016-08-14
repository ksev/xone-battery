#![feature(conservative_impl_trait)]
extern crate winapi;
extern crate xinput;
extern crate user32;
extern crate comctl32;
extern crate kernel32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn main() {
    let class_name = encode_wide("xboxone-battery-class");    

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

        println!("{:?}", atom);

        let hwnd = user32::CreateWindowExW(
           0, atom as winapi::LPCWSTR, 
           encode_wide("xboxone-battery-window").as_ptr(), 
           0, 0, 0, 0, 0, winapi::HWND_MESSAGE, 
           std::ptr::null_mut(), 
           std::ptr::null_mut(), 
           std::ptr::null_mut()); 

        println!("{:?}", hwnd);

        std::thread::sleep(std::time::Duration::from_secs(15));            
    }
}


pub unsafe extern "system" fn window_proc(h_wnd: winapi::HWND, 
	msg: winapi::UINT, w_param: winapi::WPARAM, l_param: winapi::LPARAM) -> winapi::LRESULT
{
    if msg == winapi::winuser::WM_DESTROY {
        user32::PostQuitMessage(0);
    }
    return user32::DefWindowProcW(h_wnd, msg, w_param, l_param);
}

fn encode_wide<'a>(input: &'a str) -> Vec<u16> {
    OsStr::new(input).encode_wide()
                     .chain(std::iter::once(0))
                     .collect()
}
