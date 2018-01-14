pub mod generic_fn;
pub mod closures;
pub mod iterators;
pub mod smart_pointers;
pub mod points;
pub mod limit_tracker;
pub mod longest_string;
pub mod rc_refcell;
pub mod ref_cycles;
pub mod threads_msg_passing;
pub mod spring;

pub fn run() {
    // longest_string::run();
    // points::run();
    // closures::run();
    // smart_pointers::run();
    // limit_tracker::run();
    // rc_refcell::run();
    // threads_msg_passing::run();
    spring::run();
}
