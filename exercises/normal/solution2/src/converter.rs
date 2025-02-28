// 实现⼀个算法，⽀持任意2-16进制数之间转换，如：2进制转7进制，7进制转16进制，15进制转8进制等。

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let mut ans = String::new();
    let (n, b) = num_str.split_at(num_str.find('(').unwrap());
    let num: Vec<_> = n.chars().rev().collect();
    let base = b[1..b.len() - 1].parse::<u32>().unwrap();
    let symbol = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut num_base10 = 0;

    // to base10
    for i in 0..num.len() {
        num_base10 +=
            symbol.iter().position(|&x| x.to_ascii_lowercase() == num[i]).unwrap() as u32 * (base.pow(i as u32));
    }

    if num_base10 == 0 {
        return '0'.to_string();
    }

    // to to_base
    while num_base10 > 0 {
        let temp = num_base10 % to_base;
        ans.push(symbol[temp as usize]);
        num_base10 /= to_base;
    }
    ans.chars().rev().collect()
}
