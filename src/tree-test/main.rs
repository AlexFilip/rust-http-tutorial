use std::ops::ControlFlow;

pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn traverse_inorder<B, C>(&self, initial: C, f: &mut impl FnMut(&T, &C) -> ControlFlow<B, C>) -> ControlFlow<B, C> {
        let val = if let Some(node) = self.left.as_ref() {
            node.traverse_inorder(initial, f)
        } else {
            ControlFlow::Continue(initial)
        }?;

        let val2 = f(&self.value, &val)?;

        let val3 = if let Some(node) = self.right.as_ref() {
            node.traverse_inorder(val2, f)
        } else {
            ControlFlow::Continue(val2)
        }?;

        ControlFlow::Continue(val3)
    }

    fn leaf(value: T) -> Option<Box<TreeNode<T>>> {
        Some(Box::new(Self { value, left: None, right: None }))
    }
}

fn tree_test() {
    let node = TreeNode {
        value: 0,
        left: TreeNode::leaf(1),
        right: Some(Box::new(TreeNode {
            value: -1,
            left: TreeNode::leaf(5),
            right: TreeNode::leaf(2),
        }))
    };

    let res = node.traverse_inorder(0, &mut |val, accumulator| {
        println!("{}", val);
        ControlFlow::<(), i32>::Continue(accumulator + val)
    });

    if let ControlFlow::Continue(v) = res {
        println!("Final result = {}", v);
    }

    // assert_eq!(res, ControlFlow::Break(-1));
    // assert_eq!(sum, 6);

}

fn main() {
    tree_test();
}
