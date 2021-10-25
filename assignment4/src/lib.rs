use std::fmt;

pub struct LinkedList<T> {
    head: Option<Node<T>>,
    len: i32,
}

pub struct Node<T> {
    pub next: Option<Box<Node<T>>>,
    pub data: T,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> LinkedList<T> {
    pub fn push_unique(&mut self, item: T) -> Result<(), String> {
        if self.head.is_none() {
            self.len += 1;
            self.head = Some(Node {
                next: None,
                data: item,
            });
            return Ok(());
        }

        let mut curr = self.head.as_mut().unwrap();
        while curr.next.is_none() {
            if curr.data == item {
                return Err("Data is already present in list".to_string());
            }
            curr = curr.next.as_mut().unwrap();
        }

        self.len += 1;
        curr.next = Some(Box::new(Node {
            next: None,
            data: item,
        }));
        Ok(())
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.head.as_ref().unwrap().data, self.len)
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&mut self) -> i32 {
        self.len
    }

    pub fn is_empty(&mut self) -> bool {
        if self.len == 0 {
            return true;
        }
        false
    }

    pub fn push(&mut self, item: T) {
        self.len += 1;
        if self.head.is_none() {
            self.head = Some(Node {
                next: None,
                data: item,
            });
            return;
        }
        let mut curr = self.head.as_mut().unwrap();
        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }

        curr.next = Some(Box::new(Node {
            next: None,
            data: item,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;

        let holder = &mut self.head;
        if self.len == 1 {
            let Node { data, .. } = holder.take().unwrap();
            self.len -= 1;
            return Some(data);
        }

        let mut holder = &mut self.head.as_mut().unwrap().next;
        while holder.as_mut().unwrap().next.is_some() {
            holder = &mut holder.as_mut().unwrap().next;
        }
        let Node { data, .. } = *holder.take().unwrap();
        self.len -= 1;
        Some(data)
    }
}
