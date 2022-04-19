// 创建链表 枚举常见的用法是创建链表

// 不提醒为引用代码的警告
#![allow(dead_code)]

use crate::linked_list::List::*;

enum List {
    // 元组结构,包含了一个元素和指向下一个节点的指针
    Cons (u32, Box<List>),
    // 表示链表结束的节点
    Nil,
}

// 方法可以附加到枚举
impl List {
    // 创建一个空的list

    fn new() -> List {
        // Nil的类型是List
        Nil
    }

    // 使用一个列表并且返回相同的列表 新元素在前面
    fn prepend(self, elem: u32) -> List {
        // Cons有类型列表
        Cons(elem, Box::new(self))
    }

    // 返回列表的长度
    fn len(&self) -> u32 {
        // self必须匹配因此方法行为取决于self变体
        // `self` has type `&List`, 和 `*self` has type `List`
        // 匹配具体类型T优先于引用&T上的匹配项
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            // 空列表长度为0
            Nil => 0
        }
    }

    fn stringfiy(&self) -> String {

        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringfiy())
            },

            Nil => {
                format!("Nil")
            },
        }
    }
}

#[test]
fn main(){
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked的长度: {}", list.len());
    println!("Linked的元素: {}", list.stringfiy())
}