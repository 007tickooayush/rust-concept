use std::sync::Mutex;
use closures::closures_mod::{generate_workout, test_move_keyword};
use collections::collections::test_collections;
use error_handling::{test_file_error, test_file_error_propagation};
use iterators::iterators::iterator_test;
use lifetimes::lifetimes_mod::test_vars_lifetime;
use smart_pointers::smart_pointers::{test_box, test_box_multiple};
use smart_pointers_drop::smart_pointers_drop::test_drop;
use smart_pointers_interior_mutability::smart_pointers_refCell::{test_with_ref, test_without_ref};
use smart_pointers_reference_counted::smart_pointers_rc::{test_reference_counted, test_without_rc};
use traits::traits_mod::{test_traits,notify,notify_2};
use types::types_mod::{get_largest, test_types};
use crate::concurrency_messages::concurrency_msg::{test_mpsc, test_multiple_tx};
use crate::concurrency_state::concurrency_shred_state::{test_mutex, test_mutex_mark2};
use crate::concurrency_threads::concurrency::{test_thread_main_vs_spawn_thread_1, test_thread_main_vs_spawn_thread_2, test_thread_main_vs_spawn_thread_3, test_thread_move};
use crate::object_oriented::object_oriented_oop::test_average_collection;
use crate::smart_pointers_interior_mem_safe::interior_mutability_mem_safe::test_stack_interior;

mod collections;
mod enums;
mod error_handling;
mod rectangle;
mod types;
mod traits;
mod lifetimes;
mod closures;
mod iterators;
mod smart_pointers;
mod smart_pointers_deref;
mod smart_pointers_drop;
mod smart_pointers_reference_counted;
mod smart_pointers_interior_mutability;
mod smart_pointers_interior_mem_safe;
mod concurrency_threads;
mod concurrency_messages;
mod concurrency_state;
mod object_oriented;
mod inheritance;
mod state_design_pattern;


fn main() {
    test_average_collection();

    // test_mutex_mark2();
    // test_mutex();
    // test_multiple_tx();
    // test_mpsc();
    // test_thread_move();
    // test_thread_main_vs_spawn_thread_2();
    // test_stack_interior();
    // test_without_ref();
    // test_with_ref();
    // test_without_rc();
    // test_reference_counted();
    // test_without_rc();
    // test_drop();
    // test_box();
    // test_box_multiple();
    // iterator_test();
    // generate_workout(7, 2);
    // test_move_keyword();
    // test_vars_lifetime();
    // test_traits();
    // test_types();
    // test_rectangle();
    // test_collections();

    // let _ = test_file_error_propagation();
    // test_file_error();
}
