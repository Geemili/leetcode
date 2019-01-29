// Estimated time: 30min
// Actual time: 23min

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        let mut max_val = None;
        for k in 0..a.len() as i32 {
            let res = rotate_function(&a, k);
            max_val = Some(max_val.unwrap_or(res).max(res));
        }
        max_val.unwrap_or(0)
    }
}

fn rotate_function(a: &[i32], k: i32) -> i32 {
    let mut out = 0;
    for i in 0..a.len() as i32 {
        let idx = mod_sub(i, k, a.len() as i32) as usize;
        out += i * a[idx];
    }
    out
}

fn mod_sub(a: i32, b: i32, top: i32) -> i32 {
    match a - b {
        x if x < 0    =>{ top + x }
        x if x >= top =>{ x - top }
        x => x
    }
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mod_sub_1() {
        assert_eq!(3, mod_sub(0, 1, 4));
        assert_eq!(2, mod_sub(0, 2, 4));
        assert_eq!(1, mod_sub(0, 3, 4));
        assert_eq!(0, mod_sub(0, 0, 4));
    }

    #[test]
    fn rotate_func_1() {
        assert_eq!(25, rotate_function(&[4, 3, 2, 6][..], 0));
    }

    #[test]
    fn rotate_func_2() {
        assert_eq!(16, rotate_function(&[4, 3, 2, 6][..], 1));
    }

    #[test]
    fn example_1() {
        assert_eq!(26, Solution::max_rotate_function(vec![4, 3, 2, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(-2147483648, Solution::max_rotate_function(vec![-2147483648,-2147483648]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::max_rotate_function(vec![1000000007]));
    }

    // Definitions
    pub struct Solution;
}
