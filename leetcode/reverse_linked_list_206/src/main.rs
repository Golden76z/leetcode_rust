fn main() {
    let test_1: Option<Box<ListNode>> = ListNode::list(vec![1, 2, 3, 4, 5]);
    let test_2: Option<Box<ListNode>> = ListNode::list(vec![1, 2]);
    let test_3: Option<Box<ListNode>> = ListNode::list(vec![]);

    let result_1: Option<Box<ListNode>> = Solution::reverse_list(test_1);
    let result_2: Option<Box<ListNode>> = Solution::reverse_list(test_2);
    let result_3: Option<Box<ListNode>> = Solution::reverse_list(test_3);

    assert_eq!(result_1, ListNode::list(vec![5, 4, 3, 2, 1]));
    assert_eq!(result_2, ListNode::list(vec![2, 1]));
    assert_eq!(result_3, ListNode::list(vec![]));
}

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>> = None;
        for &i in nums.iter().rev() {
            let mut node = ListNode::new(i);
            node.next = current;
            current = Some(Box::new(node));
        }

        current
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;

        while let Some(mut node) = current {
            let next = node.next;
            node.next = previous;
            previous = Some(node);
            current = next;
        }

        previous
    }
}
