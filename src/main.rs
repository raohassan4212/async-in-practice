use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use futures::join;

fn main() {
    println!("Hello, world!");
    get_two_site();

    block_on(get_two_site_with_async());

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
    join!(future2,future1);
}

async fn task_one() {
    thread::sleep(Duration::from_secs(5));
        println!("Task One");
}

async fn task_two() {
    thread::sleep(Duration::from_secs(2));
        println!("Task Two");
}