pub fn raindrops(n: u32) -> String {
    let res = String::new();
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => res + "Pling" + "Plang" + "Plong",
        (0, 0, _) => res + "Pling" + "Plang",
        (0, _, 0) => res + "Pling" + "Plong",
        (_, 0, 0) => res + "Plang" + "Plong",
        (0, _, _) => res + "Pling",
        (_, 0, _) => res + "Plang",
        (_, _, 0) => res + "Plong",
        (_, _, _) => n.to_string(),
    }
}
