use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct StackUpError;

#[derive(Debug)]
pub struct UnstackUpError;

impl Error for StackUpError {}

impl Error for UnstackUpError {}

impl Display for StackUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Couldn't stack up element because stack was full")
    }
}

impl Display for UnstackUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Couldn't unstack up element because stack was empty")
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        return self.content.pop();
    }
}

impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for el in &self.content {
            s.push_str(format!("{}\n", el).as_str());
        }
        s.pop();
        return f.write_str(&s);
    }
}

impl<T:Clone> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Stack::new();
        for item in iter {
            let _= s.stack_up(item);
        }
        return s;
    }
}

#[derive(Clone)]
pub struct Stack<T> {
    pub content: Vec<T>,
    sized: Option<usize> 
}

impl<T: Clone> Stack<T> {

    pub fn new() -> Self {
        return Self {content: Vec::new(), sized: None}
    }

    pub fn with_max_size(max_size: usize) -> Self {
        return Self {content: Vec::new(), sized: Some(max_size)}
    }

    pub fn from_vec_draining(v: &mut Vec<T>) -> Stack<T> {
        let mut s = Stack::new();
        v.drain(..).for_each(|el| {
            let _=s.stack_up(el);
        });
        return s;
    } 

    pub fn stack_up(&mut self, element: T) -> Result<(), StackUpError>  {
        if self.can_stack_up() {
            self.content.push(element);
            return Ok(());
        }
        return Err(StackUpError)
    }

    pub fn unstack_up(&mut self) -> Result<T, UnstackUpError> {
        if self.can_unstack_up() {
            return Ok(self.content.pop().unwrap());
        }
        return Err(UnstackUpError);
    }

    pub fn can_stack_up(&self) -> bool {
        if let None = self.sized {
            return true;
        }
        return self.sized.unwrap() > self.content.len();
    }

    pub fn can_unstack_up(&self) -> bool {
        return self.content.len() > 0;
    }

    pub fn transform<F, G:Clone>(&mut self, mut f: F) -> Stack<G>
    where F: FnMut(T) -> G {
        let mut s = Stack::new();
        while let Some(el) = self.content.pop() {
            s.stack_up(f(el)).unwrap();
        }
        return s;
    }

}


mod stack {
    use std::{any::type_name, ops::Range};

    use crate::types::stack::Stack;

    #[test]
    fn should_create_sized_stack() {
        const SIZE: usize=12;
        const LAST_ELEMENT: i32=13;
        let mut stack : Stack<i32>= Stack::with_max_size(SIZE);
        for i in 0..=12 {
            let _=stack.stack_up(i);
        }

        let res = stack.stack_up(LAST_ELEMENT);
        assert!(res.is_err());
    }


    #[test]
    fn should_create_default_stack() {
        let elements: Range<_> = 0..100;
        let mut stack = Stack::new();
        for element in elements {
            stack.stack_up(element).expect("Should stack up");
        }
    }


    #[test]
    fn should_give_error_when_unstack_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();
        let res = stack.unstack_up();
        assert!(res.is_err());
    }

    #[test]
    fn should_transform() {
        let mut stack = Stack::new();
        let elements = 0..100;
        elements.clone().for_each(|el| {stack.stack_up(el).unwrap()});
        stack.transform(|el| {format!("{}", el)});
    }
}