use std::fmt::Display;

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
    pub fn new(content: String) -> Self {
        Self {
            content,
            children: Vec::new(),
        }
    }

    fn display_map(&self, depth: u16) -> Vec<(u16, String)> {
        let mut vec = Vec::with_capacity(self.children.len() + 1);
        for c in self.children.iter().rev() {
            vec.append(&mut c.display_map(depth + 1))
        }
        vec.push((depth, self.content.to_owned()));
        vec
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut depth_map = self.display_map(0);
        depth_map.reverse();
        for d in depth_map {
            let mut indent = String::new();
            for _ in 0..d.0 {
                indent.push_str("    ");
            }
            writeln!(f, "{}└── {}", indent, d.1)?;
        }
        Ok(())
    }
}

pub fn build_paren_tree(expr: &str) -> Result<Node, Error> {
    let mut expr = prepare_expr(expr);
    let mut stack = Vec::new();
    let mut free: Vec<(Node, usize)> = Vec::new();

    let mut x = 0;
    let mut len = expr.len();

    while x < len {
        match expr.chars().nth(x).unwrap() {
            '(' => {
                stack.push(x);
            }
            ')' => {
                let start = stack.pop().unwrap();
                if x - start == 1 {
                    expr.replace_range(start..=x, "");
                    x = start;
                    len = expr.len();
                    continue;
                }
                let mut children = Vec::new();
                for y in (0..free.len()).rev() {
                    if free[y].1 > stack.len() {
                        children.push(free.remove(y).0);
                    }
                }
                children.reverse();
                let n = Node {
                    content: expr[start..=x].to_string(),
                    children,
                };
                free.push((n, stack.len()));
                expr.replace_range(start..=x, "$");
                x = start;
                len = expr.len();
            }
            _ => (),
        }
        x += 1;
    }

    if stack.len() > 0 {
        return Err(Error::MismatchedParens);
    }

    // remove redundant nodes
    fn flatten(mut node: Node) -> Node {
        node.children = node.children.into_iter().map(|n| flatten(n)).collect();
        if node.content == "($)" {
            return node.children[0].clone();
        }
        node
    }

    fn prepare_expr(expr: &str) -> String {
        let mut expr = expr.replace(" ", "");
        expr.insert(0, '(');
        expr.push(')');
        expr
    }

    Ok(flatten(free[0].0.clone()))
}

pub fn build_op_tree(root: Node) -> Node {
    unimplemented!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }
