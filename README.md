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
12--ε-->10
13--ε-->9
12--ε-->13
11--b-->12
10--ε-->11
10--ε-->13
6--ε-->10
5--ε-->6
4--b-->5
3--ε-->4
2--a-->3
1--ε-->2
0--c-->1
```


```regex
c(a|bbcb*)*(ab)
```

上述样例结果如下

```mermaid
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

