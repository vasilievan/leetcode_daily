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

/// # Merge In Between Linked Lists
/// ## Arguments
/// * `list1` - The first linked list;
/// * `a` - starting index;
/// * `b` - finish index;
/// * `list2` - Linked list to insert;
///
/// You are given two linked lists: list1 and list2 of sizes n and m respectively.
/// Remove list1's nodes from the a_th node to the b_th node, and put list2 in their place.

pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = Box::new(ListNode::new(0));
    result.next = list1;
    let mut current = &mut result;
    for _ in 0..a {
        current = current.next.as_mut().unwrap();
    }
    let mut to_append_mid = &mut current.next;
    for _ in a..=b {
        to_append_mid = &mut to_append_mid.as_mut().unwrap().next;
    }
    let to_append_end = to_append_mid.take();
    current.next = list2;
    while !current.next.is_none()  {
        current = current.next.as_mut().unwrap();
    }
    current.next = to_append_end;
    return result.next;
}