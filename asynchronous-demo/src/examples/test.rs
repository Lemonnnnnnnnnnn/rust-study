pub struct A {}

pub struct B {
    pub name : String
}

impl A {
    pub async fn make_b(&self) -> B {
        B {
            name : String::from("test")
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use super::*;

    #[tokio::test]
    async fn test() {
        let a = A {};
        let b = a.make_b();

        let rt = Runtime::new().unwrap();
        let b = rt.block_on(b);

        println!("{}", b.name);
    }
}
