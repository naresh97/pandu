mod threading;

fn main() {
    threading::lock_memory().unwrap();
    let attributes = threading::ThreadAttributes {
        stack_size: libc::PTHREAD_STACK_MIN,
        scheduler_policy: threading::SchedulerPolicy::Fifo,
        thread_priority: 99,
    };
    let thread = threading::JoinHandle::spawn(attributes, || {
        let mut clock = threading::Clock::new().unwrap();
        loop {
            println!("Hello from lambda!");
            clock.sleep(std::time::Duration::from_millis(1000)).unwrap();
        }
    })
    .unwrap();
    thread.join().unwrap();
}
