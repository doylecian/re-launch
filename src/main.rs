#![allow(dead_code)]

use crate::process::get_process_list;
mod process;

fn main() {
    for process in get_process_list() {
        println!("{:?}", process);
    }
}

#[cfg(test)]
mod tests {
    use crate::process::Process;

    #[test]
	fn find_process_by_name() {
		Process::from_name("System").expect("Find process by name failed");
	}

    #[test]
    fn find_process_from_pid() {
		Process::from_pid(4).expect("Find process by PID failed");
	}
}

