pub struct Calculator {
    stack: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { stack: vec![] }
    }

    pub fn push(&mut self, value: String) {
        self.stack.push(value);
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }

    pub fn stack_iter(&self) -> impl Iterator<Item = &String> {
        self.stack.iter().rev()
    }
}
