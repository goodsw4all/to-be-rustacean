pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            list.windows(2)
                .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
                .collect::<String>()
                + &format!("And all for the want of a {}.", list[0])
        }
    }
}
