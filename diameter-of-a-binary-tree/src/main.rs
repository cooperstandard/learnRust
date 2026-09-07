use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diameter = 0;
    dfs(root, &mut diameter);
    diameter
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
    if let Some(node) = root {
        let left = dfs(node.borrow().left.clone(), diameter);
        let right = dfs(node.borrow().right.clone(), diameter);
        *diameter = (*diameter).max(left + right);
        return left.max(right) + 1;
    }
    0
}

fn from_level_order(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vals
        .into_iter()
        .map(|v| v.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
        .collect();
    let n = nodes.len();
    for i in 0..n {
        if let Some(node) = nodes[i].clone() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < n {
                node.borrow_mut().left = nodes[left].clone();
            }
            if right < n {
                node.borrow_mut().right = nodes[right].clone();
            }
        }
    }
    nodes.into_iter().next().flatten()
}

fn main() {
    let root = from_level_order(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
    let d = diameter_of_binary_tree(root);
    println!("Diameter: {d}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = from_level_order(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn example_2() {
        let root = from_level_order(vec![Some(1), Some(2)]);
        assert_eq!(diameter_of_binary_tree(root), 1);
    }
}

