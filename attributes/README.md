# attributes

属性是应用于某个模块、板条箱或项目的元数据。此元数据可用于/用于：

* [代码的条件编译](https://doc.rust-lang.org/stable/rust-by-example/attribute/cfg.html)
* [设置 crate 名称、版本和类型（二进制或库）](https://doc.rust-lang.org/stable/rust-by-example/attribute/crate.html)
* 禁用 [lints](https://en.wikipedia.org/wiki/Lint_%28software%29) (警告)
* 启用编译器功能（宏、全局导入等）
* 链接到 foreign library
* 将函数标记为单元测试
* 标记将成为基准的一部分的函数

当属性应用于整个 crate 时，它们的语法是#![crate_attribute]，当它们应用于模块或项目时，语法是#[item_attribute] （注意缺少的 bang !）。

属性可以采用不同语法的参数：

* `#[attribute = "value"]`
* `#[attribute(key = "value")]`
* `#[attribute(value)]`

属性可以有多个值，也可以用多行分隔：

* `#[attribute(value, value2, value3,
  value4, value5)]`