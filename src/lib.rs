/// Some generic functions
///
fn list_to_string(list: Vec<&str>) -> String {
    let joined = list.join(", ");
    return joined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_strings() {
        let test_args = vec!["-w", "60", "arg", "whoa"];

        assert_eq!(list_to_string(test_args), "-w, 60, arg, whoa")
    }
}
