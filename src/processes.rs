use std::ffi::c_void;

use cidre::{core_audio as ca, sys};

const PROC_PIDPATHINFO_MAXSIZE: usize = 4 * 1024;

unsafe extern "C" {
    fn proc_pidpath(pid: i32, buffer: *mut c_void, buffersize: u32) -> i32;
}

pub struct AudioProcess {
    pub pid: Option<sys::Pid>,
    pub bundle_id: String,
    pub running_output: bool,
    pub path: Option<String>,
}

pub fn list() -> Result<Vec<AudioProcess>, String> {
    let processes = ca::System::processes()
        .map_err(|e| format!("listing Core Audio processes failed: {e:?}"))?;
    Ok(processes
        .iter()
        .map(|p| {
            let pid = p.pid().ok();
            AudioProcess {
                pid,
                bundle_id: p.bundle_id().map(|s| s.to_string()).unwrap_or_default(),
                running_output: p.is_running_output().unwrap_or(false),
                path: pid.and_then(executable_path),
            }
        })
        .collect())
}

fn executable_path(pid: sys::Pid) -> Option<String> {
    let mut buf = vec![0u8; PROC_PIDPATHINFO_MAXSIZE];
    let len = unsafe { proc_pidpath(pid, buf.as_mut_ptr().cast(), buf.len() as u32) };
    if len <= 0 {
        return None;
    }
    buf.truncate(len as usize);
    String::from_utf8(buf).ok()
}
