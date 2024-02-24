pub struct Student<'a> {
    pub name: &'a str,
    pub age: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUDENTS: [Student; 2] = [
        Student {
            name: "xiaoming",
            age: "18",
        },
        Student {
            name: "xiaohong",
            age: "19",
        },
    ];

    #[test]
    fn find_xiao_hong() {
        assert!(STUDENTS.iter().any(|x| x.name == "xiaohong"));
    }

    #[test]
    fn find_xiao_zhang() {
        assert!(!STUDENTS.iter().any(|x| x.name == "xiaozhang"))
    }
}
