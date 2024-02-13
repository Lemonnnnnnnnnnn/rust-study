// impl Default
#[derive(Debug, PartialEq)]
pub enum D {
    Foo,
    Bar,
}

impl Default for D {
    fn default() -> Self {
        D::Foo
    }
}

// derive Default
#[derive(Default, Debug, PartialEq)]
pub enum D2 {
    Foo,
    #[default]
    Bar,
}

// Default::default
pub fn some_function(d: D) -> D {
    d
}

// stuct default
#[derive(PartialEq, Debug)]
pub struct SD {
    pub width: u32,
    pub height: u32,
}

impl Default for SD {
    fn default() -> Self {
        Self {
            width: 12,
            height: 24,
        }
    }
}

// Default::default with function
pub fn some_function2(sd: SD) -> SD {
    sd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(D::default(), D::Foo);
        assert_eq!(D2::default(), D2::Bar);
        assert_eq!(some_function(Default::default()), D::Foo);
        assert_eq!(
            SD::default(),
            SD {
                width: 12,
                height: 24
            }
        );
        assert_eq!(
            some_function2(SD {
                width: 24,
                ..Default::default()
            }),
            SD {
                width: 24,
                height: 24
            }
        )
    }
}
