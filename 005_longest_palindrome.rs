impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 { return String::new(); }
        let chars: Vec<_> = s.chars().collect();
        let mut longest_palindrome = String::new();
        longest_palindrome.push(chars[0]);
        for left in 0..chars.len() {
            let mut right = chars.len()-1;
            'INNER: while right >= (left+longest_palindrome.len()) {
                let text = &chars[left..right+1];
                if chars[left] == chars[right] && is_palindrome(text) {
                    longest_palindrome.clear();
                    for c in text {
                        longest_palindrome.push(*c);
                    }
                    break 'INNER;
                }
                right -= 1;
            }
        }
        longest_palindrome
    }
}

fn is_palindrome(text: &[char]) -> bool {
    if text.len() == 0 { return true; }
    let mut left = 0;
    let mut right = text.len()-1;
    while left < right {
        if text[left] != text[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
