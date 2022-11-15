use sysinfo::{Pid, System, SystemExt, ProcessExt};

pub struct ProcessData;

impl ProcessData {
    
    pub fn get_processes(&self, sys: System) {
        for (pid, process) in sys.processes() {
            println!("[{}] {} {:?}", pid, process.name(), process.memory());
        }
    }
}

