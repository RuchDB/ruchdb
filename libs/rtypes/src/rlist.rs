use std::cell::RefCell;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Range;
use std::rc::{Rc, Weak};

struct Node<T>
where
    T: Copy + Clone,
{
    pub data: T,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct RList<T>
where
    T: Copy + Clone,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

pub struct Iter<T>
where
    T: Copy + Clone,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Node<T>
where
    T: Copy + Clone,
{
    // Constructs a node with some `data` initializing prev and next to null.
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }

    pub fn append(cur: &mut Rc<RefCell<Node<T>>>, node: Node<T>) {
        let wrap = Rc::new(RefCell::new(node));
        if let Some(next) = cur.clone().borrow().next.clone() {
            wrap.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(Rc::downgrade(&wrap));
        }
        cur.borrow_mut().next = Some(wrap.clone());
        wrap.borrow_mut().prev = Some(Rc::downgrade(&cur));
    }
}

// private methods
impl<T> RList<T>
where
    T: Copy + Clone,
{
    fn push_front_node(&mut self, node: Node<T>) {
        let wrap = Rc::new(RefCell::new(node));
        if let Some(ref mut head) = self.head {
            wrap.borrow_mut().next = Some(head.clone());
            head.borrow_mut().prev = Some(Rc::downgrade(&wrap));
        } else {
            self.tail = Some(wrap.clone());
        }
        self.head = Some(wrap);
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.len -= 1;
            if let Some(next) = head.borrow_mut().next.clone() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }
            head.borrow_mut().next = None;
            head.borrow().data
        })
    }

    fn push_back_node(&mut self, node: Node<T>) {
        let wrap = Rc::new(RefCell::new(node));
        if let Some(ref mut tail) = self.tail {
            tail.borrow_mut().next = Some(wrap.clone());
            wrap.borrow_mut().prev = Some(Rc::downgrade(&tail));
        } else {
            self.head = Some(wrap.clone());
        }
        self.tail = Some(wrap);
        self.len += 1;
    }

    fn pop_back_node(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            self.len -= 1;
            if let Some(prev) = tail
                .borrow_mut()
                .prev
                .clone()
                .and_then(|weak| weak.upgrade())
            {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }
            tail.borrow_mut().prev = None;
            tail.borrow().data
        })
    }

    fn find_node(&self, idx: usize) -> Option<Rc<RefCell<Node<T>>>> {
        let full = self.len;
        let half = full / 2;
        match idx {
            n if n <= half => {
                let mut cur = self.head.clone();
                for _ in 0..idx {
                    if let Some(node) = cur.clone() {
                        cur = node.borrow().next.clone();
                    } else {
                        return None;
                    }
                }
                cur
            }

            _ => {
                let mut cur = self.tail.clone();
                for _ in idx..full {
                    if let Some(node) = cur.clone() {
                        cur = node.borrow().prev.clone().and_then(|weak| weak.upgrade());
                    } else {
                        return None;
                    }
                }
                cur
            }
        }
    }

    fn insert(&mut self, idx: usize, node: Node<T>) {
        let full = self.len;
        match idx {
            0 => self.push_front_node(node),
            n if n > 0 && n < full => {
                let cur = self.find_node(idx - 1);
                if let Some(mut cur) = cur {
                    Node::append(&mut cur, node);
                    self.len += 1
                }
            }
            _ => self.push_back_node(node),
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head.clone(),
            tail: self.tail.clone(),
            len: self.len,
        }
    }
}

// public methods
impl<T> RList<T>
where
    T: Copy + Clone,
{
    // Constructs an empty list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let node = Node::new(data);
        self.push_front_node(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node()
    }

    pub fn push_back(&mut self, data: T) {
        let node = Node::new(data);
        self.push_back_node(node);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node()
    }

    pub fn front(&self) -> Option<T> {
        self.head.clone().map(|node| node.borrow().data)
    }

    pub fn back(&self) -> Option<T> {
        self.tail.clone().map(|node| node.borrow().data)
    }

    pub fn insert_before(&mut self, idx: usize, data: T) {
        let node = Node::new(data);
        self.insert(idx, node);
    }

    pub fn insert_after(&mut self, idx: usize, data: T) {
        let node = Node::new(data);
        self.insert(idx + 1, node);
    }

    pub fn range(&self, r: Range<usize>) -> Vec<T> {
        let len = self.len;
        let Range { start, mut end } = r;
        if len == 0 {
            return Vec::<T>::new();
        }
        if end >= len {
            end = len;
        }
        self.iter().skip(start).take(end - start).collect()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.len {
            None
        } else {
            self.iter().nth(idx)
        }
    }

    pub fn set(&mut self, idx: usize, val: T) -> Option<T> {
        if idx >= self.len {
            None
        } else {
            let cur = self.find_node(idx);
            cur.map(|node| {
                node.replace_with(|old_node| {
                    let mut new_node = Node::new(val);
                    new_node.prev = old_node.prev.take();
                    new_node.next = old_node.next.take();
                    new_node
                })
            })
            .map(|node| node.data)
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        let full = self.len - 1;
        match idx {
            0 => self.pop_front(),
            n if n > full => None,
            n if n == full => self.pop_back(),
            _ => {
                let cur = self.find_node(idx);
                match cur {
                    Some(cur) => {
                        let next = cur.borrow().next.clone();
                        let prev = cur.borrow().prev.clone().and_then(|weak| weak.upgrade());
                        match (prev, next) {
                            (Some(prev), Some(next)) => {
                                prev.borrow_mut().next = Some(next.clone());
                                next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                                Some(cur.borrow().data)
                            }
                            _ => None,
                        }
                    }
                    None => None,
                }
            }
        }
    }

    pub fn trim(&mut self, r: Range<usize>) {
        let len = self.len;
        let Range { start, mut end } = r;
        if end >= len {
            end = len;
        }
        for _ in 0..start {
            self.pop_front();
        }
        for _ in end..len {
            self.pop_back();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for RList<T>
where
    T: Copy + Clone,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for Iter<T>
where
    T: Copy + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.take().map(|head| {
                self.len -= 1;
                if let Some(next) = head.borrow().next.clone() {
                    self.head = Some(next);
                } else {
                    self.tail = None;
                }
                head.borrow().data
            })
        }
    }
}

// Pretty-printing
impl<T> Display for RList<T>
where
    T: Copy + Clone + Display,
{
    fn fmt(&self, w: &mut Formatter) -> Result<(), Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

impl<T> Debug for RList<T>
where
    T: Copy + Clone + Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}
