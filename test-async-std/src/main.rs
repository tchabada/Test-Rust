use async_std::{sync::Mutex, task::sleep};
use futures::future::{join, join_all};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

async fn async_test1(input: &str) {
    sleep(Duration::from_secs(1)).await;
    println!("{}", input);
}

async fn async_test2(input: &str) -> Result<String, ()> {
    sleep(Duration::from_secs(1)).await;
    Ok(String::from(input))
}

async fn async_test3(input: u32) -> String {
    sleep(Duration::from_secs(1)).await;
    input.to_string()
}

async fn async_test4(input: u32, output: Arc<Mutex<Vec<String>>>) {
    sleep(Duration::from_secs(1)).await;
    output.lock().await.push(input.to_string());
}

#[async_std::main]
async fn main() {
    let now = Instant::now();

    async_test1("test1").await;
    println!("{} ms", now.elapsed().as_millis());

    join(async_test1("test2"), async_test1("test2")).await;
    println!("{} ms", now.elapsed().as_millis());

    println!("{}", async_test2("test3").await.unwrap());
    println!("{} ms", now.elapsed().as_millis());

    let numbers = vec![1, 2, 3];

    for n in numbers.iter() {
        println!("{}", async_test3(*n).await);
    }
    println!("{} ms", now.elapsed().as_millis());

    let result = join_all(numbers.iter().map(|n| async_test3(*n))).await;
    println!("{:?}", result);
    println!("{} ms", now.elapsed().as_millis());

    let result: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    join_all(numbers.iter().map(|n| async_test4(*n, result.clone()))).await;
    println!("{:?}", result.lock().await);
    println!("{} ms", now.elapsed().as_millis());
}
