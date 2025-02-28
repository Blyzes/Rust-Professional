use std::collections::HashMap;

/*
⾃⼰设计实现⼀个统计不重复元素个数的算法，输⼊为逗号分隔的字符串
不重复指的是重复的只算一次...
*/

pub fn new_count_distinct(input_str: &str) -> usize {
    let data: Vec<_> = input_str.split(',').collect();
    println!("{data:?}");
    let mut map = HashMap::new();
    for i in data {
        map.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    map.values().count()
}
