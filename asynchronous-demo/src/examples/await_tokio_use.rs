use tokio::time::{sleep , Duration};
use tokio::join;
use tokio::runtime::Runtime;

async fn boil_water(){
    println!("boil water");
}

async fn drink_water(){
    println!("drink water");
}

async fn play_game(){
    println!("play game");
}

async fn boil_and_drink(){
    boil_water().await;
    sleep(Duration::from_millis(1000)).await;
    drink_water().await;
} 

async fn plan(){
    let future1 = boil_and_drink();
    let future2 = play_game();

    join!(future1, future2);
}

pub fn run(){
    let future = plan();
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    #[ignore]
    fn test(){
        run();
    }
}