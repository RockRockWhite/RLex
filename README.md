# RLex

## 运算符定义

| 符号 | 解释 |
| ---- | ---- |
| （   |      |
| *    |      |
| ｜   |      |
| -   |      |
| ）   |      |

## Mermaid

```regex
(c(abc|b*))
```

上述样例结果如下

```mermaid
graph LR
0((S))
1((1))
2((2))
3((3))
4((4))
5((5))
6((6))
7((7))
8((8))
9((E))
10((10))
11((11))
12((12))
13((13))

8--ε-->9
7--c-->8
6--ε-->7
5--b-->6
4--ε-->5
3--a-->4
2--ε-->3
12--ε-->10
13--ε-->9
12--ε-->13
11--b-->12
10--ε-->11
10--ε-->13
2--ε-->10
1--ε-->2
0--c-->1
```

转换DFA如下：
```mermaid
graph LR
0((0))
1((1-A))
2((2-A))
3((3))
4((4))
5((5-A))

2--b-->2
1--b-->2
4--c-->5
3--b-->4
1--a-->3
0--c-->1
```


```regex
c(a|bbcb*)*(ab)
```

上述样例结果如下

```mermaid
graph LR
0((S))
1((1))
2((2))
3((3))
4((4))
5((5))
6((6))
7((7))
8((8))
9((9))
10((10))
11((E))
12((12))
13((13))
14((14))
15((15))
16((16))
17((17))
18((18))
19((19))
20((20))
21((21))

6--ε-->2
10--b-->11
9--ε-->10
8--a-->9
7--ε-->8
6--ε-->7
5--ε-->6
4--a-->5
3--ε-->4
20--ε-->18
21--ε-->6
20--ε-->21
19--b-->20
18--ε-->19
18--ε-->21
17--ε-->18
16--c-->17
15--ε-->16
14--b-->15
13--ε-->14
12--b-->13
3--ε-->12
2--ε-->3
2--ε-->7
1--ε-->2
0--c-->1
```
DFA如下
```mermaid
graph LR
0((0))
1((1))
2((2))
3((3))
4((4))
5((5))
6((6-A))
7((7))
8((8))

6--b-->3
5--b-->6
5--a-->5
4--a-->5
8--a-->5
8--c-->4
8--b-->8
7--b-->8
7--a-->5
4--b-->7
3--c-->4
2--b-->3
1--b-->2
1--a-->5
0--c-->1
```

# Getting Start
```bash
Usage: rlex <config_file> <output_file>
```
the sample of sample_ouput.rs:
```rust
%{
    // your struct here
    pub struct Foo {
        x: i32,
        y: i32,
    }

    pub struct Bar {
        x: i32,
        y: i32,
    }
%}
    // your regex definitions here
    number = "[0-9]*"
    idenfitier = "[A-Za-z][A-Za-z0-9]*"
    error = "( |;|?|,|!|=)*"
    test = "\*|\\|\||\."
%%
    // your rules here
    {number} -> |s|{
        println!("number: {}", s);
    } ;;
    {idenfitier} -> |s|{
        println!("idenfitier: {}", s);
    } ;;
    {error} -> |s|{
        println!("error: {}", s);
    } ;;
    ({test})* -> |s|{
        println!("test: {}", s);
    } ;;
%%
    // your variable here
    pub a: i32,
    pub b: i64, 
```

```bash
rlex sample.rlex sample_ouput.rs                                         
Done.
Please add the following dependencies to your Cargo.toml:
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Then copy sample_ouput.rs to your own project, add the following dependencies to your Cargo.toml.
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
fn main() {
    let r = rlex_gen::rlex::Rlex { a: 1, b: 2 };
    r.lex("int abc_123 = 123; if (abc_123 == 123) { abc_123 = 456; }");
}
```
Run, then the output is:

```bash
idenfitier: int
error:  
idenfitier: abc
unknown error:_
number: 123
error:  = 
number: 123
error: ; 
idenfitier: if
error:  
unknown error:(
idenfitier: abc
unknown error:_
number: 123
error:  == 
number: 123
unknown error:)
error:  
unknown error:{
error:  
idenfitier: abc
unknown error:_
number: 123
error:  = 
number: 456
error: ; 
unknown error:}
```
