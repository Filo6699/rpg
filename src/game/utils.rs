pub fn calculate_xp_filled(xp: u64, needed_xp: u64, precision: u32) -> (String, String) {
    let percentage: u64 = xp * <u32 as Into<u64>>::into(precision) / needed_xp;
    let mut filled = String::from("");
    let mut empty = String::from("");
    for i in 0..precision {
        if percentage > i.into() {
            filled.push('_');
        } else {
            empty.push('_');
        }
    }
    (filled, empty)
}
