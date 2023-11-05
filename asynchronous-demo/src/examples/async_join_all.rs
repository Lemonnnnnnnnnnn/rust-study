use std::time::Duration;
use futures::future::join_all;
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct A {}

impl A {
    pub async fn make(&self , num : i32) -> i32 {
        sleep(Duration::from_secs(1)).await;
        num * 2
    }
}

#[tokio::test]
async fn test() {
    let v = vec![A{} , A{}];

    let futs : Vec<_> = v.iter().map(|a|{
        a.make(1)
    }).collect();

    let res = join_all(futs).await;

    println!("{:?}", res);
}