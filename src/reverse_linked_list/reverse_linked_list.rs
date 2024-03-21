#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/// # Reverse Linked List
/// ## Arguments
/// * `head` - linked list to reverse;
///
/// Given the head of a singly linked list, reverse the list, and return the reversed list.

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut current = head;
    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = result.take();
        result = Some(node);
        current = next;
    }
    return result;
}

