
use closures::closures_mod::{generate_workout, test_move_keyword};
use collections::collections::test_collections;
use error_handling::{test_file_error, test_file_error_propagation};
use iterators::iterators::iterator_test;
use lifetimes::lifetimes_mod::test_vars_lifetime;
use smart_pointers::smart_pointers::{test_box, test_box_multiple};
use smart_pointers_drop::smart_pointers_drop::test_drop;
use smart_pointers_interior_mutabiity::smart_pointers_refCell::{test_with_ref, test_without_ref};
use smart_pointers_reference_counted::smart_pointers_rc::{test_reference_counted, test_without_rc};
use traits::traits_mod::{test_traits,notify,notify_2};
use types::types_mod::{get_largest, test_types};

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
mod smart_pointers_interior_mutabiity;
fn main() {
    test_without_ref();
    test_with_ref();

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
