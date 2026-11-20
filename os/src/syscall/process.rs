//! Process management syscalls
use crate::{
    mm::{MapPermission, VirtAddr},
    task::{
        change_program_brk, exit_current_and_run_next, get_pa, get_syscall_count, mmap, munmap,
        suspend_current_and_run_next, try_getting_pa,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let addr = get_pa(ts as usize).get_mut();
    let us = get_time_us();
    *addr = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");

    if trace_request == 0 {
        if let Some(pa) = try_getting_pa(id, false, true) {
            let des = pa.get();
            return *des;
        }
    } else if trace_request == 1 {
        if let Some(pa) = try_getting_pa(id, true, false) {
            *pa.get_mut() = data;
            return 0;
        }
    } else if trace_request == 2 {
        return get_syscall_count(id);
    }
    -1
}
// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let va: VirtAddr = start.into();
    let prot: MapPermission = (prot as u8).into();
    if !va.aligned() {
        return -1;
    }
    if !mmap(va, len, prot) {
        return -1;
    }
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let va: VirtAddr = start.into();
    if !va.aligned() {
        return -1;
    }
    if !munmap(va, len) {
        return -1;
    }
    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
