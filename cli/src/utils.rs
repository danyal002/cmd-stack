pub fn truncate_string(s: &str, width: usize) -> String {
    
    if s.chars().count() > width {
        if width < 3 {
            "...".to_string()
        } else {
            format!("{}...", &s.chars().take(width - 3).collect::<String>())
        }
    } else {
        s.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_truncate() {
        assert_eq!("...", truncate_string("abc", 2));
        assert_eq!("...", truncate_string("abcd", 3));
        assert_eq!("a...", truncate_string("abcde", 4))
    }

    #[test]
    fn fits() {
        assert_eq!("ab", truncate_string("ab", 2));
        assert_eq!("ab", truncate_string("ab", 3));
        assert_eq!("abc", truncate_string("abc", 3));
        assert_eq!("abcd", truncate_string("abcd", 7));
    }
}
