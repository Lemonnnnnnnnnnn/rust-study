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
// cannot return value referencing function parameter `a`
// returns a value referencing data owned by the current function
// -----
// #[tokio::test]
// async fn test() {
//     let v = vec![A{} , A{}];
//     let futs : Vec<_> = v.into_iter().map(|a|a.make(1)).collect(); 

//     // println!("{:?}", futs);
// }

// `&impl futures::Future<Output = i32>` is not a future
// the trait `futures::Future` is not implemented for `&impl futures::Future<Output = i32>`
// &impl futures::Future<Output = i32> must be a future or must implement `IntoFuture` to be awaited
//  -----
// #[tokio::test]
// async fn test() {
//     let v = vec![A{} , A{}];

//     let futs : Vec<_> = v.iter().map(|a|a.make(1)).collect(); 

//     let handlers : Vec<_> = futs.iter().map(|f| tokio::spawn(f)).collect();
// }


// #[tokio::test]
// async fn test() {
//     let v = vec![A{} , A{}];

//     let futs : Vec<_> = v.iter().map(|a|a.make(1)).collect(); 

//     let handlers : Vec<_> = futs.iter().map(|f| tokio::spawn(async move{f})).collect();
// }

