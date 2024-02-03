use std::{mem::MaybeUninit, ptr::addr_of_mut};

use super::{
    attributes::{SchedulerPolicy, ThreadAttributes},
    errors::{ThreadingError, ThreadingResult},
};

pub fn lock_memory() -> ThreadingResult<()> {
    let result: i32;
    unsafe {
        result = libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
    }
    match result {
        0 => Ok(()),
        _ => Err(ThreadingError::new("Could not lock memory")),
    }
}

pub struct JoinHandle<T> {
    handle: libc::pthread_t,
    phantom: std::marker::PhantomData<T>,
}

impl<T> JoinHandle<T> {
    pub fn spawn<F>(attributes: ThreadAttributes, fun: F) -> ThreadingResult<JoinHandle<T>>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let attributes = initialize_thread_attributes(attributes)?;
        let mut handle = MaybeUninit::<libc::pthread_t>::uninit();
        let arg = Box::new(GenericThreadArgument {
            main_function: Box::new(fun),
        });
        let result = unsafe {
            libc::pthread_create(
                handle.as_mut_ptr(),
                &attributes,
                generic_thread_function::<T>,
                Box::leak(arg) as *mut GenericThreadArgument<T> as *mut libc::c_void,
            )
        };
        match result {
            0 => Ok(JoinHandle {
                handle: unsafe { handle.assume_init() },
                phantom: Default::default(),
            }),
            e => Err(ThreadingError::new(&format!(
                "Could not start thread: {}.",
                e
            ))),
        }
    }

    pub fn join(&self) -> ThreadingResult<T> {
        unsafe {
            let mut value = MaybeUninit::<*mut libc::c_void>::uninit();
            let result = libc::pthread_join(self.handle, value.as_mut_ptr());
            let value = value.assume_init() as *mut T;
            let value = Box::from_raw(value);
            match result {
                0 => Ok(*value),
                _ => Err(ThreadingError::new("Could not join thread")),
            }
        }
    }
}

impl SchedulerPolicy {
    fn libc_value(&self) -> libc::c_int {
        match self {
            SchedulerPolicy::Fifo => libc::SCHED_FIFO,
            SchedulerPolicy::RoundRobin => libc::SCHED_RR,
        }
    }
}

fn initialize_thread_attributes(
    attributes: ThreadAttributes,
) -> ThreadingResult<libc::pthread_attr_t> {
    let mut attr: MaybeUninit<libc::pthread_attr_t> = MaybeUninit::uninit();
    let result = unsafe { libc::pthread_attr_init(attr.as_mut_ptr()) };
    let mut attr = match result {
        0 => Ok(unsafe { attr.assume_init() }),
        e => Err(ThreadingError::new(&format!(
            "pthread_attr_init failed: {}",
            e
        ))),
    }?;

    let result = unsafe { libc::pthread_attr_setstacksize(&mut attr, attributes.stack_size) };
    if result != 0 {
        return Err(ThreadingError::new(&format!(
            "pthread_attr_setstacksize failed: {}",
            result
        )));
    }

    let result = unsafe {
        libc::pthread_attr_setschedpolicy(&mut attr, attributes.scheduler_policy.libc_value())
    };
    if result != 0 {
        return Err(ThreadingError::new(&format!(
            "pthread_attr_setschedpolicy failed: {}",
            result
        )));
    }

    let result =
        unsafe { libc::pthread_attr_setinheritsched(&mut attr, libc::PTHREAD_EXPLICIT_SCHED) };
    if result != 0 {
        return Err(ThreadingError::new(&format!(
            "pthread_attr_setschedpolicy failed: {}",
            result
        )));
    }

    let mut scheduler_parameter = MaybeUninit::<libc::sched_param>::uninit();
    let pscheduler_parameter = scheduler_parameter.as_mut_ptr();
    unsafe {
        addr_of_mut!((*pscheduler_parameter).sched_priority).write(attributes.thread_priority)
    };
    let scheduler_parameter = unsafe { scheduler_parameter.assume_init() };

    let result = unsafe { libc::pthread_attr_setschedparam(&mut attr, &scheduler_parameter) };
    if result != 0 {
        return Err(ThreadingError::new(&format!(
            "pthread_attr_setschedpolicy failed: {}",
            result
        )));
    }

    Ok(attr)
}

struct GenericThreadArgument<T> {
    main_function: Box<dyn FnOnce() -> T + Send + 'static>,
}

extern "C" fn generic_thread_function<T>(arg: *mut libc::c_void) -> *mut libc::c_void {
    let arg = unsafe { Box::from_raw(arg as *mut GenericThreadArgument<T>) };
    let result = Box::new((arg.main_function)());
    Box::leak(result) as *mut T as *mut libc::c_void
}

#[cfg(test)]
mod test {
    use crate::threading::Clock;

    use super::*;
    #[test]
    fn test_threading_functionality() {
        lock_memory().unwrap();
        let attributes = ThreadAttributes {
            stack_size: libc::PTHREAD_STACK_MIN,
            scheduler_policy: SchedulerPolicy::Fifo,
            thread_priority: 99,
        };
        let thread = JoinHandle::spawn(attributes, || {
            let mut clock = Clock::new().unwrap();
            clock.sleep(std::time::Duration::from_millis(50)).unwrap();
            123
        })
        .unwrap();
        let result = thread.join().unwrap();
        assert_eq!(123, result);
    }
}
