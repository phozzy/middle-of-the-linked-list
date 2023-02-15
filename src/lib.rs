// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct ListNodeIter {
    list: Option<Box<ListNode>>,
}

impl ListNodeIter {
    fn add(&mut self, val: Option<i32>) {
        if let Some(item) = val {
            let current = self.list;
            self.list = Some(Box::new(ListNode {
                val: item,
                next: current,
            }));
        }
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn iter(&self) -> ListNodeIter {
        ListNodeIter {
            list: Some(Box::new(*self)),
        }
    }
}

impl Iterator for ListNodeIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            Some(inner) => {
                let ListNode { val, next } = *inner;
                self.list = next;
                Some(val)
            }
            _ => None,
        }
    }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let list_iter = ListNodeIter { list: head };
    let mut list: Vec<i32> = list_iter.collect();
    let middle = list.len() / 2;
    list.reverse();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
