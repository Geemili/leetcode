// Estimate: 20mins
// Actual: 50mins
//
// Wrangling linked lists in rust is hard, even if merging two lists isn't that complex.

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut new_list, mut cur_l1, mut cur_l2) = match Self::comp_nodes(l1, l2) {
            None => return None,
            Some((first, other)) => (Box::new(ListNode::new(first.val)), first.next, other),
        };
        {
            let mut current_elem = &mut new_list;
            loop {
                let res = match Self::comp_nodes(cur_l1, cur_l2) {
                    None => break,
                    Some((cur, other)) => {
                        let cur_val_clone = Some(Box::new(ListNode::new(cur.val)));
                        let current = current_elem;
                        current.next = cur_val_clone;
                        current_elem = current.next.as_mut().unwrap();
                        (cur.next, other)
                    }
                };
                cur_l1 = res.0;
                cur_l2 = res.1;
            }
        }
        Some(new_list)
    }

    fn comp_nodes(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<(Box<ListNode>, Option<Box<ListNode>>)> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                if l2.val < l1.val {
                    Some((l2, Some(l1)))
                } else {
                    Some((l1, Some(l2)))
                }
            }
            (None, Some(l2)) => Some((l2, None)),
            (Some(l1), None) => Some((l1, None)),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let a = slice_to_list(&[1, 2, 4]);
        let b = slice_to_list(&[1, 3, 4]);
        let expected = slice_to_list(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(a, b), expected);
    }

    #[test]
    fn no_input_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn first_list_only() {
        let a = slice_to_list(&[1, 2, 4]);
        let expected = slice_to_list(&[1, 2, 4]);
        assert_eq!(Solution::merge_two_lists(a, None), expected);
    }

    #[test]
    fn second_list_only() {
        let a = slice_to_list(&[1, 2, 4]);
        let expected = slice_to_list(&[1, 2, 4]);
        assert_eq!(Solution::merge_two_lists(None, a), expected);
    }

    #[test]
    fn different_size_lists() {
        let a = slice_to_list(&[1]);
        let b = slice_to_list(&[1, 3, 4]);
        let expected = slice_to_list(&[1, 1, 3, 4]);
        assert_eq!(Solution::merge_two_lists(a, b), expected);

        let a = slice_to_list(&[1]);
        let b = slice_to_list(&[1, 3, 4]);
        let expected = slice_to_list(&[1, 1, 3, 4]);
        assert_eq!(Solution::merge_two_lists(b, a), expected);
    }

    // Definitions
    pub struct Solution;

    fn slice_to_list(slice: &[i32]) -> Option<Box<ListNode>> {
        if slice.len() == 0 {
            return None;
        }
        let mut first = Box::new(ListNode::new(slice[0]));
        {
            let mut cur = &mut first;
            for v in slice.iter().skip(1) {
                let next = Some(Box::new(ListNode::new(*v)));
                let prev = cur;
                prev.next = next;
                cur = prev.next.as_mut().unwrap();
            }
        }
        Some(first)
    }

    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
}
