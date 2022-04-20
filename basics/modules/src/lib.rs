mod visibility;
mod struct_visibility;
mod use_declaration;
mod super_self;
mod file;
mod test_file_method;

///
/// Rust 提供了功能强大的模块系统，可以用于按业务逻辑进行 模块拆分，有点类似java微服务 若干服务
/// 并管理每个模块之间的可见性(public / private)
/// 模块是项目的集合: functions , structs, traits, impl 甚至其他模块都可以进行修饰
///


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}