use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    id: usize,
    left_fork: Arc<Mutex<usize>>,
    right_fork: Arc<Mutex<usize>>,
}

impl Philosopher {
    fn new(id: usize, left_fork: Arc<Mutex<usize>>, right_fork: Arc<Mutex<usize>>) -> Self {
        Philosopher {
            id,
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        // Try to pick up the left fork
        let left = self.left_fork.lock().unwrap();
        // Try to pick up the right fork
        let right = self.right_fork.lock().unwrap();

        thread::sleep(Duration::from_millis(1000)); // Simulate eating time
        println!(
            "Philosopher {} is eating with forks {} and {}.",
            self.id, *left, *right
        );
    }

    fn think(&self) {
        thread::sleep(Duration::from_millis(1000)); // Simulate thinking time
        println!("Philosopher {} is thinking.", self.id);
    }

    fn dine(&self) {
        loop {
            self.think();
            self.eat();
        }
    }
}

fn main() {
    // create five forks (each a distinct Arc<Mutex<_>)
    let fork0 = Arc::new(Mutex::new(0));
    let fork1 = Arc::new(Mutex::new(1));
    let fork2 = Arc::new(Mutex::new(2));
    let fork3 = Arc::new(Mutex::new(3));
    let fork4 = Arc::new(Mutex::new(4));

    // create five philosophers, wiring the forks manually
    let phil0 = Philosopher::new(0, fork0.clone(), fork1.clone());
    let phil1 = Philosopher::new(1, fork1.clone(), fork2.clone());
    let phil2 = Philosopher::new(2, fork2.clone(), fork3.clone());
    let phil3 = Philosopher::new(3, fork3.clone(), fork4.clone());
    let phil4 = Philosopher::new(4, fork4.clone(), fork0.clone());

    // spawn a thread for each philosopher
    let h0 = thread::spawn(move || phil0.dine());
    let h1 = thread::spawn(move || phil1.dine());
    let h2 = thread::spawn(move || phil2.dine());
    let h3 = thread::spawn(move || phil3.dine());
    let h4 = thread::spawn(move || phil4.dine());

    // wait forever (threads run infinite loops)
    let _ = h0.join();
    let _ = h1.join();
    let _ = h2.join();
    let _ = h3.join();
    let _ = h4.join();
}
