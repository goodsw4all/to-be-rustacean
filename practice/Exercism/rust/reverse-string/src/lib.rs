pub fn reverse(input: &str) -> String {
    // let mut reveresed_string = String::from("");
    // for ch in input.chars().rev() {

    //     reveresed_string.push(ch);
    // }
    // reveresed_string

    let temp = input.chars().rev().collect::<String>();
    temp
}
