use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Language
{
    CPP,
    C,
    Rust,
    Java,
    Python
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
