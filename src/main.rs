
mod queue {
    use std::collections::VecDeque;

    pub struct Queue<T> {
        elements: VecDeque<T>
    }

    impl<T> Queue<T> {
        pub fn new() -> Queue<T> {
            Queue {
                elements: VecDeque::new()
            }
        }

        pub fn enqueue(&mut self, value: T) {
            self.elements.push_back(value);
        }

        pub fn dequeue(&mut self) -> Option<T> {
            self.elements.pop_front()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }
    }
}

fn main() {
    use queue::Queue;

    let mut queue = Queue::new();
    println!("Queue is empty: {}", queue.is_empty());
    queue.enqueue(5);
    queue.enqueue(10);
    queue.enqueue(15);
    println!("Queue is empty: {}", queue.is_empty());
    println!("Dequeueing Items");
    while let Some(item) = queue.dequeue() {
        println!("Item: {}", item);
    }
    println!("Queue is empty: {}", queue.is_empty());
}