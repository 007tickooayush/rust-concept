pub mod inheritance_demo {
    use std::error::Error;
    use std::io;

   pub trait AddNumbers<I> {
       type Item;

       fn add_all(&self) -> Result<I, io::Error>;
   }

    pub trait SubtractFrom<T> {
        type Item;

        fn subtract_from(&self) ;
    }

    pub struct AddI32<Nu> {
        numbers: Vec<Nu>
    }

    pub struct SubtractI32<Nu> {
        numbers: Vec<Nu>
    }

    impl SubtractFrom<i32> for SubtractI32<i32> {
        type Item = i32;

        fn subtract_from(&self) {
            todo!()
        }
    }
    impl AddNumbers<i32> for AddI32<i32> {
        type Item = i32;

        fn add_all(&self) -> Result<i32, io::Error> {
            // let mut sum = 0;
            // for num in self.numbers {
            //     sum += num;
            // }
            // return Ok(sum);

            Ok(self.numbers.iter().sum())
        }
    }

    #[test]
    pub fn test_addition() {
        let add = AddI32 { numbers: vec![1,2,3,4,5] };
        let result = add.add_all().unwrap();
        assert_eq!(result, 15);
    }

}

// correct impl for the AddNumbers trait provide <i32>
// pub mod inheritance_demo {
//     pub struct Number<T> {
//         num: T
//     }
//
//     impl Number<i32> {
//
//         pub fn new(num: i32) -> Number<i32> {
//             Number { num }
//         }
//     }
//
//
// }