use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value, next }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        let mut ret = Node::new(self.value.clone(), None);
        if !self.next.is_none() {
            ret.next = self.next.clone()
        }
        ret
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        let mut ret: LinkedList<T> = LinkedList::new();
        ret.head = self.head.clone();
        ret.size = self.size;
        ret
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, rhs: &Self) -> bool {
        if self.size != rhs.size {
            return false;
        }
        let mut p = self.head.as_ref();
        let mut q = rhs.head.as_ref();
        loop {
            if p.is_none() && q.is_none() {
                return true;
            }
            if p.unwrap().value != q.unwrap().value {
                return false;
            }
            p = p.unwrap().next.as_ref();
            q = q.unwrap().next.as_ref();
        }
    }
}

impl<T> Iterator for LinkedList<T>
where
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.pop_front() // updates self && returns front item
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(ptr) => {
                let ret = &ptr.as_ref().value;
                self.current = &ptr.as_ref().next;
                Some(ret)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {
            current: &self.head,
        }
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<u32> {
    fn compute_norm(&self) -> f64 {
        let mut sum: u64 = 0;
        for x in self {
            let t: u64 = *x as u64;
            sum += t * t;
        }
        (sum as f64).sqrt()
    }
}
