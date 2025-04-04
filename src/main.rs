use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Debug)]
pub struct Node<T: Clone + PartialEq> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T: Clone + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }

    pub fn find(&self, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        if self.is_empty() {
            return None;
        }

        let mut cur = self.head.clone();
        while let Some(n) = cur {
            let item = n.borrow();
            if item.data != data {
                cur = item.clone().next;
            } else {
                return Some(n.clone());
            }
        }

        None
    }

    pub fn push(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data: data,
            next: None,
        }));

        if self.head.is_none() {
            self.head = Some(node.clone());
        }

        if !self.tail.is_none() {
            self.tail.as_mut().unwrap().borrow_mut().next = Some(node.clone());
        }

        self.tail = Some(node.clone());
    }

    pub fn collect(&self) -> Option<Vec<T>> {
        if self.is_empty() {
            return None;
        }

        let mut out = Vec::<T>::new();

        let mut cur = self.head.clone();
        while let Some(n) = cur {
            let item = n.borrow();
            out.push(item.data.clone());
            cur = item.clone().next;
        }

        Some(out)
    }

    pub fn remove(&mut self, data: T) {
        if self.is_empty() {
            return;
        }

        let mut prev = None;
        let mut cur = self.head.clone();
        while let Some(n) = cur {
            let item: std::cell::Ref<'_, Node<T>> = n.borrow();
            if item.data != data {
                prev = Some(n.clone());
                cur = item.clone().next;

                continue;
            }

            if prev.is_none() {
                if item.next.is_none() {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.head = item.next.clone();
                    self.tail = item.next.clone();
                }
            } else {
                if item.next.is_none() {
                    prev.clone().unwrap().borrow_mut().next = None;
                    self.tail = prev.clone();
                } else {
                    prev.clone().unwrap().borrow_mut().next = item.next.clone();
                }
            }

            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.is_empty());

        list.push(2);

        assert!(!list.is_empty())
    }

    #[test]
    fn test_linked_list_find() {
        let mut list = LinkedList::<i32>::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.find(1).unwrap().borrow().data, 1);
        assert_eq!(list.find(2).unwrap().borrow().data, 2);
        assert_eq!(list.find(3), None);
    }

    #[test]
    fn test_linked_list_push_collect() {
        let mut list = LinkedList::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(2);
        list.push(3);
        list.push(4);
        list.remove(2);
        list.remove(4);
        list.remove(5);
        list.push(5);

        assert_eq!(list.collect().unwrap(), vec![1, 2, 3, 5]);
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    for i in numbers.iter() {
        println!("{}", i);
    }

    println!("Hello, world!");
}
