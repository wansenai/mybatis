pub mod tests {
    pub use std::thread;
    pub use std::time::Duration;
    pub use std::sync::{
        mpsc,
        Mutex,
        Arc,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_spawn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("闭包 thread: {}", i);
                thread::sleep(Duration::from_millis(1))
            }
        });

        for j in 1..5 {
            println!("other thread: {}", j);
            thread::sleep(Duration::from_millis(1))
        }
    }
    

    #[test]
    fn test_join_spawn() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("闭包 thread: {}", i);
                thread::sleep(Duration::from_millis(1))
            }
        });
        

        for j in 1..5 {
            println!("other thread: {}", j);
            thread::sleep(Duration::from_millis(1))
        }

        handle.join().unwrap();
    }

    #[test]
    fn test_move_closure_spawn() {
        let v = vec![6, 7, 9];

        let handle = thread::spawn(move || {
            println!("{:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn test_mpsc_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(String::from("rust :)")).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("{:?}", received);
    }

    #[test]
    fn test_mpsc_channel_two() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let send_data = vec![String::from("Hello"), String::from("James"), String::from("Zow")];
            for val in send_data {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("{:?}", received);
        }
    }

    #[test]
    fn test_mpsc_channel_three() {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let send_data = vec![String::from("Hello"), String::from("James"), String::from("Zow")];
            for val in send_data {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let send_data = vec![String::from("How"), String::from("are"), String::from("you")];
            for val in send_data {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("{:?}", received);
        }
    }

    #[test]
    fn test_single_thread_mutex() {
        let n1 = Mutex::new(5);

        {
            let mut n2 = n1.lock().unwrap();
            *n2 = 6;
        }

        println!("n1 = {:?}", n1);
    }

    #[test]
    fn test_multiple_thread_mutex() {
        let count = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&count);

            let handle = thread::spawn(move || {
                let mut num =counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("result count : {}", *count.lock().unwrap());
    }
}
