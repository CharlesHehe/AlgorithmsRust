// Definition for singly-linked list.
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

    fn append(&mut self, val: i32) {
        match &mut self.next {
            None => {
                let n = ListNode {
                    val,
                    next: None,
                };
                self.next = Some(Box::new(n));
            }
            Some(ref mut x) => x.append(val),
        }
    }
}


pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut boxed_node) = curr {
        let mut next = boxed_node.next.take();
        boxed_node.next = prev.take();
        prev = Some(boxed_node);
        curr = next.take();
    }
    prev
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut linked_list = ListNode::new(1);
        linked_list.append(2);
        linked_list.append(3);
        linked_list.append(4);
    }
}
