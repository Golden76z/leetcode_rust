fn main() {
    let test_1: Option<Box<ListNode>> = ListNode::list(vec![1, 2, 3, 4, 5]);
    let test_2: Option<Box<ListNode>> = ListNode::list(vec![1, 2, 3, 4, 5, 6]);

    let result_1: Option<Box<ListNode>> = Solution::middle_node(test_1);
    let result_2: Option<Box<ListNode>> = Solution::middle_node(test_2);

    assert_eq!(result_1, ListNode::list(vec![3, 4, 5]));
    assert_eq!(result_2, ListNode::list(vec![4, 5, 6]));
}

#[derive(Eq, PartialEq, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;

        for &i in nums.iter().rev() {
            let mut node = ListNode::new(i);
            node.next = current;
            current = Some(Box::new(node));
        }

        current
    }
}

struct Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_ref();
        let mut count = 0;

        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }

        for _ in 0..count / 2 {
            head = head.unwrap().next;
        }

        head
    }
}
