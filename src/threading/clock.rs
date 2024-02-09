use std::mem::MaybeUninit;

use crate::threading::errors::ThreadingError;

use super::errors::ThreadingResult;

pub struct Clock {
    timespec: libc::timespec,
}

impl Clock {
    pub fn new() -> ThreadingResult<Clock> {
        let mut timespec = MaybeUninit::<libc::timespec>::uninit();
        let result = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, timespec.as_mut_ptr()) };
        match result {
            0 => Ok(Clock {
                timespec: unsafe { timespec.assume_init() },
            }),
            _ => Err(ThreadingError::new("Could not get clock time")),
        }
    }
    pub fn sleep(&mut self, duration: std::time::Duration) -> ThreadingResult<()> {
        let duration_ns = duration.as_nanos().try_into().unwrap_or(0);
        self.timespec.tv_nsec += duration_ns;
        while self.timespec.tv_nsec >= 1_000_000_000 {
            self.timespec.tv_sec += 1;
            self.timespec.tv_nsec -= 1_000_000_000;
        }
        while self.timespec.tv_nsec < 0 {
            self.timespec.tv_sec -= 1;
            self.timespec.tv_nsec += 1_000_000_000;
        }
        let result = unsafe {
            libc::clock_nanosleep(
                libc::CLOCK_MONOTONIC,
                libc::TIMER_ABSTIME,
                &self.timespec,
                std::ptr::null_mut(),
            )
        };
        match result {
            0 => Ok(()),
            _ => Err(ThreadingError::new("Could not sleep thread")),
        }
    }
}
