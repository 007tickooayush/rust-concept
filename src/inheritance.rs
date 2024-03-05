pub mod inheritance_demo {
    use std::error::Error;
    use std::io;

   pub trait AddNumbers<I> {
       type Item;

       fn add_all(&self) -> Result<I, io::Error>;
   }

    pub trait SubtractFrom<T> {
        type Item;

        fn subtract_from(&self) -> Result<T, Self::Item> ;
    }

    pub struct AddI32<Nu> {
        numbers: Vec<Nu>
    }

    pub struct SubtractI32<Nu> {
        numbers: Vec<Nu>
    }

    impl SubtractFrom<i32> for SubtractI32<i32> {
        type Item = i32;

        fn subtract_from(&self) -> Result<i32, Self::Item> {
            let first = self.numbers[0];
            let mut sum =0;
            for num in &self.numbers[1..] {
                sum += num;
            }
            Ok(first - sum)
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

    #[test]
    pub fn test_subtraction() {
        let sub = SubtractI32 { numbers: vec![10,5,3] };
        let result = sub.subtract_from().unwrap().checked_abs().unwrap();
        assert_eq!(result, 2);
    }

}
