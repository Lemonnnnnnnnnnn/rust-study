use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::time::{sleep, Duration };
use tokio::task;

async fn run_task(num: i32) -> i32 {
    sleep(Duration::from_secs(1)).await;
    num
}

async fn main()-> Vec<i32> 
{
    const RATE_LIMIT: u32 = 1;
    let governor = Arc::new(RateLimiter::direct(Quota::per_second(
        NonZeroU32::new(RATE_LIMIT).unwrap(),
    )));

    let mut tasks = Vec::new();

    for i in 0..10 {
        let f = run_task(i);
        tasks.push(task::spawn(f));
    }

    let mut res = Vec::new();

    for task in tasks {
        let g = governor.clone();
        g.until_ready().await;
        let n = task.await.unwrap();
        res.push(n);
    }

    res

}



#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let res = main().await;
        println!("res: {:?}", res);
    }
}
