use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

use std::borrow::BorrowMut;

use crate::List::{Cons, Nil};

fn hello(s: &str) {
    println!("{}", s);
}

fn main() {
    println!("Hello, world!");

    let b = Box::new("string");
    println!("{}", b);

    let data = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // 解引用其实就是跟踪引用跳转到它指向的值
    let x = 6;
    let y = &x;

    assert_eq!(6, x);
    assert_eq!(*y, 6);

    let a = String::from("hello");
    let b = Box::new(a);

    assert_eq!(*b, "hello");

    let c = 5;
    let d = MyBox::new(c);

    assert_eq!(*d, 5);
    assert_eq!(*(d.deref()), 5);

    let s1 = String::from("test");
    hello(&s1);

    let s2 = Box::new("james");
    hello(&s2[2..]);
}
