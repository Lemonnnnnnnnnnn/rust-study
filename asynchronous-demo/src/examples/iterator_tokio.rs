use std::{time::Duration, sync::Arc};
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
    let r1 = Request{};
    let r2 = Request{};
    let v = vec![r1 , r2];

    let requests : Vec<_> = v.into_iter().map(Arc::new).collect();

    let futs : Vec<_> = requests.iter().map(|req|{
        let req_clone = Arc::clone(req);
        tokio::spawn(async move{
            let f  = req_clone.do_req(1).await; 
            f
        }) 
    }).collect();

    let res = join_all(futs).await;

    println!("{:?}", res);
}