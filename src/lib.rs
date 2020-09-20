/// Some generic functions
///
fn list_to_string(list: &Vec<String>) -> String {
    return list.join(", ")
}

#[cfg(test)]
mod tests {
    use super::list_to_string;

    #[test]
    fn test_list_to_string() {
        let test_args = vec!["-w".to_string(), "60".to_string(), "arg".to_string(), "whoa".to_string()];

        assert_eq!(list_to_string(&test_args), "-w, 60, arg, whoa")
    }
}
