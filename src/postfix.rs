pub struct PostfixExpr(pub Vec<u8>);

/// to_postfix
/// 将中缀表达式转换为后缀表达式
pub fn to_postfix(expr: &str) -> PostfixExpr {
    // 表达正则运算符的优先级
    // other表示其他所有的优先级
    let mut op_stack = Vec::new();
    let mut res_stack = Vec::new();

    expr.as_bytes().iter().for_each(|&curr| {
        match curr {
            b'(' => {
                // ( 无条件入栈
                op_stack.push(curr);
            }
            b')' => {
                // ） 出栈直到遇到 (
                while let Some(top) = op_stack.pop() {
                    if top == b'(' {
                        break;
                    }

                    res_stack.push(top);
                }
            }
            b'|' => {
                while let Some(top) = op_stack.last() {
                    if *top == b'(' {
                        break;
                    }

                    res_stack.push(op_stack.pop().unwrap());
                }

                op_stack.push(curr);
            }
            b'*' => {
                while let Some(top) = op_stack.last() {
                    if *top == b'(' || *top == b'|' {
                        break;
                    }

                    res_stack.push(op_stack.pop().unwrap());
                }

                op_stack.push(curr);
            }
            _ => {
                res_stack.push(curr);
            }
        }
    });

    // 将剩余的运算符出栈
    while let Some(top) = op_stack.pop() {
        res_stack.push(top);
    }

    return PostfixExpr(res_stack);
}
