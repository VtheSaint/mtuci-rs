use std::{rc::Rc, cell::RefCell};



fn main() {
    let v = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5] ));
    let a = A{vector: v.clone()};
    let b = A{vector: v.clone()};
    a.vector.borrow_mut().push(6);
    println!("{:?}", b.vector.borrow());
    println!("{:?}", a.vector.borrow());
}


struct A {
    vector: Rc<RefCell<Vec<i32>>>
}