use regex::Regex;
use std::error::Error;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Charactor {
    Char(u8),
    LeftBracket,
    RightBracket,
    Closure,
    Concat,
    Or,
}

pub struct RegexExpr(pub Vec<Charactor>);

impl RegexExpr {
    pub fn build(expr: &str) -> Result<RegexExpr, Box<dyn Error>> {
        let res: String = Self::to_simple_regex(expr)?;
        let res = Self::to_charactors(&res)?;
        let res = Self::to_explicit_concat_expr(&res);
        if let Ok(res) = Self::to_postfix(&res) {
            Ok(RegexExpr(res))
        } else {
            Err(format!("parsing RegexExpr error: invalid regex \"{}\"", expr).into())
        }
    }

    /// to_simple_regex
    /// 将复杂正则表达式转换为简单正则表达式
    fn to_simple_regex(expr: &str) -> Result<String, Box<dyn Error>> {
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
                    return Err(
                        format!("parsing RegexExpr error: invalid regex: {}-{}", from, to).into(),
                    );
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

                // 转义特殊字符
                if c == '\\' || c == '.' || c == '(' || c == ')' || c == '*' || c == '|' {
                    to_replace.push('\\');
                }

                to_replace.push(c);
            }
            // warp with ()
            to_replace = format!("({})", to_replace);

            // replace
            expr = expr.replace(&captures[0], &to_replace);
        }

        Ok(expr)
    }

    fn to_explicit_concat_expr(expr: &Vec<Charactor>) -> Vec<Charactor> {
        let mut res = Vec::new();
        res.push(Charactor::LeftBracket);

        expr.iter().enumerate().for_each(|(index, &curr)| {
            res.push(curr);

            if curr == Charactor::LeftBracket || curr == Charactor::Or {
                return;
            }

            // 还有下一个，则查看下一个
            if index + 1 < expr.len() {
                let next = expr[index + 1];

                if next == Charactor::RightBracket
                    || next == Charactor::Or
                    || next == Charactor::Closure
                {
                    return;
                }

                res.push(Charactor::Concat);
            }
        });

        res.push(Charactor::RightBracket);
        res
    }

    pub fn to_charactors(expr: &str) -> Result<Vec<Charactor>, Box<dyn Error>> {
        let mut res = Vec::new();
        let mut last = b' ';

        for &curr in expr.as_bytes().iter() {
            // 如果上一个字符是\，则表示当前字符是转义字符
            if last == b'\\' {
                last = b' ';
                match curr {
                    b'.' => {
                        res.push(Charactor::Char(b'.'));
                    }
                    b'(' => {
                        res.push(Charactor::Char(b'('));
                    }
                    b')' => {
                        res.push(Charactor::Char(b')'));
                    }
                    b'*' => {
                        res.push(Charactor::Char(b'*'));
                    }
                    b'|' => {
                        res.push(Charactor::Char(b'|'));
                    }
                    b'\\' => {
                        res.push(Charactor::Char(b'\\'));
                    }
                    _ => {
                        return Err(format!(
                            "parsing RegexExpr error: invalid regex escape: \\{}",
                            curr as char
                        )
                        .into());
                    }
                }
            } else {
                // 如果上一个字符不是\，则表示当前字符不是转义字符
                match curr {
                    b'(' => {
                        res.push(Charactor::LeftBracket);
                    }
                    b')' => {
                        res.push(Charactor::RightBracket);
                    }
                    b'*' => {
                        res.push(Charactor::Closure);
                    }
                    b'|' => {
                        res.push(Charactor::Or);
                    }
                    b'.' => {
                        res.push(Charactor::Concat);
                    }
                    b'\\' => last = curr,
                    _ => {
                        res.push(Charactor::Char(curr));
                    }
                }
            }
        }

        Ok(res)
    }

    /// to_postfix
    /// 将中缀表达式转换为后缀表达式
    pub fn to_postfix(expr: &Vec<Charactor>) -> Result<Vec<Charactor>, Box<dyn Error>> {
        // 表达正则运算符的优先级
        // other表示其他所有的优先级
        let mut op_stack = Vec::new();
        let mut res_stack = Vec::new();

        expr.iter().for_each(|&curr| {
            match curr {
                Charactor::LeftBracket => {
                    // ( 无条件入栈
                    op_stack.push(curr);
                }
                Charactor::RightBracket => {
                    // ） 出栈直到遇到 (
                    while let Some(top) = op_stack.pop() {
                        if top == Charactor::LeftBracket {
                            break;
                        }

                        res_stack.push(top);
                    }
                }
                Charactor::Closure | Charactor::Or | Charactor::Concat => {
                    let priority = |c: &Charactor| match c {
                        Charactor::Closure => 2,
                        Charactor::Or => 0,
                        Charactor::Concat => 1,
                        _ => 0,
                    };

                    // 弹栈直到栈顶元素为(或 优先级小于当前元素
                    while let Some(&top) = op_stack.last() {
                        if top == Charactor::LeftBracket || priority(&top) < priority(&curr) {
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

        // 如果栈中还有元素，则表示遇到错误
        if op_stack.len() != 0 {
            return Err("".into());
        }

        Ok(res_stack)
    }
}
