#[warn(dead_code, non_snake_case, clippy::module_inception)]
pub struct BSTNode<K, T> {
    key: K,
    value: T,
    left: Option<Box<BST<K, T>>>,
    right: Option<Box<BST<K, T>>>,
}

pub struct BST<K, T> {
    pub root: Option<BSTNode<K, T>>,
}

impl<K: Ord, T> BST<K, T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn get(&self, key: &K) -> Option<&T> {
        Self::get_node(&self.root, key)
    }

    fn get_node<'a>(node: &Option<Box<BSTNode<K, T>>>, searched_key: &K) -> Option<&'a T> {
        match node {
            Some(n) => {
                if searched_key < &n.key {
                    Self::get_node(&n.left, searched_key)
                } else if searched_key > &n.key {
                    Self::get_node(&n.right, searched_key)
                } else {
                    Some(&n.value)
                }
            }
            None => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::BST;

    #[test]
    fn test_creation() {
        let bst: BST<u8, u8> = BST::new();
        assert_eq!(bst.root.is_none(), true);
    }
}
