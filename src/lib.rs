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
pub mod threads_shared_state;
pub mod state_pattern;
pub mod state_pattern_rust;

pub fn run() {
    // longest_string::run();
    // points::run();
    // closures::run();
    // smart_pointers::run();
    // limit_tracker::run();
    // rc_refcell::run();
    // threads_msg_passing::run();
    // spring::run();
    // threads_shared_state::run();
    state_pattern::run();

    state_pattern_rust::run();
}
