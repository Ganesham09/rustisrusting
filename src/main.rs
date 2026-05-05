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


use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    let b = Rc::new(RefCell::new(Node { next: None }));

    // Create a cycle → memory leak
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(a.clone());

    println!("Leaking memory via Rc cycle!");
}