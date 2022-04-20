use std::ops::Deref;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    
        let percentage_max = self.value as f64 / self.max as f64;
    
        if percentage_max >= 1.0 {
            self.messenger.send("错误：你已超出指标");
        } else if percentage_max >= 0.9 {
            self.messenger.send("紧急警告：你已经用完了90%的指标")
        } else if percentage_max >= 0.75 {
            self.messenger.send("警告：你已经用完了75%的指标")
        }
    }
}

struct MyBox<T> (T);

#[allow(dead_code)]
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

#[allow(dead_code)]
fn hello(s: &str) {
    println!("{}", s);
}

#[derive(Debug)]
struct CustomDropPointer {
    data: String
}

impl Drop for CustomDropPointer {
    fn drop(&mut self) {
        println!("test drop pointer :{}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::rc::Weak;
    use std::cell::RefCell;

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    enum List2 {
        Cons2(i32, Rc<List2>),
        Nil2,
    }
    
    #[derive(Debug)]
    enum List3 {
        Cons3(Rc<RefCell<i32>>, Rc<List3>),
        Nil3,
    }

    #[derive(Debug)]
    enum List4 {
        Cons4(i32, RefCell<Rc<List4>>),
        Nil4,
    }

    use List::{Cons, Nil};
    use List2::{Cons2, Nil2};
    use List3::{Cons3, Nil3};
    use List4::{Cons4, Nil4};

    impl List4 {
        fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
            match self {
                Cons4(_, item) => Some(item),
                Nil4 => None,
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_box_pointer() {
        let b = Box::new("string");
        println!("{}", b);
    
        let _data = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        // 解引用其实就是跟踪引用跳转到它指向的值
        let x = 6;
        let y = &x;
    
        assert_eq!(6, x);
        assert_eq!(*y, 6);
    
        let a = String::from("hello");
        let b = Box::new(a);
    
        assert_eq!(*b, "hello");
    }

    #[test]
    fn test_my_pointer() {
        let c = 5;
        let d = MyBox::new(c);
    
        assert_eq!(*d, 5);
        assert_eq!(*(d.deref()), 5);
    }

    #[test]
    fn test_string_slice() {
        let s1 = String::from("test");
        hello(&s1);
    
        let s2 = Box::new("james");
        hello(&s2[2..]);
    }

    #[test]
    fn test_drop_trait() {
        let s3 = CustomDropPointer{data: String::from("test")};
        println!("{:?}",s3);
        drop(s3);
        println!("CustomDropPointer created..")
    }

    #[test]
    fn test_rc_pointer() {
        let s1 = Rc::new(Cons2(5, Rc::new(Cons2(6, Rc::new(Cons2(7, Rc::new(Nil2)))))));
        println!("s1 rc 引用计数: {}", Rc::strong_count(&s1));

        let _s2 = Cons2(99, Rc::clone(&s1));
        println!("s1 rc 引用计数: {}", Rc::strong_count(&s1));

        {
            let _s3 = Cons2(666, Rc::clone(&s1));
            println!("s1 rc 引用计数: {}", Rc::strong_count(&s1));
        }

        println!("s3离开作用域后的引用计数: {}", Rc::strong_count(&s1));
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![])}
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {

            let mut one_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            
            // run error panic 违反借用规则
            // 虽然RefCell<T> 不在编译器间进行多个可变引用多检查，但是在运行阶段 如果出现多个可变引用 会导致数据不一致 这里运行期就会报错 触发panic宏
            // let mut two_borrow = self.sent_messages.borrow_mut();
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn test_message_example_75_percent() {
        let mok_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mok_messenger, 100);

        limit_tracker.set_value(90);

        assert_eq!(mok_messenger.sent_messages.borrow_mut().len(), 1);
    }

    #[test]
    fn test_nesting_pointer() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

        let b = Cons3(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons3(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("c = {:?}", c);
    }

    #[test]
    fn test_error_for_reference() {
        let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));

        println!("a rc 引用计数: {}", Rc::strong_count(&a));
        println!("a 下一项: {:?}", a.tail());

        let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));

        println!("a rc 引用计数: {}", Rc::strong_count(&a));
        println!("b rc 引用计数: {}", Rc::strong_count(&b));
        println!("a 下一项: {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b)
        }

        println!("a rc 引用计数: {}", Rc::strong_count(&a));
        println!("b rc 引用计数: {}", Rc::strong_count(&b));
        
        // 取消这段注释代码会造成栈溢出
        // println!("a 下一项: {:?}", a.tail());
    }

    #[test]
    fn test_node_week_pointer() {
        let leaf = Rc::new(Node{
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        println!("1. 叶子树 父节点: {:?}", leaf.parent.borrow().upgrade());
        println!("1. leaf 强引用计数:{}，弱引用计数:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node{
                value: 5,
                children: RefCell::new(vec![Rc::clone(&leaf)]),
                parent: RefCell::new(Weak::new()),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("branch 强引用计数:{}，弱引用计数:{}", Rc::strong_count(&branch), Rc::weak_count(&branch));
            println!("2. leaf 强引用计数:{}，弱引用计数:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        }

        println!("2. 叶子树 父节点: {:?}", leaf.parent.borrow().upgrade());
        println!("3. leaf 强引用计数:{}，弱引用计数:{}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}