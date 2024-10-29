#![windows_subsystem = "windows"]

use std::{env, fs};
use std::process::Command;
use std::os::windows::process::CommandExt;
use winapi::um::winbase::CREATE_NO_WINDOW;
use urlencoding::decode;

fn main() {
    // hide_console_window();
    // unsafe { winapi::um::wincon::FreeConsole() };

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        return;
    }

    let mut str = args[1].replace("openfile://", "");
    str = decode(&str).expect("UTF-8").to_string();

    match fs::metadata(&mut str) {
        Ok(_) => {
            // handle space and special characters in path
            let result = str.split("/").skip(1).map(|x| format!("\"{}\"", x)).collect::<Vec<String>>().join("/");
            let drive = str.split("/").next().unwrap();

            str = format!("/C {}/{}", drive, result);

            let mut binding = Command::new("cmd");
            let command = binding.raw_arg(str);
            command.creation_flags(CREATE_NO_WINDOW); // Be careful: This only works on windows
            command.spawn( ).unwrap( );
        },
        Err(_) => {
            // println!("File does not exist!")
        },
    }
}

// fn hide_console_window() {
//    let window = unsafe {GetConsoleWindow()};
//    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
//    if window != ptr::null_mut() {
//        unsafe {
//            ShowWindow(window, SW_HIDE);
//        }
//    }
// }