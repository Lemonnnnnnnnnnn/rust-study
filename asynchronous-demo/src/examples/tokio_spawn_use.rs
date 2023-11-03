use rand::Rng; // trait that must be in scope
use tokio::{
    task,
    time::{sleep, Duration},
};

// #[tokio::main(worker_threads = 4)]
pub async fn main() {
    let mut tasks = Vec::new();
    for i in 0..10 {
        // task::spawn requires a Future and using an async block provides this.
        tasks.push(task::spawn(async move {
            let ms = rand::thread_rng().gen_range(1000..3000);
            sleep(Duration::from_millis(ms)).await;
            (i, ms)
        }));
    }
    
    let mut res = Vec::new();

    for task in tasks {
        let (task_number, sleep_ms) = task.await.unwrap();
        res.push((sleep_ms, task_number));
    }

    println!("res: {:?}", res);

    println!("done");
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test() {
        main().await;
    }
}
