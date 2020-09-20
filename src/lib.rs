/// Some generic functions
///
fn list_to_string(list: Vec<&str>) -> String {
    return list.join(", ")
}

#[cfg(test)]
mod tests {
    use super::list_to_string;

    #[test]
    fn test_list_to_string() {
        let test_args = vec!["-w", "60", "arg", "whoa"];

        assert_eq!(list_to_string(test_args), "-w, 60, arg, whoa")
    }
}
