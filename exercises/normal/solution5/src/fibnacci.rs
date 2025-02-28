pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut sum = 0;
    let mut a = 0; // f(0)
    let mut b = 1; // f(1)
    while a < threshold {
        if (a & 1) != 0 {
            sum += a;
        }
        
        let temp = a + b;
        a = b;
        b = temp;
    }
    sum
}

// fn fibnacci(n: u32) -> u32 {
//     if n == 0 {
//         0
//     } else if n == 1 {
//         1
//     } else {
//         fibnacci(n - 1) + fibnacci(n - 2)
//     }
// }
