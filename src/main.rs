use error_handling::{test_file_error, test_file_error_propagation};

mod rectangle;
mod enums;
mod collections;
mod error_handling;

fn main() {
    // test_rectangle();
    // let _ = test_file_error_propagation();
    test_file_error();
}
