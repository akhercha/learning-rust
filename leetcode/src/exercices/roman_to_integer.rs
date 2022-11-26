use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Solution {}

lazy_static! {
    static ref ASSO: HashMap<&'static str, &'static int> = {
        let mut map = HashMap::new();
        map.insert("I", 1);
        map.insert("V", 5);
        map.insert("X", 10);
        map.insert("L", 50);
        map.insert("C", 100);
        map.insert("D", 500);
        map.insert("M", 1000);
        map
    };
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.chars().collect();
        let nb = 0;
        
        for c in &s {
            nb += ASSO.get(c);
        }
        nb
    }
}