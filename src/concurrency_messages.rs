pub mod concurrency_msg {
use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn test_mpsc() {
        let (tx,rx) = mpsc::channel();

        thread::spawn(move || {
            let msg = String::from("New message from thread");
            tx.send(msg).unwrap();
        });

    //     RECEIVE THE MESSAGE
    //     try_recv : DOES NOT BLOCK THE EXECUTION
    //     recv     : BLOCKS THE EXECUTION
        let receive = rx.recv().unwrap();
        println!("RECEIVED: {}",receive);
    }

    pub fn test_multiple_tx() {
        let (tx,rx) = mpsc::channel();
        let tx2 = tx.clone();

        thread::spawn(move || {
           let messages = vec![
               String::from("message 1"),
               String::from("message 2"),
               String::from("message 3"),
               String::from("message 4")
           ];

            for msg in messages {
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let messages = vec![
                String::from("message 1 FROM THREAD 2"),
                String::from("message 2 FROM THREAD 2"),
                String::from("message 3 FROM THREAD 2"),
                String::from("message 4 FROM THREAD 2")
            ];

            for msg in messages {
                tx2.send(msg).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("MESSAGE RECEIVED: {}",received);
        }
    }
}
