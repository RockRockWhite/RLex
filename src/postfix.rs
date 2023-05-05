use std::collections::HashMap;

pub struct PostfixExpr(pub Vec<u8>);

pub fn to_explicit_concat_expr(expr: &str) -> String {
    let mut res = String::from("(");

    expr.as_bytes()
        .iter()
        .enumerate()
        .for_each(|(index, &curr)| {
            res.push(char::from(curr));

            if curr == b'(' || curr == b'|' {
                return;
            }

            // 还有下一个，则查看下一个
            if index + 1 < expr.len() {
                let next: u8 = expr.as_bytes()[index + 1];

                if next == b')' || next == b'|' || next == b'*' {
                    return;
                }

                res.push('.');
            }
        });

    res.push_str(")");

    res
}

/// to_postfix
/// 将中缀表达式转换为后缀表达式
pub fn to_postfix(expr: &str) -> PostfixExpr {
    let expr = to_explicit_concat_expr(expr);
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
            b'*' | b'|' | b'.' => {
                let priority = HashMap::from([(b'*', 2), (b'|', 0), (b'.', 1)]);

                // 弹栈直到栈顶元素为(或 优先级小于当前元素
                while let Some(&top) = op_stack.last() {
                    if top == b'(' || priority.get(&top).unwrap() < priority.get(&curr).unwrap() {
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

    // todo 处理错误

    return PostfixExpr(res_stack);
}
