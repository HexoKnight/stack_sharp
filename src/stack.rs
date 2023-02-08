pub struct Stack<T> {
    stack : Vec<T>
}

//: stack methods
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn pop_multiple(&mut self, amount: usize) {
        self.stack.truncate(self.stack.len().saturating_sub(amount));
    }
    // pub fn remove_range(&mut self, from_top: usize , amount: usize) {

    // }
    pub fn push_multiple(&mut self, values: impl IntoIterator<Item = T>) {
        self.stack.extend(values);
    }

    pub fn try_peek(&self, from_top: usize) -> Option<T> where T: Copy {
        if from_top < self.stack.len() {
            Some(self.stack[self.stack.len() - from_top - 1])
        } else {
            super::print_err("stack underflow");
            None
        }
    }
    pub fn try_set(&mut self, from_top: usize, value: T) -> Option<()> {
        if from_top < self.stack.len() {
            let len = self.stack.len();
            self.stack[len - from_top - 1] = value;
            Some(())
        } else {
            super::print_err("stack underflow");
            None
        }
    }

    //pub fn pop(&mut self) -> Option<T> {
    //    self.stack.pop()
    //}
    pub fn try_pop(&mut self) -> Option<T> {
        if let Some(val) = self.stack.pop() {
            Some(val)
        } else {
            super::print_err("stack underflow");
            None
        }
    }
    pub fn try_pop_with_err(&mut self, err: &str) -> Option<T> {
        if let Some(val) = self.stack.pop() {
            Some(val)
        } else {
            super::print_err(err);
            None
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value)
    }
    //fn try_push(&mut self, value: Option<T>) {
    //    if let Some(val) = value {
    //        self.stack.push(val);
    //    }
    //}
}
//;

//: stack display
impl<T> std::fmt::Display for Stack<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for int in &self.stack {
            if let Err(error) = write!(f, "{} ", int) {
                return Err(error);
            }
        }
        if let Err(error) = write!(f, "<") {
            return Err(error);
        }
        Ok(())
    }
}
//;