pub fn message() -> &'static str {
    "World!"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_message() {
        assert_eq!(message(), "World!");
    }
}
