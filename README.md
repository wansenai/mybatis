# rust library

此项目主要是对Rust的快速学习，以及读书笔记，我大概是在2021年的春天吧，
有一天我在刷infoQ，看到Rust语言，起初是没怎么感兴趣，后来查阅来一下它的资料，
我发现它可以对每一个变量进行生命周期管控，以及内存管理，然后就开始燃起来了，我平常是在周末学习
Rust，我最初是先看了一些概念，然后看了官方的book，以及学习了microsoft的教程，我把它们整合到
本项目里面，每一个模块，每一段代码 都标有中文代码注释，然后根据自己的理解举一反三进行学习。

## 每个模块的说明

* `attributes` 模块介绍了每个模块或者crate 以及每一项的元数据，我理解为Java 的注解


* `conversion` 模块介绍了Rust 基础数据类型转换，也可以通过traits进行自定义（struct和enum）之间的转换


* `crates` 模块介绍了Rust 的编译，crate可以编译成二进制文件或rust库


* `customtypes` 模块介绍Rust 自定义数据类型，包括常量的创建，struct（Java的类）和enum


* `expressions` 模块介绍Rust code 注释，以及doc文档编写


* `flowofcontrol` 模块介绍Rust if-else条件和 for循环等


* `formatprint` 模块介绍Rust 格式化打印，以及Display `fmt::Debug`的实现


* `function` 模块介绍Rust 函数，通过使用fn关键字创建一个方法，包括高阶函数HOF


* `generics` 模块介绍Rust 泛型，泛型创建和调用以及使用方式


* `journal` 模块是Microsoft Rust的命令行学习


* `leetcode` 模块是使用Rust 进行leetcode上面的算法学习


* `microsoft` 模块主要是学习Microsoft Rust相关的教程


* `modules` 模块介绍Rust多个模块之间方法调用的学习，以及分模块开发


* `primitives` 模块介绍Rust的数据类型和元组学习，基本数据类型的操作


* `types` 模块介绍了Rust可以提供一些机制来更改或定义Rust原始类型和自定义类型


* `varbindings` 模块介绍了Rust 变量绑定，类型注释等


* `webservice` 模块用于第三方库actix-web以及mysql 进行api开发