#[cfg(test)]
mod tests {
    use kalibak::utils::*;
    #[test]
    fn test_list_to_string() {
        let test_args = vec![
            "-w".to_string(),
            "60".to_string(),
            "arg".to_string(),
            "whoa".to_string(),
        ];

        assert_eq!(list_to_string(&test_args), "-w, 60, arg, whoa")
    }

    #[test]
    fn test_articles() {
        let words = "The Avengers: Endgame".to_string();

        assert_eq!(remove_articles(&words), "avengers endgame")
    }
}
