use error_handling::{test_file_error, test_file_error_propagation};
use traits::traits_mod::{test_traits,notify,notify_2};
use types::types_mod::{get_largest, test_types};

mod collections;
mod enums;
mod error_handling;
mod rectangle;
mod types;
mod traits;

fn main() {
    test_traits();
    // test_types();
    // test_rectangle();
    // let _ = test_file_error_propagation();
    // test_file_error();
}
