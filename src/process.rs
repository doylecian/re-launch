use std::collections::HashMap;

use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use windows::Win32::Foundation::{HANDLE, WIN32_ERROR, GetLastError};

static WIN32_ERROR_CODES: phf::Map<u32, &str> = phf::phf_map! {
    0_u32 => "ERROR_SUCCESS",
    5_u32 => "ERROR_ACCESS_DENIED",
    6_u32 => "ERROR_INVALID_HANDLE",
};

pub fn get_process_list() -> Vec<Process> {
    let mut system_info = System::new_all();
    system_info.refresh_all();
    let mut processes = Vec::new();
    for (_, process) in system_info.processes() {
        processes.push( Process { pid: process.pid().as_u32(), name: process.name().to_string(), handle: None, exit_code: None } );
    }    
    return processes;
}

pub struct Process {
    name: String,
    pid: u32,
    handle: Option<HANDLE>,
    exit_code: Option<u32>,
}

impl Process {
    pub fn new(name: String, pid: u32, handle: HANDLE, exit_code: u32) -> Self {
        Self { name, pid, handle: Some(handle), exit_code: Some(exit_code) }
    }
    
    // TODO: Instantiate new process if found
    pub fn from_name(name: &str) -> Result<(), WIN32_ERROR> {
        for process in get_process_list().iter() {
            if process.name == name {
                return Ok(())
            }
        }
        unsafe { Err(GetLastError()) }
    }

    // TODO: Instantiate new process if found
    pub fn from_pid(pid: u32) -> Result<(), WIN32_ERROR> {
        for process in get_process_list() {
            if process.pid == pid {
                return Ok(())
            }
        }
        unsafe { Err(GetLastError()) }
    }

    // TODO: Implement method for checking current exit status code
    pub fn get_status() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Process;

    #[test]
	fn find_process_by_name() {
		Process::from_name("System").expect("Find process by name failed");
	}

    #[test]
    fn find_process_from_pid() {
		Process::from_pid(4).expect("Find process by PID failed");
	}
}