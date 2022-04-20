mod common;

#[test]
fn it_adder_two() {
    // 调用通用模块测试文件
    common::setp();
    assert_ne!(3, 2+2);
}