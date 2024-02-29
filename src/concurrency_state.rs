pub mod concurrency_shred_state {
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// MUTUAL EXCLUSION USING MUTEX SMART POINTER
    /// FOR SHARING MEMORY AMONG THREADS
    pub fn test_mutex() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 60;
        } // DROP IMPLEMENTATION AUTOMATICALLY CALLED WHEN THE VARIABLE IS OUT OF SCOPE

        println!("m = {:?}",m);
    }

    pub fn test_mutex_mark2 () {
        /// ERROR PRONE CODE WITHOUT REFERENCE COUNTED SMART POINTER
        // let counter = Mutex::new(0);
        //
        // let mut handles = vec![];
        //
        // for i in 1..10 {
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //
        //         *num += 1;
        //     });
        //
        //     handles.push(handle);
        // }

        /// DIFFERENT ERROR REGARDING SAFETY
        // let counter = Rc::new(Mutex::new(0));
        //
        // let mut handles = vec![];
        //
        // for i in 1..10 {
        //     let counter = Rc::clone(&counter);
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }

        /// PERFORMING THE OPERATION ATOMICALLY USING Arc SMART POINTER
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 1..10 {
            let counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("RESULT= {}",*counter.lock().unwrap());
    }

}