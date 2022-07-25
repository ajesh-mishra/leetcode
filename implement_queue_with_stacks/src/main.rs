
#[derive(Debug)]
struct MyQueue {
    queue: Vec<i32>
}

impl MyQueue {
    fn new() -> Self {
        MyQueue { queue: Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        let mut temp: Vec<i32> = Vec::new();
        temp.push(x);

        for i in &self.queue {
            temp.push(*i);
        }

        self.queue = temp;
    }
    
    fn pop(&mut self) -> i32 {
        if let Some(x) = self.queue.pop() {
            return x;
        }

        unreachable!()
    }
    
    fn peek(&self) -> i32 {
        self.queue[self.queue.len() - 1]
    }
    
    fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}

fn main() {
    let mut my_queue = MyQueue::new();
    
    my_queue.push(1);
    my_queue.push(2);
    my_queue.push(3);

    println!("{}", my_queue.pop());
    println!("{}", my_queue.peek());
    println!("{}", my_queue.empty());

    println!("{:?}", my_queue);
}
