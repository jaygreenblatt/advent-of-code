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