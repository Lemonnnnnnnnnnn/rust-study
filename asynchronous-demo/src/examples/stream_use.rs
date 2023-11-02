use futures::stream::Stream;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

type CrawlRequest = Pin<Box<dyn Future<Output = i32>>>;

pub struct MyStream {
    pub queued_results: Vec<i32>,
    pub in_progress_crawl_requests: Vec<CrawlRequest>,
}

impl Stream for MyStream {
    type Item = i32;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("running poll_next...");
        let pin = self.get_mut();

        // for n in 0 .. pin.in_progress_crawl_requests.len() {
        //     let mut request = pin.in_progress_crawl_requests.swap_remove(n);
        //     if let Poll::Ready(res) = request.as_mut().poll(cx) {
        //         println!("get_value");
        //         pin.queued_results.push(res);
        //         return Poll::Ready(Some(res))
        //     }else{
        //         pin.in_progress_crawl_requests.push(request);
        //     }
        // }

        // if pin.in_progress_crawl_requests.len() == 0 {
        //     return Poll::Ready(None);
        // }else{
        //     return Poll::Pending;
        // }

        match pin.in_progress_crawl_requests.pop() {
            Some(mut request) => {
                if let Poll::Ready(res) = request.as_mut().poll(cx) {
                    pin.queued_results.push(res);

                    return Poll::Ready(Some(res));
                } else {
                    pin.in_progress_crawl_requests.push(request);
                    return Poll::Pending;
                }
            }
            None => {
                return Poll::Ready(None);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::StreamExt;
    use tokio::time::sleep;
    use tokio::time::Duration;

    async fn task(num: i32) -> i32 {
        sleep(Duration::from_secs(1)).await;
        num
    }

    #[tokio::test]
    async fn test() {
        let fut1 = task(1);
        let fut2 = task(2);

        let stream = MyStream {
            queued_results: vec![],
            in_progress_crawl_requests: vec![Box::pin(fut1), Box::pin(fut2)],
        };

        const MAX_CONCURRENT_JUMPERS: usize = 100;


        stream.for_each_concurrent(MAX_CONCURRENT_JUMPERS , |x| async move { 
            println!("{}", x);
        }).await;

        // println!("{:#?}", stream.queued_results);
    }
}
