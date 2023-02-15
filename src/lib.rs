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
            let current = self.list.to_owned();
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
            list: Some(Box::new(self.to_owned())),
        }
    }
}

impl Iterator for ListNodeIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.list.to_owned() {
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
    let middle = list.len() / 2 + list.len() % 2;
    list.reverse();
    let mut rev_iter = list.into_iter();
    let mut mid_list = ListNodeIter { list: None };
    for _ in 0..middle {
        mid_list.add(rev_iter.next());
    }
    mid_list.list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elemnt5() {
        let lnode = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        };
        let mnode = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));
        let res = middle_node(Some(Box::new(lnode)));
        assert_eq!(mnode, res);
    }

    #[test]
    fn elemnt6() {
        let lnode = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode { val: 6, next: None })),
                        })),
                    })),
                })),
            })),
        };
        let mnode = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        }));
        let res = middle_node(Some(Box::new(lnode)));
        assert_eq!(mnode, res);
    }
}
