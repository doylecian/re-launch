#![allow(dead_code)]

extern crate relaunch;
use relaunch::process::get_process_list;

fn main() { //
    let args: Vec<String> = std::env::args().collect();
    let process_name = &*args[1];

    for process in get_process_list() {
        if process.name.starts_with(process_name) {
            println!("\nFound {:#?}", process);
            std::process::exit(0);
        }
    }
}
