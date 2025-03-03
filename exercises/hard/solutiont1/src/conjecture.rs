pub fn goldbach_conjecture() -> String {
    let mut primes = vec![];
    const N:usize = 100000;
    let mut is_prime = [false;N];
    let mut ans = vec![];

    // prime
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..N {
        is_prime[i] = true
    }
    for i in 2..N.isqrt()+1 {
        if is_prime[i] {
            for j in (i*i..N).step_by(i){
                is_prime[j] = false
            }
        }
    }

    for i in 2..N {
        if is_prime[i] {
            primes.push(i);
        }
    }

    let mut holds = [false;N];
    holds[0] = true;
    holds[1] = true;
    for prime in primes {
        holds[prime] = true;
        for i in 1..((N-prime)/2).isqrt() + 1 {
            let num = prime + 2 * i.pow(2);
            if num < N {
                holds[num] = true;
            }
        }
    }

    for (num, hold) in holds.iter().enumerate() {
        if !*hold && num % 2 == 1 {
            ans.push(num.to_string());
        }
    }

    // println!("{ans:?}");
    ans[..2].join(",")
}
