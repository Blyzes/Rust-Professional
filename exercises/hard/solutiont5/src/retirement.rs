#![allow(unused)]
/*
从2025年1月1日起，男职工和原法定退休年龄为五十五周岁的女职工，法定退休年龄每四个月延迟一个月，分别逐步延迟至六十三周岁和五十八周岁；
原法定退休年龄为五十周岁的女职工，法定退休年龄每二个月延迟一个月，逐步延迟至五十五周岁。
男职工原法定退休年龄为六十周岁
不满四个月/两个月的向上取整
*/

use std::collections::HashMap;

const TYPES: [(&str, [u32; 4]); 3] = [
    ("男职工", [1965, 60, 4, 36]),
    ("原法定退休年龄55周岁女职工", [1970, 55, 4, 36]),
    ("原法定退休年龄50周岁女职工", [1975, 50, 2, 60]),
];

fn calculate(
    year: u32,
    month: u32,
    start_year: u32,
    old_retire_age: u32,
    ratio: u32,
    max_months: u32,
) -> String {
    if year < start_year {
        return format!(
            "{}-{:02},{},0",
            year + old_retire_age,
            month,
            old_retire_age
        );
    }

    let months = (year - start_year) * 12 + month - 1; // 距离原退休年龄，计算延迟的月数
    // let delay_months = months.div_ceil(ratio).min(max_months); // 最多延迟的月数, div_ceil 结果为0时少加1
    let delay_months = (months / ratio + 1).min(max_months);
    let total_months = old_retire_age * 12 + delay_months; // 从出生到退休的总月数
    println!("months {months} delay_months {delay_months} total_months {total_months}");

    // let retire_year = total_months / 12 + start_year;
    let retire_year = (total_months + month - 1) / 12 + year;
    let retire_month = (months + total_months) % 12 + 1;

    return format!(
        "{:04}-{:02},{},{}",
        retire_year,
        retire_month,
        if total_months % 12 == 0 {
            (total_months / 12).to_string()
        } else {
            format!("{:.2}", total_months as f64 / 12.0)
        },
        delay_months
    );
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let time: Vec<_> = time.split('-').map(|x| x.parse::<u32>().unwrap()).collect();
    let year = time[0];
    let month = time[1];

    let binding = HashMap::from(TYPES);
    let info = binding[tp];
    let [start_year, old_retire_age, ratio, max_months] = info;

    calculate(year, month, start_year, old_retire_age, ratio, max_months)
}
