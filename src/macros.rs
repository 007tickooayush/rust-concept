#[cfg(test)]
pub mod macros {
    pub trait Bounded {
        fn min_value_get() -> Self;
        fn max_value_get() -> Self;
    }
    #[macro_export]
    macro_rules! bounded_impl {
        ($t:ty, $min:expr, $max:expr) => {
            impl Bounded for $t {
                #[inline]
                fn min_value_get() -> $t {
                    // println!("Min value is {}", $min);
                    $min
                }

                #[inline]
                fn max_value_get() -> $t {
                    // println!("Max value is {}", $max);
                    $max
                } 
            }
        };
    }

    #[macro_export]
    macro_rules! some_lang {
        ($t:ty, $val:expr) => {
            fn get_the_customVal() -> $t {
                $val
            }
        };
    }


    #[test]
    pub fn test_some_function() {
        some_lang!(i32,33);
        let val = get_the_customVal();
        println!("The value is {}", val);
    }

    #[test]
    pub fn test_some_function_str() {
        some_lang!(String,String::from("Hello World"));
        let val = get_the_customVal();
        println!("The value is {}", val);
    }
    
    #[test]
    pub fn test_macros() {
        bounded_impl!(i32, 0, 100);
        let i = i32::min_value_get();
        println!("Min value is {}", i);
    }

    #[test]
    fn test_bounded_i32() {
        assert_eq!(i32::min_value_get(), 0, "The minimum value should match i32::MIN");
        assert_eq!(i32::max_value_get(), 100, "The maximum value should match i32::MAX");
    }

    #[derive(Debug)]
    pub struct Percentage {
        pub value: i32,
    }
    impl Percentage {
        pub fn get_min() -> Self {
            Percentage { value: 0 }
        }

        pub fn get_max() -> Self {
            Percentage { value: 100 }
        }

        pub fn new(value: i32) -> Self {
            Percentage { value }
        }
    }

    #[test]
    pub fn test_my_type() {
        bounded_impl!(Percentage, Percentage::get_min(), Percentage::get_max());
        let p = Percentage::new(24);
        assert_eq!(p.value, 24);
        assert_eq!(Percentage::min_value_get().value, Percentage { value: 0 }.value);
        assert_eq!(Percentage::max_value_get().value, 100);
    }
}