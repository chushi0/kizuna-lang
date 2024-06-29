# Kizuna Lang

只有一个关键字的编程语言

## 语法
Kizuna Lang支持变量、条件、循环、函数。

### 代码示例
下面是一个计算从1加到100的程序：
```kizuna
kizuna main() {
    kizuna s = 0;
    kizuna i = 0;

    kizuna {
        kizuna (i > 100) {
            kizuna;
        }

        s = s + i;
        i = i + 1;
    }

    println("The sum from 1 to 100 is " + s);
}

main();
```

该程序可近似等价为以下Rust代码：
```rust
fn main() {
    let mut s = 0;
    let mut i = 0;
    
    loop {
        if i > 100 {
            break;
        }

        s = s + i;
        i = i + 1;
    }

    println!("The sum from 1 to 100 is {}", s);
}
```

### 变量

**`kizuna <variable> = <expr-value>;`**

定义新变量。新变量将定义在最近的作用域中。定义变量时必须附带初始值。

该语句允许覆盖当前作用域之前定义的变量。同一作用域内，如果一个变量被定义了两次，那么之后只能访问到最后一次定义的变量。

**`<variable> = <expr-value>;`**

修改变量的值。注意该语句不能用于定义新变量，`=`左边的变量必须之前定义过。

该语句会从最近的作用域开始向上寻找变量，并将其修改。

### 条件判断

**`kizuna ( <expr-value> ) { <exec-if-true> }`**

**`kizuna ( <expr-value> ) { <exec-if-true> } { <exec-if-false> }`**

判断`<expr-value>`的值是否为`true`。如果为`true`，执行`exec-if-true`的代码，否则执行`<exec-if-false>`的代码。

### 循环

**`kizuna { <exec-body> }`**

无限循环执行`<exec-body>`的内容，除非循环被中断。

**`kizuna;`**

中断循环和函数执行。

### 函数

**`kizuna <func-name> ( <param-list> ) { <exec-body> }`**

定义函数。参数列表使用`,`分割，无需标注类型。当函数调用时执行`<exec-body>`。

**`<func-name>( <param-list> )`**

调用函数，结果可作为表达式一部分参与计算，如果需要单独使用需在末尾加`;`。


## 命令行参数

### kizuna build &lt;files...&gt;

编译代码文件，并将编译后的内容保存为二进制文件。程序允许同时编译多个文件，编译后的文件扩展名为`*.kb`。

二进制文件中存储的实质上是AST的二进制压缩，没有任何优化。

### kizuna run &lt;files...&gt;

执行文件。传入的文件必须是编译后的二进制文件。此命令允许传入多个文件，它们将依次执行，并且会共享函数和全局变量。

## 运行时细节

### 值类型

有三种类型：`None`、`String`、`Number`和`bool`。

`None`只在内部使用，不能在代码中定义（因为这需要引入新的关键字）。未定义的变量和没有返回值的函数均为此值。

`String`是字符串，使用Rust的`std::String`实现，因此它不能表示非unicode字符。

`Number`是数字，使用64位浮点数实现。

在需要时，三种类型可以互相转化：

||原类型为`None`|原类型为`String`|原类型为`Number`|
|:--:|:--:|:--:|:--:|
|转为`String`|空字符串|N/A|数字对应的字符串|
|转为`Number`|`0`|字符串对应的数字，如果失败则为`0`|N/A|

#### 布尔值

在条件判断及逻辑运算符时，需要将三种类型转为布尔值。其转换规则如下：

|类型|转换规则|
|:--:|:--:
|`None`|永远为`false`|
|`String`|如果是空字符串则为`false`|
|`Number`|如果大于`0`则为`true`|

逻辑运算符计算完成后，结果会转为数字。这里符合传统，`true`转为`1`，`false`转为`0`。

### 返回值

所有函数均具有返回值，返回值被定义为该函数执行的最后一段代码的结果。

如下示例，函数`main`的最后一条语句是`s + 5`，因此函数的返回值为`8`。
```kizuna
kizuna main() {
    kizuna s = 3;
    s + 5;
}

println(main());
```


### 错误处理

所有错误均静默处理，不报错，代码继续执行。