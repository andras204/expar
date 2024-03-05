#[derive(Debug)]
pub enum Error {
    MismatchedParens,
}

#[derive(Debug, Clone)]
pub struct Node {
    content: String,
    children: Vec<Node>,
}

impl Node {
    fn new(content: String) -> Self {
        Self { content, children: Vec::new() }
    }
}

pub fn build_paren_tree(expr: String) -> Result<Node, Error> {
    let mut expr = expr.clone();
    let mut stack = Vec::new();
    let mut free = Vec::new();
    let mut seq = 0;

    let mut x = 0;
    let mut len = expr.len();
    
    while x < len {
        match expr.chars().nth(x).unwrap() {
            '(' => {
                stack.push(x);
            },
            ')' => {
                let start = stack.pop().unwrap();
                let n = Node {
                    content: expr[start..=x].to_string(),
                    children: free.drain(..).collect()
                };
                free.push(n);
                expr.replace_range(start..=x, &format!("${}", seq));
                x = start;
                len = expr.len();
                seq += 1;
            }
            _ => (),
        }
        x += 1;
    }

    if stack.len() > 0 { return Err(Error::MismatchedParens); }

    Ok(free[0].clone())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }
