use crate::constants::STONE_MULTIPLIER;
use std::collections::HashMap;

pub fn count_stones(stones: Vec<i128>, num_iterations: usize) -> usize {
    let mut stones_map = construct_hashmap(stones);

    for _ in 0..num_iterations {
        stones_map = iterate_hashmap(&stones_map);
    }

    stones_map.values().sum()
}

fn construct_hashmap(stones: Vec<i128>) -> HashMap<i128, usize> {
    let mut result: HashMap<i128, usize> = HashMap::new();

    for stone in stones {
        *result.entry(stone).or_insert(0) += 1;
    }

    result
}

fn iterate_hashmap(stones: &HashMap<i128, usize>) -> HashMap<i128, usize> {
    let mut next: HashMap<i128, usize> = HashMap::new();

    for (num, total) in stones {
        if *num == 0 {
            *next.entry(1).or_insert(0) += total;
            continue;
        }

        let digit_count = num.abs().to_string().len();

        if digit_count % 2 == 0 {
            let (left, right) = split_num(*num);
            *next.entry(left).or_insert(0) += total;
            *next.entry(right).or_insert(0) += total;
            continue;
        }

        *next.entry(num * STONE_MULTIPLIER).or_insert(0) += total;
    }

    next
}

// Handle one "blink" or iteration for the stones
fn iterate(stones: Vec<i128>) -> Vec<i128> {
    let mut result: Vec<i128> = Vec::new();

    for stone in stones {
        if stone == 0 {
            result.push(1);
            continue;
        }
        let digit_count = stone.abs().to_string().len();
        if digit_count % 2 == 0 {
            let (left, right) = split_num(stone);
            result.push(left);
            result.push(right);
            continue;
        }

        result.push(stone * STONE_MULTIPLIER);
    }

    result
}

// Splits a number into its left and right digits
// e.g. 99 -> (9, 9)
fn split_num(num: i128) -> (i128, i128) {
    let digit_count = num.abs().to_string().len();
    let mut left = 0;
    let mut right = 0;

    let mut num = num;
    let mut power = 0;
    let base_10: i128 = 10;

    for _ in 0..digit_count / 2 {
        right += (num % 10) * (base_10.pow(power));
        power += 1;
        num /= 10;
    }

    power = 0;

    for _ in 0..digit_count / 2 {
        left += (num % 10) * (base_10.pow(power));
        power += 1;
        num /= 10;
    }

    (left, right)
}

#[cfg(test)]
mod tests {

    use crate::constants::NUM_ITERATIONS;

    use super::*;

    #[test]
    fn test_split_num_simple() {
        assert_eq!(split_num(99), (9, 9))
    }

    #[test]
    fn test_split_num_leading_zeros() {
        assert_eq!(split_num(100000), (100, 0))
    }

    #[test]
    fn test_iterate() {
        let mut stones = vec![125, 17];
        let iterations = vec![
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        ];
        for i in 0..iterations.len() {
            stones = iterate(stones);
            assert_eq!(stones, iterations[i]);
        }
    }

    #[test]
    fn test_count_stones() {
        let stones = vec![125, 17];
        assert_eq!(count_stones(stones, NUM_ITERATIONS), 55312);
    }
}
