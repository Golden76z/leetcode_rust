fn main() {}

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);

        let current_min = match self.min_stack.last() {
            Some(&min) => min.min(value),
            None => value,
        };

        self.min_stack.push(current_min);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        if let Some(top) = self.stack.last() {
            *top
        } else {
            0
        }
    }

    fn get_min(&self) -> i32 {
        if let Some(min) = self.min_stack.last() {
            *min
        } else {
            0
        }
    }
}

// Your MinStack object will be instantiated and called as such:
// let obj = MinStack::new();
// obj.push(value);
// obj.pop();
// let ret_3: i32 = obj.top();
// let ret_4: i32 = obj.get_min();
