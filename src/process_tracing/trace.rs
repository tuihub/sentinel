use std::{
    ops::Add,
    path::Path,
    process,
    sync::{Arc, Mutex},
    thread,
    thread::sleep,
    time::{Duration, UNIX_EPOCH},
};

use log::info;
use strum_macros::{Display, EnumString};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
use time;

use crate::{err_msg, Result};

/// trace mode
#[derive(Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TraceMode {
    /// Wait child process to exit
    Simple,
    /// Search process by given name and wait them all exit
    ByName,
}

/// find and trace stared process then get end_time and exit_code
pub fn start_and_trace(
    mode: TraceMode,
    name: &str,
    path: &Path,
    sleep_count: i32,
    sleep_millis: u64,
) -> Result<(time::OffsetDateTime, time::OffsetDateTime, bool)> {
    let mut child = process::Command::new(path).spawn()?;
    let start_time = time::OffsetDateTime::now_utc();
    match mode {
        TraceMode::Simple => {
            let status = child.wait()?;
            let end_time = time::OffsetDateTime::now_utc();
            Ok((start_time, end_time, status.success()))
        }
        TraceMode::ByName => {
            for _ in 0..sleep_count {
                sleep(Duration::from_millis(sleep_millis));
                let s = System::new_all();
                let exit_code_mutex = Arc::new(Mutex::new(0));
                let processes: Vec<&Process> = s
                    .processes_by_exact_name(name)
                    .filter(|p| {
                        time::OffsetDateTime::from(
                            UNIX_EPOCH.add(time::Duration::seconds(p.start_time() as i64)),
                        )
                        .add(time::Duration::seconds(1))
                            > start_time
                    })
                    .filter(|p| p.exe() == path)
                    .collect();
                info!("processes num: {}", processes.len());
                let handles: Vec<_> = processes
                    .into_iter()
                    .map(|p| p.pid().as_u32())
                    .map(|pid| {
                        let mu = Arc::clone(&exit_code_mutex);
                        thread::spawn(move || {
                            if let Ok(exit_code) = wait_for_process_exit(pid) {
                                if exit_code != 0 {
                                    let mut guard = mu.lock().unwrap();
                                    *guard = exit_code;
                                }
                            }
                        })
                    })
                    .collect();
                if !handles.is_empty() {
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    let end_time = time::OffsetDateTime::now_utc();
                    let exit_code = *exit_code_mutex.lock().unwrap();
                    return Ok((start_time, end_time, exit_code == 0));
                }
            }
            Err(err_msg!("sleep time limit exceeded"))
        }
    }
}

#[cfg(target_os = "linux")]
fn wait_for_process_exit(pid: u32) -> Result<i32> {
    use libc::{c_int, pid_t, WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG};

    extern "C" {
        fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    }
    let mut status: c_int = 0;
    unsafe {
        let result = waitpid(pid as i32, &mut status as *mut c_int, 0);
        if result == -1 {
            return Err(err_msg!(
                "An error occurred while waiting for the process to exit"
            ));
        }
    }

    if WIFEXITED(status) {
        let exit_code = WEXITSTATUS(status);
        Ok(exit_code)
    } else if WIFSIGNALED(status) {
        let signal = WTERMSIG(status);
        Err(err_msg!(
            "Process terminated by signal, signal number: {}",
            signal
        ))
    } else {
        Err(err_msg!("Unknown error"))
    }
}

#[cfg(target_os = "windows")]
fn wait_for_process_exit(pid: u32) -> Result<i32> {
    use winapi::um::{
        processthreadsapi::{GetExitCodeProcess, OpenProcess},
        synchapi::WaitForSingleObject,
        winnt::{DWORD, PROCESS_QUERY_INFORMATION, STILL_ACTIVE, SYNCHRONIZE},
    };

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | SYNCHRONIZE, 0, pid as DWORD);
        if handle.is_null() {
            return Err(err_msg!("can't get handle"));
        }

        WaitForSingleObject(handle, winapi::um::winbase::INFINITE);

        let mut exit_code: DWORD = 0;
        GetExitCodeProcess(handle, &mut exit_code);

        if exit_code == STILL_ACTIVE {
            WaitForSingleObject(process_handle, INFINITE);
            GetExitCodeProcess(process_handle, &mut exit_code);
        }

        Ok(exit_code as i32)
    }
}
