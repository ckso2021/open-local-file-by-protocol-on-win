#![windows_subsystem = "windows"]

use std::{env, fs};
use std::process::Command;
use std::os::windows::process::CommandExt;
use winapi::um::winbase::CREATE_NO_WINDOW;
// use winapi::um::winbase::DETACHED_PROCESS;
use urlencoding::decode;
// use std::ptr;
// use winapi::um::wincon::GetConsoleWindow;
// use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn main() {
   let args: Vec<String> = env::args().collect();

   if args.len() <= 1 {
      return;
   }

   let mut str = args[1].replace("openfile://", "");
   str = decode(&str).expect("UTF-8").to_string();

    match fs::metadata(&mut str) {
        Ok(_) => {
         str = format!("{}", str);
         // println!("Opening {}", str);
         let mut binding = Command::new("cmd");
         let command = binding.args(&["/C", &str]);
         command.creation_flags(CREATE_NO_WINDOW); // Be careful: This only works on windows
         command.spawn( ).unwrap( );
        },
        Err(_) => {
         // println!("File does not exist!")
        },
    }
}
