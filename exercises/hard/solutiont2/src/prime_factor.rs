// AI

/// 一个简单的线性同余随机数生成器（LCG）
fn simple_rand(state: &mut u128, lower: u128, upper: u128) -> u128 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    lower + (*state % (upper - lower))
}

/// Miller–Rabin 素性检测
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    // 表示 n-1 为 d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let bases: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    'witness: for &a in &bases {
        if a % n == 0 {
            continue;
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..(s - 1) {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// 模乘，防止溢出
fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut a = a % m;
    let mut b = b;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

/// 模幂
fn mod_exp(mut base: u128, mut exp: u128, m: u128) -> u128 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

/// 辗转相除法求最大公约数
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// 用试除法剔除较小因子，这里预处理所有小于 limit 的因子
fn trial_division(mut n: u128, limit: u128, factors: &mut Vec<u128>) -> u128 {
    // 先处理 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    // 然后试除奇数
    let mut i = 3;
    while i <= limit && i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    n
}

/// Pollard Rho 算法
fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    let mut rng_state = 88172645463325252u128;
    let mut x = simple_rand(&mut rng_state, 2, n);
    let mut y = x;
    let c = simple_rand(&mut rng_state, 1, n);
    let mut d = 1;
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(if x > y { x - y } else { y - x }, n);
        if d == n {
            return pollard_rho(n);
        }
    }
    d
}

/// 采用迭代方式分解 n 的因子，并预处理小因子
fn factorize_iter(n: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    // 先试除一部分小因子，预处理掉所有小于 10^6 的因子
    let n = trial_division(n, 1_000_000, &mut factors);
    let mut stack = vec![n];
    while let Some(x) = stack.pop() {
        if x == 1 {
            continue;
        }
        if is_prime(x) {
            factors.push(x);
        } else {
            let factor = pollard_rho(x);
            stack.push(factor);
            stack.push(x / factor);
        }
    }
    factors
}

/// 返回正整数 n 的最大素数因子
pub fn find_max_prime_factor(n: u128) -> u128 {
    let factors = factorize_iter(n);
    *factors.iter().max().unwrap()
}
