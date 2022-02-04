use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

fn main() {
    thread_spawn();
    channel_use();
    channel_multiple_msg_clone();
    mutex_use();
}

fn thread_spawn() {
    let handle = thread::spawn(||{
        for i in 0..10 {
            println!("hi number: {} from spawned thread", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in 0..3 {
        println!("hi number: {} from main thread", i);
        thread::sleep(Duration::from_millis(10));
    }
    handle.join().unwrap();

    let vec = vec!["a", "b", "c"];
    thread::spawn(move ||{
        println!("print vec from spawned thread: {:?}", vec);
    }).join().unwrap();
}

fn channel_use() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let msg = String::from("msg");
        thread::sleep(Duration::from_millis(100));
        //send would move ownership of msg
        tx.send(msg).unwrap();
    });

    //It'll block the main thread to wait for message
    let received = rx.recv().unwrap();
    //not block the main thread;
    // let received = rx.try_recv().unwrap();
    println!("Received message: {}", received);
}

fn channel_multiple_msg_clone() {
    let (tx, rx) = mpsc::channel();
    //Clone sender
    let tx1 = mpsc::Sender::clone(&tx); //tx.clone() //deep clone
    thread::spawn(move ||{
        let vec = vec!["msg1 from 1", "msg2 from 1", "msg3 from 2"];
        for msg in vec {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(move ||{
        let vec = vec!["msg1 from 2", "msg2 from 2", "msg3 from 2"];
        for msg in vec {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    //can iterate rx without explicit recv call
    for received in rx {
        println!("received {}", received);
    }
}
/// Arc<T> & Mutex<T> usage together is similar to Rc<T> & RefCell<T>, but can used in multiple threads cases
/// Mutex<T> provided internal mutable behaviour likes Cell<T> (RefCell)
/// Mutex<T> has risk of deadlock
///
fn mutex_use() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    println!("current counter: {}", *counter.lock().unwrap())
}
/// Sync & Send: std::marker::Sync, std::marker::Send
/// Any types implemented Send would move ownership and can be used in multiple threads
/// Most of the rust types have implemented Send trait, but Rc<T> doesn't
///
/// If types implemented Sync trait, can be referenced by multiple threads
/// All primitive types have implemented Sync, Rec<T> and Cell<T>/RefCell<T> are not synced, Mutex<T> is synced
/// Manual implementation of Send and Sync is not safe, need to be carefully used.