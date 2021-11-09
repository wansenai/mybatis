///
/// 创建一个library库 然后提供给其他代码引用
///

pub fn get_once_function(){
    println!("获得第一个方法");
}

fn get_Jwt_token(){
    println!("获取token:sad3213zc231vcxv21vsv2v1");
}

pub fn get_coon(){
    get_Jwt_token();
    println!("连接成功~~~");
}

// 这里写完后进入到该目录，进行打包rustc --crate-type=lib Library.rs, 然后同目录会生成lib文件