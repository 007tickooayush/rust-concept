
pub mod iterators {

    #[derive(Debug)]
    pub struct Counter {
        pub count : u32
    }

    impl Counter{
        pub fn new() -> Counter {
            Counter { count: 0}
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {

            if self.count < 5 {
                self.count += 1;

                Some(self.count) // return the count until the value is less than 5
            } else {
                None // after the condition return None
            }
        }
    }

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

    }   

    pub fn iterator_test() {
        let v1 = vec![1,2,23];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got : {}",val);
        }

        println!("{:?}",iterator_inc_test(vec![10,20,30,40]));
    }

    pub fn iterator_inc_test(list: Vec<u32>) -> Vec<u32>{
        list.iter().map(|x| x+1).collect()
    }

    #[cfg(test)]
    pub mod test_iterators{
        use crate::iterators::{iterators::Iterator};

        use super::Counter;


        #[test]
        pub fn iterator_demonstration(){
            let v1 = vec![1,2,3,4,5,6];

            let mut v1_iter = v1.iter();

            assert_eq!(v1_iter.next(),Some(&1));
            assert_eq!(v1_iter.next(),Some(&2));
            assert_eq!(v1_iter.next(),Some(&3));
            assert_eq!(v1_iter.next(),Some(&4));
            assert_eq!(v1_iter.next(),Some(&5));
            assert_eq!(v1_iter.next(),Some(&6));
            assert_eq!(v1_iter.next(),None);
        }

        #[test]
        pub fn iterator_mut_demonstration(){
            let mut v1 = vec![1,2,3,4,5,6];

            let mut v1_iter = v1.iter_mut();

            assert_eq!(v1_iter.next(),Some(&mut 1));
            assert_eq!(v1_iter.next(),Some(&mut 2));
            assert_eq!(v1_iter.next(),Some(&mut 3));
            assert_eq!(v1_iter.next(),Some(&mut 4));
            assert_eq!(v1_iter.next(),Some(&mut 5));
            assert_eq!(v1_iter.next(),Some(&mut 6));
            assert_eq!(v1_iter.next(),None);
        }

        #[test]
        pub fn iterator_into_demonstration() {
            let v1 = vec![1,2,3,4,5,6];

            let mut v1_iter = v1.into_iter();

            assert_eq!(v1_iter.next(),Some(1));
            assert_eq!(v1_iter.next(),Some(2));
            assert_eq!(v1_iter.next(),Some(3));
            assert_eq!(v1_iter.next(),Some(4));
            assert_eq!(v1_iter.next(),Some(5));
            assert_eq!(v1_iter.next(),Some(6));
            assert_eq!(v1_iter.next(),None);
        }

        #[test]
        pub fn iterator_sum(){
            let v1 = vec![1,2,3,4,5,6];

            let v1_iter = v1.into_iter();

            let total: u32 = v1_iter.sum();
            assert_eq!(total,21);
        }

        #[test]
        pub fn test_counter_direct(){
            let mut counter = Counter::new();

            assert_eq!(counter.next(),Some(1));
            assert_eq!(counter.next(),Some(2));
            assert_eq!(counter.next(),Some(3));
            assert_eq!(counter.next(),Some(4));
            assert_eq!(counter.next(),Some(5));
            assert_eq!(counter.next(),None);
            
        }

        // #[test]
        // pub fn test_counter_methods(){
        //     let sum:u32 = Counter::new()
        //     .zip(Counter::new().skip(1))
        //     .map(|(a,b)| a*b)
        //     .filter(|x| X%3 == 0)
        //     .sum()
        //     ;
        // }

    }
}