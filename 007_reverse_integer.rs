// Estimated completion time: 15mins
// Actual time: 15 mins

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let string = format!("{}", x.abs());
        let mut out = String::new();
        out.extend(string.chars().rev());
        out.parse::<i32>().unwrap_or(0) * sign
    }
}


#[cfg(test)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

}
