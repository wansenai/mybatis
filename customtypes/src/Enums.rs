use crate::Enums::CodeEnum::COORDINATE;

///
/// 自定义创建Enums 枚举类型
/// enums 允许创建几个不同类型的的构造体
/// 任何作为结构structures有效的变量也可以作为枚举有效
///

enum CodeEnum {
    SUCCESS,
    ERROR,
    WARN,
    INFO,
    SEX(char),
    HOBBY(String),
    // 坐标
    COORDINATE {
      x : f32,
      y : f32,
    },
}


fn inspect(event: CodeEnum){
    match event {
        CodeEnum:: SUCCESS  => println!("200"),
        CodeEnum:: ERROR    => println!("500"),
        CodeEnum:: WARN     => println!("400"),
        CodeEnum:: INFO     => println!("300"),
        CodeEnum:: SEX(c) => println!("{:?}", c),
        CodeEnum:: HOBBY(s) => println!("\"{}\"", s),
        CodeEnum::COORDINATE { x, y} => {
            println!("x坐标={}, y坐标={}", x, y);
        },
        _ => {
            println!("~~~")
        }
    }
}

#[test]
fn main(){
    let code_one    =   CodeEnum::SUCCESS;
    let code_two    =   CodeEnum::ERROR;
    let core_three  =   CodeEnum:: WARN;
    let sex         =   CodeEnum::SEX('男');
    // to_owned() 从字符串片段创建一个String
    let hobby       =   CodeEnum::HOBBY("保龄球".to_owned());


    inspect(code_one);
    inspect(code_two);
    inspect(core_three);
    inspect(CodeEnum::INFO);
    inspect(sex);
    inspect(hobby);
    inspect(COORDINATE {
        x   :   1578.3251,
        y   :   -984.1535,
    })
}