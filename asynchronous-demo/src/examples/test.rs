#[derive(Clone)]
pub struct A {}

pub struct B {
    pub name: String,
}

impl A {
    pub async fn make_b(&self) -> i32 {
        1
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;

    #[tokio::test]
    async fn test() {
        let a = Arc::new(A {});
        let a2 = Arc::new(A {});
        let v = vec![a, a2];

        // let b: Vec<_> = v.into_iter().map(|a| async { a.make_b() }).collect();
        let b: Vec<_> = v.into_iter().map(|a| {
            let a_clone = Arc::clone(&a);
            async move {
                a_clone.make_b().await
            }
            }).collect::<Vec<_>>();

        for i in b {
            let aa = i.await;
            println!("{}", aa);
        }

    }
}
