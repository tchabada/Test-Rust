use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
struct AvlNode<T: Ord> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
}

type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<T: Ord> AvlTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }
}

impl<T: Ord> AvlTreeSet<T> {
    fn insert(&mut self, value: T) -> bool {
        let mut current_tree = &mut self.root;

        // 1. Starting from the root node or with a current node
        while let Some(current_node) = current_tree {
            // 2. Move to the left node if the value is less than the current node,
            //    right if greater, and stop if equal
            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        // 3. Do this until you an empty node and insert the value
        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
        }));

        true
    }
}

fn main() {
    let tree = Some(Box::new(AvlNode {
        value: 2,
        left: Some(Box::new(AvlNode {
            value: 1,
            left: None,
            right: None,
        })),
        right: Some(Box::new(AvlNode {
            value: 5,
            left: Some(Box::new(AvlNode {
                value: 3,
                left: None,
                right: Some(Box::new(AvlNode {
                    value: 4,
                    left: None,
                    right: None,
                })),
            })),
            right: None,
        })),
    }));

    println!("{:?}", &tree);

    let mut set = AvlTreeSet::new();

    assert!(set.insert(1)); // Insert new value
    assert!(!set.insert(1)); // Should not insert existing value

    assert!(set.insert(2)); // Insert another new value
    assert_eq!(
        // Checking the tree structure
        set.root,
        Some(Box::new(AvlNode {
            value: 1,
            left: None,
            right: Some(Box::new(AvlNode {
                value: 2,
                left: None,
                right: None
            })),
        }))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
}
