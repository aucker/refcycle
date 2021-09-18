/*enum List {
    Cons(i32, Box<List>),
    Nil,
}*/

//use crate::List::{Cons, Nil};

use std::ops::Deref;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CostomSmartPointer with data '{}'", self.data);
    }
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let x = 3;
    let y = &x;
    assert_eq!(3, x);
    assert_eq!(3, *y);

    let x1 = 3;
    let y1 = MyBox::new(x1);
    assert_eq!(x1, 3);
    assert_eq!(*y1, 3);
    println!("Hello, world!");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));


    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //this will overflow the stack
    println!("a next item = {:?}", a.tail());
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}