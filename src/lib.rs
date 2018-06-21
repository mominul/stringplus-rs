pub trait StringPlus {
    fn substring(&self, start: usize, length: usize) -> &str;
    fn at(&self, pos: usize) -> &str;
}

impl StringPlus for str {
    fn substring(&self, start: usize, length: usize) -> &str {
        &self[start..(start + length)]
    }

    fn at(&self, pos: usize) -> &str {
        &self[pos..pos + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::StringPlus;

    #[test]
    fn substring() {
        assert_eq!("Bangladesh".to_string().substring(0, 6), "Bangla");
        assert_eq!("Bangladesh".substring(3, 4), "glad");
    }

    #[test]
    fn at() {
        assert_eq!("Hello".at(1), "e");
        assert_eq!("Object".to_string().at(3), "e");
    }
}
