use std::time::Duration;
use futures::future::join_all;
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct Request {}

impl Request {
    pub async fn do_req(&self , num : i32) -> i32 {
        sleep(Duration::from_secs(1)).await;
        num * 2
    }
}

#[tokio::test]
async fn test() {
    let v = vec![Request{} , Request{}];

    let futs : Vec<_> = v.iter().map(|req|{
        req.do_req(1)
    }).collect();

    let res = join_all(futs).await;

    println!("{:?}", res);
}