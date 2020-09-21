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

    let mut word_list = Vec::<String>::new();

    'outer: for word in text.to_lowercase().split_whitespace() {
        for article in &articles {
            if word == article {
                continue 'outer;
            }
        }
        word_list.push(word.to_string());
    }
    let mut new_string = word_list.join(" ");

    // Now remove some other special characters
    new_string = new_string.replace(":", "");
    new_string = new_string.replace(",", "");
    new_string = new_string.replace("-", " ");

    return new_string;
}
