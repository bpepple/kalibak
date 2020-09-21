/// Some generic functions
///
pub fn list_to_string(list: &Vec<String>) -> String {
    return list.join(", ");
}

pub fn remove_articles(text: &String) -> String {
    // Articles to remove
    let articles = vec![
        "and".to_string(),
        "a".to_string(),
        "&".to_string(),
        "issue".to_string(),
        "the".to_string(),
    ];

    let lowered_text = text.to_lowercase();
    let mut new_string = String::new();

    'outer: for word in lowered_text.split_whitespace() {
        for article in &articles {
            if word == article {
                continue 'outer;
            }
        }
        new_string.push_str(&word);
        new_string.push_str(" ");
    }
    // Remove space at end of string
    let len = new_string.len();
    new_string.truncate(len - 1);

    // Now remove some other special characters
    new_string = new_string.replace(":", "");
    new_string = new_string.replace(",", "");
    new_string = new_string.replace("-", " ");

    return new_string;
}
