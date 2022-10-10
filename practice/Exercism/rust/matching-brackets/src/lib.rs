use std::collections::HashMap;

pub fn brackets_are_balanced_hashmap(string: &str) -> bool {
    let map = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
    let mut brakets_stack: Vec<char> = vec![];

    for bracket in string.chars() {
        if map.contains_key(&bracket) {
            brakets_stack.push(bracket);
            continue;
        }
        if map.values().any(|v| v == &bracket) {
            let match_braket = brakets_stack.pop();
            match match_braket {
                Some(left_bracket) => {
                    let right_braket = map.get(&left_bracket);
                    if right_braket == Some(&bracket) {
                        continue;
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }

    brakets_stack.is_empty()
}

pub fn brackets_are_balanced_vec(string: &str) -> bool {
    let mut bracket_stack: Vec<char> = vec![];

    for ch in string.chars() {
        match ch {
            '[' => bracket_stack.push(']'),
            '{' => bracket_stack.push('}'),
            '(' => bracket_stack.push(')'),
            ']' | '}' | ')' => {
                if bracket_stack.pop() != Some(ch) {
                    return false;
                }
            }

            _ => continue,
        }
    }

    bracket_stack.is_empty()
}

pub fn brackets_are_balanced(string: &str) -> bool {
    brackets_are_balanced_vec(string)
}
