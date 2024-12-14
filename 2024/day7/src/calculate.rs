use std::cmp::max;

pub fn calibrate(total: i64, curr_total: i64, i: usize, nums: &Vec<i64>) -> i64 {

    if i == nums.len() && curr_total == total {
        return curr_total;
    }

    if i == nums.len() {
        return 0;
    }
    
    let add_result = calibrate(total, curr_total + nums[i], i + 1, nums);
    let mul_result = calibrate(total, curr_total * nums[i], i + 1, nums);

    max(add_result, mul_result)
}


pub fn calibrate_v2(total: i64, curr_total: i64, i: usize, nums: &Vec<i64>) -> i64 {

    if i == nums.len() && curr_total == total {
        return curr_total;
    }

    if i == nums.len() {
        return 0;
    }
    
    let add_result = calibrate_v2(total, curr_total + nums[i], i + 1, nums);
    let mul_result = calibrate_v2(total, curr_total * nums[i], i + 1, nums);

    let concatenated = concatenate_numbers(curr_total, nums[i]);
    let concat_result = calibrate_v2(total, concatenated, i + 1, nums);

    max(max(add_result, mul_result), concat_result)
}


fn concatenate_numbers(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}
