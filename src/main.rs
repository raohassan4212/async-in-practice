use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use futures::join;

fn main() {
    println!("Hello, world!");
    get_two_site();

    block_on(get_two_site_with_async());
    block_on(async_main());

}

// multithreading
fn get_two_site() {
    let thread1 = std::thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("Thread One")
    });

    let thread2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        println!("Thread two")
    });

    thread1.join();
    thread2.join();
}

// async
async fn get_two_site_with_async() {
    let future1 = task_one();
    let future2 = task_two();
    join!(future1,future2);
}

async fn task_one() {
    thread::sleep(Duration::from_secs(5));
        println!("Task One");
}

async fn task_two() {
    thread::sleep(Duration::from_secs(2));
        println!("Task Two");
}

// Learn_Sing and Dancing examlpe

async fn learn_song() -> String {
   thread::sleep(Duration::from_secs(5));
   "taray zameen per".to_string()
}

async fn sing_song(song: String)  {
    println!("{}",song);
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("Dancing");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    
    join!(f1,f2);
}