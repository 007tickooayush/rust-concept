pub mod closures_mod{
    use std::{thread, time::Duration};

    pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("Expensive operation");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    // // Calling the expensive calculation function here multiple times unneccessarily
    // pub fn generate_workout(intensity: u32, random_number: u32){
    //     if intensity < 25 {
    //         println!(
    //             "Today, do {} pushups!",
    //             simulated_expensive_calculation(intensity)
    //         );

    //         println!(
    //             "Today, do {} situps!",
    //             simulated_expensive_calculation(intensity)
    //         );
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today!")
    //         } else {
    //             println!(
    //                 "Today run for {} minutes!",
    //                 simulated_expensive_calculation(intensity)
    //             );
    //         }
    //     }
    // }

    // using memoization approach to address the mulitple trigger for the expensive function
    struct Cacher<T>
    where 
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T:Fn(u32) -> u32,
    {
        fn new(calculation:T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        // create a key value pair for "arg":"value" for each value
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    // // optimized calling
    // pub fn generate_workout(intensity: u32, random_number: u32){
    //     // simple initialization gets called every time regardless of the conditions  e.g if intensity > 25 and random_number == 3, where we dont need to
    //     // let expensive_result = simulated_expensive_calculation(intensity);

    //     // using closures to call the function conditionally
    //     let expensive_closure = |num| {
    //         println!("Expensive operation!");
    //         thread::sleep(Duration::from_secs(2));
    //         num
    //     };
    //     if intensity < 25 {
    //         println!(
    //             "Today, do {} pushups!",
    //             // expensive_result
    //             expensive_closure(intensity)
    //         );

    //         println!(
    //             "Today, do {} situps!",
    //             // expensive_result
    //             expensive_closure(intensity)
    //         );
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today!")
    //         } else {
    //             println!(
    //                 "Today run for {} minutes!",
    //                 // expensive_result
    //                 expensive_closure(intensity)
    //             );
    //         }
    //     }
    // }

    // more optimized using Cacher struct for memoization along with closure
    pub fn generate_workout(intensity: u32, random_number: u32){
        
        // using closures to call the function conditionally along with Cacher memoization
        let mut cached_result = Cacher::new(|num| {
            println!("Expensive operation!");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                // expensive_result
                cached_result.value(intensity)
            );

            println!(
                "Today, do {} situps!",
                // expensive_result
                cached_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today!")
            } else {
                println!(
                    "Today run for {} minutes!",
                    // expensive_result
                    cached_result.value(intensity)
                );
            }
        }
    }

    pub fn test_move_keyword(){

        // ownership of x is not moved
        {
            let x  = 123123;
    
            // ownership of x is not moved
            let equal_to_x = | z | z == x;
            println!("value of x is {}",x);

            let y = 123123;

    
            assert!(equal_to_x(y));
        }

        {
            let x = 321321;
    
            // ownership of x is moved using move keyword
            let equal_to_x = | z | z == x;
            // can not use the variable now afterward (unless new variable is created by cloning it)
    
            let y = 321321;

            // error prone line
            // println!("value of x is {}",x);
    
            assert!(equal_to_x(y));
        }
    }
}
