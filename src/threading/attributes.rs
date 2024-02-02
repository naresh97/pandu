pub struct ThreadAttributes {
    pub stack_size: usize,
    pub scheduler_policy: SchedulerPolicy,
    pub thread_priority: i32,
}

#[allow(dead_code)]
pub enum SchedulerPolicy {
    Fifo,
    RoundRobin,
}
