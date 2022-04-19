///
/// Struct 结构可见性
///
#[allow(dead_code)]
mod one {

    // 泛型类型的T公共字段的公共的结构
    pub struct OpenBox<T> {
        pub contents : T,
    }

    // 泛型类型的T私有字段的公共结构
    pub struct CloseBox<T>{
        contents : T,
    }

    impl<T> CloseBox<T> {
        // 公共构造函数方法
        pub fn new_method(contents : T) -> CloseBox<T> {
            CloseBox {
                contents : contents,
            }
        }
    }
}

#[test]
fn main(){
    let open_box = one::OpenBox {
        contents : "public method"
    };

    println!("调用one模块里的OpenBox方法:{}", open_box.contents);
    let _close_box = one::CloseBox::new_method("public new method");

    // closed_box 的 contents报错，因为他是private的
    // let closed_box = one::CloseBox { contents: "classified information" };
}