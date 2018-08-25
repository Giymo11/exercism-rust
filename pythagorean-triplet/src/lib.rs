pub fn find() -> Option<u32> {

    // a + b + c = 1000
    // a^2 + b^2 = c^2

    for a in 1u32..=1000 {
        // because if a + b is already 1000 its kinda boring
        for b in 1u32..(1000 - a) {
            // if a and b are fixed, c has to add up to 1000
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c);
            }
        }
    }
    return None;
}
