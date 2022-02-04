use std::thread;
use std::time::Duration;

// fn main() {
//
// }

#[tokio::main]
async fn main() {
    sing_a_song().await;
    dance().await;
}

async fn sing_a_song() {
    println!("Sing a song now!");
    thread::sleep(Duration::from_millis(1000));
}

async fn dance() {
    println!("Dancing");
    thread::sleep(Duration::from_millis(1000));
}