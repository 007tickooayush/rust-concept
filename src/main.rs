
use closures::closures_mod::generate_workout;
use collections::collections::test_collections;
use error_handling::{test_file_error, test_file_error_propagation};
use lifetimes::lifetimes_mod::test_vars_lifetime;
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


fn main() {

    generate_workout(7, 2);
    // test_vars_lifetime();
    // test_traits();
    // test_types();
    // test_rectangle();
    // test_collections();

    // let _ = test_file_error_propagation();
    // test_file_error();
}
