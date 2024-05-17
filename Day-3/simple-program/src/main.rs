fn main() {
    let sentence: String = String::from("my name is sulav");

    let first_word = get_first_word(sentence);
    println!("The first word is {first_word}")
}

fn get_first_word(sentence: String) -> String {
    let mut result = String::new();

    for char in sentence.chars() {
        result.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return result;
}
