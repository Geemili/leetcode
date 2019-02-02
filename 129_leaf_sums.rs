// Estimated time: 60mins
// Actual time: 33 mins

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut numbers = vec![];
                Self::numbers(0, &node, &mut numbers);
                let mut total = 0;
                for num in numbers {
                    total += num;
                }
                total
            }
            None => {
                0
            }
        }
    }

    fn numbers(num: i32, node: &Rc<RefCell<TreeNode>>, num_list: &mut Vec<i32>) {
        let node = node.borrow();
        let new_num = num * 10 + node.val;
        match (&node.left, &node.right) {
            (Some(left), Some(right)) => {
                Self::numbers(new_num, left, num_list);
                Self::numbers(new_num, right, num_list);
            }
            (Some(path), None) | (None, Some(path)) => {
                Self::numbers(new_num, path, num_list);
            }
            (None, None) => {
                num_list.push(new_num);
            }
        }
    }
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn numbers_1() {
        let tree = TreeNode::new(1)
            .left(TreeNode::new(2))
            .right(TreeNode::new(3));

        let mut numbers = Vec::new();
        Solution::numbers(0, &Rc::new(RefCell::new(tree)), &mut numbers);

        assert_eq!(numbers, &[12, 13]);
    }

    #[test]
    fn example_1() {
        let tree = TreeNode::new(1)
            .left(TreeNode::new(2))
            .right(TreeNode::new(3));

        assert_eq!(Solution::sum_numbers(tree.package()), 25);
    }

    #[test]
    fn example_2() {
        let tree = TreeNode::new(4)
            .right(0)
            .left(TreeNode::new(9).left(5).right(1));

        assert_eq!(Solution::sum_numbers(tree.package()), 1026);
    }

    // Definitions
    pub struct Solution;

    // Definition for a binary tree node.
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl From<i32> for TreeNode {
        fn from(val: i32) -> Self {
            TreeNode::new(val)
        }
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
        pub fn left<N: Into<TreeNode>>(mut self, node: N) -> Self {
            self.left = node.into().package();
            self
        }

        #[inline]
        pub fn right<N: Into<TreeNode>>(mut self, node: N) -> Self {
            self.right = node.into().package();
            self
        }

        #[inline]
        pub fn package(self) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(self)))
        }
    }
}
