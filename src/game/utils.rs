pub fn calculate_xp_filled(xp: u64, needed_xp: u64) -> (String, String) {
    let percentage = xp * 10 / needed_xp;
    let mut filled = String::from("");
    let mut empty = String::from("");
    for i in 0..10 {
        if percentage > i {
            filled.push('#');
        } else {
            empty.push('.');
        }
    }
    (filled, empty)
}
