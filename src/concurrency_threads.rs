pub mod concurrency {
    use std::thread;
    use std::time::Duration;

    // small details such as where the join function is called can affect the threads execution
    pub fn test_thread_main_vs_spawn_thread_1() {
        /// THREADS RUN ALTERNATIVELY UNTIL THE MAIN THREAD EXECUTION IS STOPPED
        thread::spawn(|| {
           for i in  0..10 {
                println!("THREAD SPAWN NO.: {}",i);
               thread::sleep(Duration::from_millis(1));
            }
        });

        // calling the main thread
        for i in 0..5 {
            println!("THREAD MAIN NO.: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn test_thread_main_vs_spawn_thread_2() {
        /// THREADS RUN ALTERNATIVELY UNTIL THE MAIN THREAD EXECUTION
        /// AND AFTER THAT THE SPAWN THREAD EXECUTION IS CONTINUED
        let handle = thread::spawn(|| {
            for i in  0..10 {
                println!("THREAD SPAWN NO.: {}",i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // calling the main thread
        for i in 0..5 {
            println!("THREAD MAIN NO.: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }

    pub fn test_thread_main_vs_spawn_thread_3() {
        /// THE SPAWN THREAD EXECUTION IS FINISHED FIRST
        /// AFTER THAT MAIN THREAD EXECUTION IS STARTED
        let handle = thread::spawn(|| {
            for i in  0..10 {
                println!("THREAD SPAWN NO.: {}",i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();
        // calling the main thread
        for i in 0..5 {
            println!("THREAD MAIN NO.: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn test_thread_move () {
        /// FORCE THE OWNERSHIP OF AN OUTER VARIABLE TO THE CLOSURE
        ///
        let v = vec![1,2,3];

        let handle = thread::spawn(move || { /// error without 'move' keyword
            println!("Vector v: {:?}",v)
        });

        handle.join().unwrap();
    }
}