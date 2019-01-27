// Estimated time: 15 mins
// Actual time: 73 mins

impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut out = Vec::new();
        let mut state = a > b;
        while a > 0 && b > 0 {
            if state {
                out.push('a');
                a -= 1;
            } else {
                out.push('b');
                b -= 1;
            }
            state = !state;
        }
        let mut pos: usize = 0;
        while a > 0 {
            if out.get(pos) == Some(&'b'){
                out.insert(pos, 'a');
                a -= 1;
                pos += 1;
            }
            if pos == out.len() {
                out.insert(pos, 'a');
                a -= 1;
            }
            pos += 1;
        }
        while b > 0 {
            if out.get(pos) == Some(&'a') {
                out.insert(pos, 'b');
                b -= 1;
                pos += 1;
            }
            if pos == out.len() {
                out.insert(pos, 'b');
                b -= 1;
            }
            pos += 1;
        }
        let mut o = String::new();
        o.extend(out);
        o
    }
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    fn no_aaa_or_bbb(text: &str) -> bool {
        text.find("aaa").is_none() && text.find("bbb").is_none()
    }

    fn correct_num_letters(text: &str, a: i32, b: i32) -> bool {
        let mut num_a = 0;
        let mut num_b = 0;
        for c in text.chars() {
            match c {
                'a' => num_a += 1,
                'b' => num_b += 1,
                _ => panic!("Invalid character in text!"),
            }
        }
        num_a == a && num_b == b
    }

    fn check(a: i32, b: i32) {
        let text = Solution::str_without3a3b(a, b);
        dbg!((&text, a, b));
        assert!(no_aaa_or_bbb(&text));
        assert!(correct_num_letters(&text, a, b));
    }

    #[test]
    fn example_1() {
        check(1, 2);
    }

    #[test]
    fn example_2() {
        check(4, 1);
    }

    #[test]
    fn example_3() {
        check(5, 2);
    }

    #[test]
    fn example_4() {
        check(1, 1);
    }

    #[test]
    fn example_5() {
        check(4, 4);
    }

    #[test]
    fn example_6() {
        check(1, 4);
    }

    // Definitions
    pub struct Solution;
}
