#![allow(unused)]
use serde_json::{Map, Value};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::BufReader,
};

type citymap = BTreeMap<String, BTreeMap<String, Vec<String>>>;

pub fn count_provinces() -> String {
    let file = File::open("exercises/hard/solutiont3/district.json").unwrap();
    let reader = BufReader::new(file);
    let datas: citymap = serde_json::from_reader(reader).unwrap();
    let mut res: Vec<usize> = vec![];

    for (i, data) in datas {
        let mut citys: Vec<HashSet<String>> = Vec::new();

        for (city, related) in data {
            let mut group = HashSet::new();
            group.insert(city);
            group.extend(related.into_iter());

            let mut merged_group = group.clone();
            let mut remaining_group:Vec<HashSet<String>> = Vec::new();

            for existing in citys.into_iter() {
                if !existing.is_disjoint(&group) {
                    merged_group = merged_group.union(&existing).cloned().collect();
                    // citys.
                } else {
                    remaining_group.push(existing);
                }
            }
            remaining_group.push(merged_group);
            citys = remaining_group;
            println!("{citys:?}");
        }
        res.push(citys.len());

    }
    println!("{res:?}");
    res.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")

    // "0".to_string()
}
