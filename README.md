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
- isize、usize : arr.len()的返回类型
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

### 对象

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

和ES6语法相当相似：

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // rust中 ..必须放在最后
    };
    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

```

## 所有权

栈是有序的，堆是无序的，堆通过查找内存中的可用空间并放入，其增加了一个搜索操作。

**所有权**主要是为了管理堆，比如减少堆中的重复数据（这在其他语言中被称为垃圾回收）。

**所有权规则：**

1. Rust 中的每一个值都有一个 所有者（owner）
。
1. 值在任一时刻有且只有一个所有者。
2. 当所有者（变量）离开作用域，这个值将被丢弃。


变量的owner发生了改变，如果再去访问原来的变量则会发生错误。为了避免其他变量和函数获取原有变量的所有权，可以使用 `&引用` 符号直接来访问其他变量的值而不用先获取该变量的所有权。

**赋值：**

```rust
// correct
fn owner() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

// wrong

fn main() {
  let s1 = String::from("hello");
  let s2 = s1;
  println!("s1 = {}, s2 = {}", s1, s2);

}

// correct
fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);

}
```

**函数**

```rust
//wrong
fn scope() {
    let s = String::from("hello");
    // s 进入作用域
    takes_ownership(s);// s 的值移动到函数里 ...

    println!("{s}");
}

fn takes_ownership(s: String){
    
}


//correct
fn scope() {
    let s = String::from("hello");
    // s 进入作用域
     takes_ownership(&s);

    println!("{s}");
}

fn takes_ownership(s: &String) -> usize{
    s.len()
}
```

**修改**

```rust

// wrong
fn main() {
    let mut s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

// corrent
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

```

变量只能被一个人借走（不允许两个写者）：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // 报错

    println!("{}, {}", r1, r2);
}

```

读得时候不能写：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 编译错误

    println!("{}, {}, and {}", r1, r2, r3);
}

```

作用域的范围到变量的最后一次引用位置为止，以下这段函数是正确的，因为 `r1,r2` 两个读者退出了作用域：

```rust
// wrong
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

```

js中的闭包在rust中是不允许的，无法通过编译：

```rust
//wrong
fn main() {
    let reference_to_nothing = dangle();
}

// 虽然s在函数体结束后就应该被释放，但函数却抛出它给外部函数
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// current

fn main() {
    let string = no_dangle();
}

// 返回一个新的值给外部函数，而非一个指针，指向会在函数结束后被释放的值。
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

## 打印方法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}

```

## 结构体 

rust中没有class，用struct执行class的功能。

**和js不同，要在对象中定义函数，需要使用`impl`关键字：**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // 默认将对象自身的引用作为第一个参数传入
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

**静态方法**

使用`struct.fn()` 会自动将对象作为函数的第一个参数引入，使用 `struct::fn()` 则不会：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn generate_square(size: u32) -> Self {
        Self { // Self即Rectangle，看看下面第一种生成结构体的方法，两者写法一致
            width: size,
            height: size,
        }
    }
}
fn main() {
  // 这两种写法相同
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle::generate_square(30);

}
```

## 枚举

枚举 = struct + interface.

你可以单纯将枚举当作某一类型的常量来使用，这使它起到了接口的作用：

```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind, // 将枚举当作接口类型来使用
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4, // 分配枚举常量
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

我们也可以给枚举值分配一个变量，这使枚举的用法更像struct，比如：

```rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

给枚举值也可以接受结构体：


```Rust
fn main() {
  struct Ipv4Addr {
      // --snip--
  }

  struct Ipv6Addr {
      // --snip--
  }

  enum IpAddr {
      V4(Ipv4Addr),
      V6(Ipv6Addr),
  }
}
```

## 空值、枚举与match（待完善）

rust中对空值的定义非常的严格，因为空值是引起运行错误的一个重要来源。因此其作出规定，只有一种变量可能为空，那就是 `Option` 型变量。

```rust
#![allow(unused)]
fn main() {
  enum Option<T> {
      None,
      Some(T),
  }
}

fn main() {
    let some_number = Some(5); // 推断类型为Option<number>
    let some_char = Some('e'); // 推断类型为Option<char>

    let absent_number: Option<i32> = None; // 必须提前定义空值的类型
}
```

要使用 `Option<T>` 枚举对象，必须提前将其转换为 `T` ，即**非空值**来运行。

总结：所有的空值都保存在 `Option` 枚举对象中，这给程序的运行带来了安全。


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

字符串：

```rust
    let s : str = "Hello, world!"; // 编译失败 
    let s1 = "Hello, world!"; // s的类型推断是&str，它是一个引用，它不可改变
    let s2 = String::from("Hello, world!"); // s的类型是String，它是一个值，它可被改变
    let s3 = &s2[..]; // 将s2作为一个整体slice返回，他的类型推断是&str
```

为什么要区分出 `String` 和 `&str` ？单单是 `String` 可变，`&str` 不可变并不是一个好理由。因为 `let` 与 `let mut` 已经帮我们对变量是否可以被改变做了处理。

试想一下，如果没有 `&str` ，只有 `String` ，那么通过 `slice` 切出的子字符串同样是 `String` ，也就是说 `String` 可以派生出许多其他的可变类型，这违反了 rust 内存绝对安全的原则。

