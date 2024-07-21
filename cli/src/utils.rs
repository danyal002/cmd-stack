pub fn truncate_string(s: &str, width: usize) -> String {
    if width < 3 {
        "...".to_string()
    } else if s.chars().count() > width {
        format!("{}...", &s.chars().take(width - 3).collect::<String>())
    } else {
        s.to_string()
    }
}
