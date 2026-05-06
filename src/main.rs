// struct Rect {
//    width: u32,
//    height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//       return self.width * self.height
//     }
//     fn perameter(&self) -> u32 {
//         return 2 * (self.width + self.height)
//     }
// }

// fn main() { // when we reach traits we will see how to use them to make this more generic and reusable for other shapes
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
 
//     println!("The area of the rectangle is {}", rect.area());
//     println!("The perimeter of the rectangle is {}", rect.perameter());
// }


// // src/main.rs
// use std::cell::RefCell;
// use std::rc::Rc;

// // A simple node that can point to another node
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: Option<Rc<RefCell<Node>>>,
// }

// fn main() {
//     // --- Leak #1: Rc cycle (real-world pattern) ---
//     let a = Rc::new(RefCell::new(Node { value: 1, next: None }));
//     let b = Rc::new(RefCell::new(Node { value: 2, next: None }));

//     // Create a cycle: a -> b -> a
//     a.borrow_mut().next = Some(b.clone());
//     b.borrow_mut().next = Some(a.clone());

//     // When main ends, both Rc counts never reach 0 due to the cycle.
//     // => memory is never freed.

//     // --- Leak #2: mem::forget (forces a leak) ---
//     let v = vec![10, 20, 30, 40, 50];
//     std::mem::forget(v); // prevents destructor (drop) from running

//     // --- Leak #3: Box::leak (turns heap into 'static ref) ---
//     let boxed = Box::new(999);
//     let _leaked_ref: &'static i32 = Box::leak(boxed);

//     // Use the `value` fields so they are read (avoid dead_code warning)
//     println!("a.value = {}", a.borrow().value);
//     println!("b.value = {}", b.borrow().value);

//     println!("Program executed. Some memory is intentionally leaked.");
// }

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>, // 👈 weak reference (important)
}

fn main() {
    let a = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
        prev: None,
    }));

    let b = Rc::new(RefCell::new(Node {
        value: 2,
        next: None,
        prev: None,
    }));

    // Link a → b (strong reference)
    a.borrow_mut().next = Some(b.clone());

    // Link b → a (weak reference, NOT strong)
    b.borrow_mut().prev = Some(Rc::downgrade(&a));

    println!("a.value = {}", a.borrow().value);
    println!("b.value = {}", b.borrow().value);

    // Everything will be dropped correctly here
    println!("Program executed with NO memory leak ✅");
}