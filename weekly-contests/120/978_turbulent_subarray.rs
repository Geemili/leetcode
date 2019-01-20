impl Solution {
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        if a.len() <= 1 {
            return a.len() as i32;
        }
        let mut last = P::Same;
        let mut max_count = 0;
        let mut cur_count = 0;
        let p_array = to_p_array(&a);
        for p in p_array {
            match (last, p) {
                (P::Less, P::More) => cur_count += 1,
                (P::More, P::Less) => cur_count += 1,
                _ => {
                    max_count = max_count.max(cur_count);
                    cur_count = 0;
                }
            }
            last = p;
        }
        max_count.max(cur_count) + 2
    }
}

#[derive(Copy, Clone, Debug)]
enum P {
    Less,
    Same,
    More,
}

fn to_p_array(a: &[i32]) -> Vec<P> {
    let mut k = 0;
    let mut p_array = Vec::new();
    while k < a.len()-1 {
        if a[k] > a[k+1] {
            p_array.push(P::Less);
        } else if a[k] < a[k+1] {
            p_array.push(P::More);
        } else {
            p_array.push(P::Same);
        }
        k += 1;
    }
    p_array
}


#[cfg(test)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    fn check(input: Vec<i32>, e: i32) {
        let output = Solution::max_turbulence_size(input);
        assert_eq!(output, e);
    }

    #[test]
    fn example_1() {
        let result = vec![9,4,2,10,7,8,8,1,9];
        let expected = 5;
        check(result, expected);
    }

    #[test]
    fn example_2() {
        let result = vec![4,8,12,16];
        let expected = 2;
        check(result, expected);
    }

    #[test]
    fn example_3() {
        let result = vec![100];
        let expected = 1;
        check(result, expected);
    }

    #[test]
    fn example_4() {
        let result = vec![0,8,45,88,48,68,28,55,17,24];
        let expected = 8;
        check(result, expected);
    }
}
