# 笔记

- Cargo 有一个很棒的功能是：运行 cargo doc −−open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
- 每个包叫做一个crate.
- trait 是一个关键的名词，它看起来像是一种依赖，引入依赖才可以调用crate中的方法。
- Result ： 
  - 一个结果，可能错误，可以调用.expect方法
  - 可以用Result{Ok() => {} , Err() => {}} 对错误和正确的结果进行处理
  - Result{} 中可以调用任意的枚举值来对结果进行处理，这使它起到了swich的作用
- u32 : 无符号32位整数，i32：有符号32位整数
- let mut a + let a : 隐藏可变值a
- match 与 switch 的异同点
  - match返回Result
- Ordering 枚举类型
  - js的枚举类：Ordering.a
  - rust的枚举类：Ordering::a
- 所有的格式转换都用 parse() 来做，依靠自动推断
- loop = while(1)