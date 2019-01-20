use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (moves, _credits, _debits) = debits_and_credits(&root);
        moves
    }
}

// Edits debits and credits, and returns the amount of moves used
fn debits_and_credits(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Vec<i32>, Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();

        let (left_moves, left_debits, left_credits) = debits_and_credits(&node.left);
        let (right_moves, right_debits, right_credits) = debits_and_credits(&node.right);

        let mut debits = Vec::new();
        let mut credits = Vec::new();
        for n in left_credits.iter().chain(right_credits.iter()) {
            credits.push(n+1);
        }
        for n in left_debits.iter().chain(right_debits.iter()) {
            debits.push(n+1);
        }

        if node.val > 1 {
            for _ in 0..node.val-1 {
                credits.push(0);
            }
        } else if node.val == 0 {
            debits.push(0);
        }

        let mut move_count = 0;
        while debits.len() > 0 && credits.len() > 0 {
            let debit = debits.pop().unwrap();
            let credit = credits.pop().unwrap();
            move_count += debit + credit;
        }
        (move_count + left_moves + right_moves, debits, credits)
    } else {
        (0, Vec::new(), Vec::new())
    }
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    use super::*;

    fn check(input: TreeNode, e: i32) {
        let output = Solution::distribute_coins(input.to_child());
        assert_eq!(output, e);
    }

    #[test]
    fn example_1() {
        let mut input = TreeNode::new(3);
        input.left = TreeNode::new(0).to_child();
        input.right = TreeNode::new(0).to_child();

        let expected = 2;
        check(input, expected);
    }

    #[test]
    fn example_2() {
        let mut input = TreeNode::new(0);
        input.left = TreeNode::new(3).to_child();
        input.right = TreeNode::new(0).to_child();

        let expected = 3;
        check(input, expected);
    }

    #[test]
    fn example_3() {
        let mut input = TreeNode::new(1);
        input.left = TreeNode::new(0).to_child();
        input.right = TreeNode::new(2).to_child();

        let expected = 2;
        check(input, expected);
    }

    #[test]
    fn example_4() {
        let mut input = TreeNode::new(1);

        let mut left = TreeNode::new(0);
        left.left = TreeNode::new(3).to_child();
        input.left = left.to_child();

        input.right = TreeNode::new(0).to_child();

        let expected = 4;
        check(input, expected);
    }

    // Definitions
    pub struct Solution;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }

        #[inline]
        pub fn to_child(self: Self) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(self)))
        }
    }
}
