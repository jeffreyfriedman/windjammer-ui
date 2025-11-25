struct Counter {
    value: i32,
}

impl Counter {
#[inline]
fn new() -> Counter {
        Counter { value: 0 }
}
#[inline]
fn increment(mut self) -> Counter {
        self.value = self.value + 1;
        self
}
#[inline]
fn set(mut self, value: i32) -> Counter {
        self.value = value;
        self
}
}

fn main() {
    let counter = Counter::new().increment().set(10);
    println!("Counter: {}", counter.value)
}

