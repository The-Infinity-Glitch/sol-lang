//enum Token {
//    SEMICOLON
//}
aa
pub fn lexer_source(source: &String) {
    for a in source_to_array(&source) {
        match &a.to_string().as_str() {
            &"import" => println!("Import Token"),
            _ => println!("Nothing")
        }
    }
}

fn source_to_array(source: &String) -> Vec<String> {
    // All the symbols and operators used for mark the division of the words
    let special_symbols = vec!["(", ")", "{", "}", ";", ",", ".", r#"""#];
    let operators = vec!["=", "+", "-", "*", "/", "<", ">"];
    
    let mut current_text = String::new(); // current_text: retains the current word formed by the c in the for loop below.
    let mut result_array = vec![];   // result_array: retains the result of the function, the for loop below insert current_text here before current_text clear.

    // The c variable of the loop retains the current char of the source, if it's a special symbol or operator, it's inserted in result_array, else, it is inserted in current_word.
    for c in source.chars() {
        if c == ' ' || c == '\n' || special_symbols.contains(&c.to_string().as_str()) || operators.contains(&c.to_string().as_str()) {
            // If current_text is diferent of nothing, so insert it in result_array.
            if current_text != ""{
                result_array.insert(result_array.len(), current_text.clone());
            }
            
            // If the current char is a \n, ignore it, else insert it in current_array
            if c != '\n' {
                result_array.insert(result_array.len(), c.to_string());
            }
            
            // Clear current_text
            current_text.clear()
        } else {
            // Insert the current char in current_text
            current_text.push(c);
        }
    }
    
    return result_array;
}