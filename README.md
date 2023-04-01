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
- isize、usize : arch平台的有符号数和无符号数
- panic : 特有名词，程序因为“恐慌”而崩溃退出


## 函数

**函数的返回值等同于函数体最后一个表达式的值**。使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。

```rust
fn five() -> i32 { // 箭头指向返回值的类型
5
}
```

## 循环

### loop
loop = while(1).

**从loop返回值可以在break后跟一个语句**

```rust
let result = loop {
  counter += 1;
  if counter == 10 {
    break counter * 2;
  }
};

```

**循环标签**

你可以用`'name` 给loop取名字，这样在多重循环中就可以直接跳出外层循环。

```rust
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
```

### for 

```rust
let a = [10, 20, 30, 40, 50];
for element in a {
  println!("the value is: {element}");
}
```

## for_each , |_|


# 易错点

## 覆盖与赋值
- let a = "   " , let a = a.len() ： 覆盖可以处理不同的数据类型 
- let mut a = "    " , a = a.len() ：重新赋值，报错，number不能赋值给string

## 元组、数组与vector

### 元组
- 元组的长度固定
- 元组每个位置的类型不用相同

**元组解构语法：**

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```


**像对象一样访问元组**

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
```

### 数组

- 数组的长度固定

**基本使用**

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3,3,3,3,3]
```

## 表达式与语句

```rust
let y = {
  let x = 3;
  x + 1 // 没有分号是表达式，作为花括号的return值
};

let y = {
  let x = 3;
  x + 1; // 有分号是语句，没有任何返回值
};

```

## 控制流

if没有括号：

```rust
if number < 5 {
println!("condition was true");
}   
```

不会自动处理布尔值：

```rust
let number = 3;
if number { // 报错， expected `bool`, found integer；其他语言会自动将number从integer转化为bool，而rust不会
  println!("number was three"); 
}

if number != 0 {
  println!("number was something other than zero");
}

```

三目运算符：

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
let number = if condition { 5 } else { "six" }; // 类型不兼容
```