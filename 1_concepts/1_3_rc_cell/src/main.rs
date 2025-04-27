use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GlobalStack<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        GlobalStack {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }

    fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }

    fn peek(&self) -> Option<T>
    where 
        T: Clone 
    {
        self.data.borrow().last().cloned()
    }

    fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }

    fn len(&self) -> usize {
        self.data.borrow().len()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        GlobalStack { 
            data: Rc::clone(&self.data),
        }
    }
}

fn main() {
    let stack = GlobalStack::new();
    let stack2 = stack.clone();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack2.push(4);

    stack.pop();
    stack.pop();
    println!("{:?}", stack);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let stack = GlobalStack::new();
        assert!(stack.is_empty());
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_clone() {
        let stack1 = GlobalStack::new();
        let stack2 = stack1.clone();

        stack1.push(1);
        stack1.push(2);
        assert_eq!(stack2.pop(), Some(2));
        assert_eq!(stack2.pop(), Some(1));
        assert!(stack1.is_empty());
        assert!(stack2.is_empty());
    }

    #[test]
    fn test_multiple_refs() {
        let stack1 = GlobalStack::new();
        let stack2 = &stack1;
        let stack3 = &stack1;

        stack1.push(1);
        stack2.push(2);
        stack3.push(3);
        
        assert_eq!(stack1.len(), 3);
        assert_eq!(stack2.len(), 3);
        assert_eq!(stack3.len(), 3);
    }
}
