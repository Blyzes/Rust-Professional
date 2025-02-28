pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut prob = 1.0;
    for i in 0..n {
        prob *= (365 - i) as f64 / 365.0;
    }
    1.0 - prob
}
