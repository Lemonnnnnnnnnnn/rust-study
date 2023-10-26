use futures::executor::block_on;

async fn hello_async() -> i8{
    println!("Hello, world!");
    1
}

pub fn run(){
    let future = hello_async();

    let res = block_on(future);

    println!("执行结果: {}", res);
    println!("执行完毕!");
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