use std::collections::HashMap;

use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use windows::Win32::Foundation::{HANDLE, WIN32_ERROR, GetLastError};

static WIN32_ERROR_CODES: phf::Map<u32, &str> = phf::phf_map! {
    0_u32 => "ERROR_SUCCESS",
    5_u32 => "ERROR_ACCESS_DENIED",
    6_u32 => "ERROR_INVALID_HANDLE",
};

pub fn get_process_list() -> HashMap<u32, String> {
    let mut system_info = System::new_all();
    system_info.refresh_all();
    let mut processes = HashMap::new();
    for (_, process) in system_info.processes() {
        processes.insert(process.pid().as_u32(), process.name().to_string());
    }    
    return processes;
}

pub struct Process {
    name: String,
    pid: u32,
    handle: HANDLE,
    exit_code: u32,
}

impl Process {
    pub fn new(name: String, pid: u32, handle: HANDLE, exit_code: u32) -> Self {
        Self { name, pid, handle, exit_code }
    }
    
    // TODO: Instantiate new process if found
    pub fn from_name(name: &str) -> Result<(), WIN32_ERROR> {
        for (_, v) in get_process_list().iter() {
            if v == name {
                return Ok(())
            }
        }
        unsafe { Err(GetLastError()) }
    }

    // TODO: Instantiate new process if found
    pub fn from_pid(pid: u32) -> Result<(), WIN32_ERROR> {
        if let Some(_) = get_process_list().get(&pid) {
            return Ok(())
        }
        else {
            unsafe { Err(GetLastError()) }
        }
    }

    // TODO: Implement method for checking current exit status code
    pub fn get_status() {
        todo!()
    }
}
