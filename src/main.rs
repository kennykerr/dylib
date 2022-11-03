#![feature(raw_dylib)]
#![feature(native_link_modifiers_verbatim)]

fn main() {
    unsafe {
        MessageBoxA(0, b"hello\0".as_ptr(), "world\0".as_ptr(), 0);
        ShellMessageBoxA(0, 0, b"hello\0".as_ptr(), "world\0".as_ptr(), 0);
    }
}

#[link(name = "user32.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "stdcall" {
    fn MessageBoxA(hwnd: usize, text: *const u8, caption: *const u8, flags: u32) -> i32;
}

#[link(name = "shlwapi.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "cdecl" {
    fn ShellMessageBoxA(hinstance: usize, hwnd: usize, text: *const u8, caption: *const u8, flags: u32) -> i32;
}
