use futures::join;
use futures::executor::block_on;

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
    drink_water().await;
} 

async fn plan(){
    let future1 = boil_and_drink();
    let future2 = play_game();

    join!(future1, future2);
}

pub fn run(){
    let future = plan();
    block_on(future);
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