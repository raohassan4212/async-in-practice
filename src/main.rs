// use std::thread;
// use std::time::Duration;
// use futures::executor::block_on;
// use futures::join;
// use async_std::task;

// fn main() {
//     // println!("Hello, world!");
//     // get_two_site();

//     // block_on(get_two_site_with_async());
//     // block_on(async_main());
//     // calling negate
//     block_on(f());

// }

// // multithreading
// fn get_two_site() {
//     let thread1 = std::thread::spawn(|| {
//         thread::sleep(Duration::from_secs(5));
//         println!("Thread One")
//     });

//     let thread2 = thread::spawn(|| {
//         thread::sleep(Duration::from_secs(5));
//         println!("Thread two")
//     });

//     thread1.join();
//     thread2.join();
// }

// // async
// async fn get_two_site_with_async() {
//     let future1 = task_one("task one");
//     let future2 = task_two("task two");
//     join!(future1,future2);
// }

// async fn task_one(text: &str) {
//     thread::sleep(Duration::from_secs(5));
//         println!("{}",text);
// }

// async fn task_two(text: &str) {
//     thread::sleep(Duration::from_secs(2));
//         println!("{}",text);
// }

// // Learn_Sing and Dancing examlpe

// async fn learn_song() -> String {
//    thread::sleep(Duration::from_secs(5));
//    "taray zameen per".to_string()
// }

// async fn sing_song(song: String)  {
//     println!("{}",song);
// }

// async fn learn_and_sing() {
//     let song = learn_song().await;
//     sing_song(song).await;
// }

// async fn dance() {
//     println!("Dancing");
// }

// async fn async_main() {
//     let f1 = learn_and_sing();
//     let f2 = dance();
    
//     join!(f1,f2);
// }

// // Negate example

// async fn negate_async(n: i32) -> i32 {
//     println!("Negate Value is: {}",n);
//     task::sleep(Duration::from_secs(2)).await;
//     println!("Negating is finished: {}!",n);
//     n * -1
// }

// async fn f() -> i32 {
//     let negate = negate_async(1);
//     let negate_task = task::spawn(negate_async(2));
//     task::sleep(Duration::from_secs(1)).await;
//     negate.await + negate_task.await

// }

// use std::thread;
// use std::time::Duration;
// use async_std::task;
// use futures::executor::block_on;
// use futures::join;


// mod file;

// fn main () {
//     block_on(load_data());
// }

// async fn load_f1() {
//     let r1 = file::read_file("src/t1.txt").await;
//     println!("File 1 {}",r1.unwrap().len());
// }

// async fn load_f2() {
//     let r2 = file::read_file("src/t2.txt").await;
//     println!("File 2 {}",r2.unwrap().len());
// }

// async fn load_data() {
//     join!(load_f1(),load_f2()); 
// }

use std::time::Duration;
use std::thread;
use async_std::task;
use futures::executor::block_on;
use futures::join;

mod file;

fn main() {
    block_on(load_file());
}

async fn load1() {
    let r1 = file::read_file("src/t1.txt").await;
    println!("Result of file 1 {}",r1.unwrap().len());
}

async fn load2() {
    let r2 = file::read_file("src/t2.txt").await;
    println!("Result of file 2 {}",r2.unwrap().len());
}

async fn load_file() {
    join!(load1(),load2());
}