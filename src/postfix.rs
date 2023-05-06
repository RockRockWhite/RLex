use regex::Regex;
use std::collections::HashMap;

pub struct PostfixExpr(pub Vec<u8>);

/// to_simple_regex
/// 将复杂正则表达式转换为简单正则表达式
pub fn to_simple_regex(expr: &str) -> String {
    let mut expr: String = expr.to_string();

    // 替换[A-z]为(a|b|c|...|z)
    for captures in Regex::new(r#"\[(.*?)\]"#)
        .unwrap()
        .captures_iter(&expr.clone())
    {
        let mut in_closure = captures[1].to_string();
        let mut to_replace = String::new();
        // 查找其中如A-z的字符，将其转换为(a|b|c|...|z)
        for a2b in Regex::new(r#"(.)-(.)"#).unwrap().captures_iter(&expr) {
            let mut a2b_replace = String::new();
            let mut from = a2b[1].chars().next().unwrap();
            let to = a2b[2].chars().next().unwrap();

            if to < from {
                panic!("Invalid regex: {}-{}", from, to);
            }

            while from <= to {
                a2b_replace.push(from);
                from = (from as u8 + 1) as char;
            }

            // 替换
            in_closure = in_closure.replace(&a2b[0], &a2b_replace);
        }

        // 为每个字符加上或
        for (index, c) in in_closure.chars().enumerate() {
            if index != 0 {
                to_replace.push('|');
            }

            to_replace.push(c);
        }
        // warp with ()
        to_replace = format!("({})", to_replace);

        // replace
        expr = expr.replace(&captures[0], &to_replace);
    }

    expr
}

pub fn to_explicit_concat_expr(expr: &str) -> String {
    let mut res: String = String::from("(");

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
    let expr = &to_simple_regex(expr);
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
