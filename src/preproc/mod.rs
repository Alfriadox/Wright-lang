// preprocessing function to remove comments from code and number the lines.
pub fn preproc(input_string: String) -> Vec<NumberedString> {
    let mut return_vec: Vec<NumberedString> = vec![NumberedString::new_empty(1)];
    // no two will ever be true simultaneously.
    let mut in_quotes = false;
    let mut in_multiline_comment = false;
    let mut in_single_line_comment = false;
    let mut last_char: char = ' ';
    let mut line_number: u64 = 2;
    // commented out because using bytes as boundaries won't always work for utf8 strings
    //let input_vec = input_string.into_bytes();
    for character in input_string.as_str().chars() {
        //let character = byte as char;
        let last_index = return_vec.len()-1;
        if in_quotes {
            if character == '"' {
                in_quotes = false;
            }
            return_vec[last_index].line.push(character);
        } else if in_multiline_comment {
            if character == '/' && last_char == '*' {
                in_multiline_comment = false;
                return_vec[last_index].line.push(' ');
                // push a space to prevent fools from putting comments in the middle of their variable names
            }
        } else {
            // not in quote or multi line comment.
            if in_single_line_comment && character == '\n' {
                in_single_line_comment = false;
            }
            if !in_single_line_comment {
                if character == '"' {
                    in_quotes = true;
                    return_vec[last_index].line.push(character);
                } else if character == '/' && last_char == '/' {
                    in_single_line_comment = true;
                    return_vec[last_index].line.pop(); // remove previous '/'
                } else if character == '*' && last_char == '/' {
                    in_multiline_comment = true;
                    return_vec[last_index].line.pop(); // remove previous '/'
                } else {
                    return_vec[last_index].line.push(character);;
                }
            }
        }
        last_char = character;
        if character == '\n' {
            return_vec.push(NumberedString::new_empty(line_number));
            line_number += 1;
        }
    }
    return return_vec;
}

#[derive(Debug, Clone)]
pub struct NumberedString {
    pub line_number: u64,
    pub line: String,
}

impl NumberedString {
    pub fn new(line_content: String, line_num: u64) -> NumberedString {
        NumberedString {line: line_content, line_number: line_num}
    }
    pub fn new_empty(line_num: u64) -> NumberedString {
        NumberedString::new("".to_string(), line_num)
    }
}