mod two_sum;

use std::collections::{HashMap};
use std::option::Option::Some;

fn main() {
    let data = vec![1,5,8,9];
    let target : i32 = 8;
    let result = two_sum(data, target);
    result.get(0);
    result.get(1);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());

    for i in 0..nums.len() {
        if let Some(k) = map.get(&(target - nums[i])) {
            if *k != i {
                return vec![*k as i32, i as i32];
            }
        }
        map.insert(nums[i], i);
    }
    panic!("not found sume data")
}
