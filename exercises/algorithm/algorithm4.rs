/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            Some(ref mut root) => {
                // 如果树非空，调用 TreeNode 的 insert 方法插入值
                root.insert(value);
            }
            None => {
                // 如果树为空，直接创建根节点
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            Some(ref root) => {
                // 如果树非空，调用辅助函数 search_in_tree 进行递归查找
                search_in_tree(root, value)
            }
            None => false, // 如果树为空，直接返回 false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 如果值小于当前节点的值，插入到左子树
                if let Some(ref mut left) = self.left {
                    left.insert(value); // 左子树存在，递归插入
                } else {
                    self.left = Some(Box::new(TreeNode::new(value))); // 左子树不存在，创建新节点
                }
            }
            Ordering::Greater => {
                // 如果值大于当前节点的值，插入到右子树
                if let Some(ref mut right) = self.right {
                    right.insert(value); // 右子树存在，递归插入
                } else {
                    self.right = Some(Box::new(TreeNode::new(value))); // 右子树不存在，创建新节点
                }
            }
            Ordering::Equal => {
                // 如果值等于当前节点的值，不插入（避免重复）
            }
        }
    }
}

// 辅助函数：递归查找值是否存在于树中
fn search_in_tree<T>(node: &Box<TreeNode<T>>, value: T) -> bool
where
    T: Ord,
{
    match value.cmp(&node.value) {
        Ordering::Less => {
            // 如果值小于当前节点的值，递归查找左子树
            if let Some(ref left) = node.left {
                search_in_tree(left, value)
            } else {
                false // 左子树为空，未找到
            }
        }
        Ordering::Greater => {
            // 如果值大于当前节点的值，递归查找右子树
            if let Some(ref right) = node.right {
                search_in_tree(right, value)
            } else {
                false // 右子树为空，未找到
            }
        }
        Ordering::Equal => true, // 找到匹配的值
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
