//! # learningrust
//!
//! 'learningrust' is me doing random hacking while learning rust

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
pub mod patterns_chapter;
pub mod threadpool;
pub mod unsafety;
pub mod adv_lifetimes;
pub mod prime;
pub mod adv_traits;
pub mod outline_print;

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

    // state_pattern::run();
    // state_pattern_rust::run();

    // patterns_chapter::run();

    // threadpool::run();

    // unsafety::run();

    // adv_lifetimes::run();

    // prime::run();

    adv_traits::run();
}
