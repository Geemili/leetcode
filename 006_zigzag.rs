impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let col_diag_width = 2*num_rows as usize - 2;
        let num_col_diag_sections = s.len() / col_diag_width + 1;
        let width = col_diag_width * num_col_diag_sections;
        let mut out: Vec<Option<char>> = vec![None; num_rows as usize * width];

        let mut x = 0;
        let mut y = 0;
        let mut going_up = false;
        for c in chars.iter() {
            out[to_index(width, x, y)] = Some(*c);
            if going_up && y == 0 {
                going_up = false;
            } else if !going_up && y == num_rows as usize - 1 {
                going_up = true;
            }
            if going_up {
                x += 1;
                y -= 1;
            } else {
                y += 1;
            }
        }
        let mut sout = String::new();
        for c in out.iter().filter_map(|co| *co) {
            sout.push(c);
        }
        sout
    }
}

fn to_index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}


#[cfg(test)]
struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    fn check(s: &str, num_rows: i32, expected: &str) {
        let result = Solution::convert(s.to_string(), num_rows);
        assert_eq!(result, expected);
    }

    #[test]
    fn paypalishiring_1rows() {
        check("PAYPALISHIRING", 1, "PAYPALISHIRING");
    }

    #[test]
    fn paypalishiring_2rows() {
        check("PAYPALISHIRING", 2, "PYAIHRNAPLSIIG");
    }

    #[test]
    fn paypalishiring_3rows() {
        check("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn paypalishiring_4rows() {
        check("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    }
}
