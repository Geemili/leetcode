impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::new();
        for num in a {
            output.push(num * num);
        }
        output.sort();
        output
    }
}

#[cfg(test)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    fn check(input: Vec<i32>, e: Vec<i32>) {
        let output = Solution::sorted_squares(input);
        assert_eq!(output, e);
    }

    #[test]
    fn example_1() {
        let result = vec![-4,-1,0,3,10];
        let expected = vec![0,1,9,16,100];
        check(result, expected);
    }

    #[test]
    fn example_2() {
        let result = vec![-7,-3,2,3,11];
        let expected = vec![4,9,9,49,121];
        check(result, expected);
    }
}
