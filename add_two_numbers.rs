use std::borrow::BorrowMut;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if !l1.is_some() && !l2.is_some() {
            return None;
        }
        let (mut new_list, mut cur_l1, mut cur_l2, mut carry) = match add_digits(l1, l2, 0) {
            (None, _, _, _) => return None,
            (Some(first), l1_next, l2_next, carry) => (first, l1_next, l2_next, carry),
        };
        let mut current_digit = &mut new_list;
        loop {
            let res = match add_digits(cur_l1, cur_l2, carry) {
                (None, _, _, _) => return Some(new_list),
                (Some(digit), l1_next, l2_next, carry) => {
                    current_digit.next = Some(digit);
                    (current_digit.next.as_mut().unwrap(), l1_next, l2_next, carry)
                }
            };
            current_digit = res.0;
            cur_l1 = res.1;
            cur_l2 = res.2;
            carry = res.3;
        }
    }

}

// returns (sum, l1.next, l2.next, carry)
fn add_digits(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>, Option<Box<ListNode>>, i32) {
    let (sum, l1_next, l2_next) = match (l1, l2) {
        (None, None) if carry > 0 => return (Some(Box::new(ListNode::new(carry))), None, None, 0),
        (None, None) => return (None, None, None, 0),
        (Some(left), None) => (left.val + carry, left.next, None),
        (Some(left), Some(right)) => (left.val + right.val + carry, left.next, right.next),
        (None, Some(right)) => (right.val + carry, None, right.next),
    };

    let carry = sum / 10;
    let val = sum - carry;

    let mut digit = Box::new(ListNode::new(val));

    (Some(digit), l1_next, l2_next, carry)
}

