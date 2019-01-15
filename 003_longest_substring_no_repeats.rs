use std::collections::BTreeSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut longest_substring_len = 0i32;
        let (mut start, mut end) = (0, 0);
        let mut chars_used = BTreeSet::new();
        loop {
            if end >= s.len() {
                break;
            }

            // If the next char is not a repeat
            if !chars_used.contains(&chars[end]) {
                // Increase the substring length, and add the char to the list of used chars
                chars_used.insert(chars[end]);
                end += 1;
            } else if start < end {
                // Otherwise, remove the char at start of the list
                chars_used.remove(&chars[start]);
                start += 1;
            } else {
                unreachable!();
            }

            // Check if the substring is the longest we've seen
            let len = (end - start) as i32;
            longest_substring_len = longest_substring_len.max(len);
        }
        longest_substring_len
    }
}
