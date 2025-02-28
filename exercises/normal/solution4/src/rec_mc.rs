pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut money = amount;
    let mut ans = 0;
    let mut moneys = [1, 2, 5, 10, 20, 30, 50, 100];
    moneys.reverse();
    for i in moneys {
        if money >= i {
            let temp = money / i;
            ans += temp;
            money %= i;
        }
    }
    ans
}
