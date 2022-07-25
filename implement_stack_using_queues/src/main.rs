
#[derive(Debug)]
struct MyStack {
    stack: Vec<i32>
}

impl MyStack {
    fn new() -> Self {
        MyStack { stack: Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(x) = self.stack.pop() {
            return x;
        }

        return 0;
    }
    
    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }
    
    fn empty(&self) -> bool {
        self.stack.len() == 0
    }
}

fn main() {
    let mut my_stack = MyStack::new();
    
    my_stack.push(1);
    my_stack.push(2);
    
    println!("Pop operation: {}", my_stack.pop());
    println!("Top operation: {}", my_stack.top());
    println!("Is stack Empty: {}", my_stack.empty());

    println!("{:?}", my_stack);
}
