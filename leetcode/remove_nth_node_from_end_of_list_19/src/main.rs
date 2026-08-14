fn main() {
    let test_1: (Option<Box<ListNode>>, i32) = (ListNode::list(vec![1, 2, 3, 4, 5]), 2);
    let test_2: (Option<Box<ListNode>>, i32) = (ListNode::list(vec![1]), 1);
    let test_3: (Option<Box<ListNode>>, i32) = (ListNode::list(vec![1, 2]), 1);

    let result_1: Option<Box<ListNode>> = Solution::remove_nth_from_end(test_1.0, test_1.1);
    let result_2: Option<Box<ListNode>> = Solution::remove_nth_from_end(test_2.0, test_2.1);
    let result_3: Option<Box<ListNode>> = Solution::remove_nth_from_end(test_3.0, test_3.1);

    assert_eq!(result_1, ListNode::list(vec![1, 2, 3, 4, 5]));
    assert_eq!(result_2, ListNode::list(vec![]));
    assert_eq!(result_3, ListNode::list(vec![1]));
}

#[derive(PartialEq, Eq, Debug)]
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        // 1. Compter le nombre de nodes
        let mut len = 0;
        let mut current = dummy.next.as_ref();

        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        // Position du node à supprimer depuis le début
        let target = len - n as usize;

        // 2. Aller jusqu'au node JUSTE AVANT celui à supprimer
        let mut current = &mut dummy;

        for _ in 0..target {
            current = current.next.as_mut().unwrap();
        }

        // 3. Supprimer current.next
        if let Some(mut node_to_remove) = current.next.take() {
            current.next = node_to_remove.next.take();
        }

        dummy.next
    }
}
