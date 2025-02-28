/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    //TODO
    q1: Queue<T>,
    q2: Queue<T>,
    flag: usize,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
            flag: 0,
        }
    }
    pub fn push(&mut self, elem: T) {
        match self.flag {
            0 => {
                self.flag = 1;
                self.q1.enqueue(elem);
            }
            1 => self.q1.enqueue(elem),
            2 => self.q2.enqueue(elem),
            _ => (),
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.flag {
            1 => {
                while self.q1.size() > 1 {
                    if let Ok(val) = self.q1.dequeue() {
                        self.q2.enqueue(val);
                    }
                }
                self.flag = 2;
                let temp = self.q1.dequeue();
                match temp {
                    Ok(_) => return temp,
                    Err(_) => return Err("Stack is empty"),
                }
            }
            2 => {
                while self.q2.size() > 1 {
                    if let Ok(val) = self.q2.dequeue() {
                        self.q1.enqueue(val);
                    }
                }
                self.flag = 1;
                let temp = self.q2.dequeue();
                match temp {
                    Ok(_) => return temp,
                    Err(_) => return Err("Stack is empty"),
                }
            }
            _ => (),
        }

        Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        match self.flag {
            1 => return self.q1.is_empty(),
            2 => return self.q2.is_empty(),
            _ => (),
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
