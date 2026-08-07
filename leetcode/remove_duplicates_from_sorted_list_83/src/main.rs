fn main() {
    let test_1: Option<Box<ListNode>> = ListNode::list(vec![1, 1, 2]);
    let test_2: Option<Box<ListNode>> = ListNode::list(vec![1, 1, 2, 3, 3]);

    let result_1: Option<Box<ListNode>> = Solution::delete_duplicates(test_1);
    let result_2: Option<Box<ListNode>> = Solution::delete_duplicates(test_2);

    assert_eq!(result_1, ListNode::list(vec![1, 2]));
    assert_eq!(result_2, ListNode::list(vec![1, 2, 3]));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

type List = Option<Box<ListNode>>;

impl ListNode {
    fn new(value: i32) -> Self {
        ListNode { value, next: None }
    }

    fn list(nums: Vec<i32>) -> List {
        let mut current: Option<Box<ListNode>> = None;
        for &i in nums.iter().rev() {
            let mut node: ListNode = ListNode::new(i);
            node.next = current;
            current = Some(Box::new(node));
        }

        current
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            if let Some(next) = node.next.as_mut()
                && node.value == next.value
            {
                node.next = next.next.take();
                current = Some(node);
            } else {
                current = node.next.as_mut();
            }
        }

        head
    }
}
