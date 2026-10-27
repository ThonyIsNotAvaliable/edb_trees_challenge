use std::cmp::{self, max};

pub struct NodeAVL<T> {
    pub value: T,
    pub height: isize,
    pub left: Option<Box<NodeAVL<T>>>,
    pub right: Option<Box<NodeAVL<T>>>,
}

pub struct AVLTree<T: Ord> {
    root: Option<Box<NodeAVL<T>>>,
    pub rotacoes: usize,
}

impl<T> NodeAVL<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            rotacoes: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_rec(self.root.take(), value, &mut self.rotacoes);
    }

    pub fn remove(&mut self, value: T) {
        self.root = Self::remove_rec(self.root.take(), value, &mut self.rotacoes);
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            match value.cmp(&node.value) {
                cmp::Ordering::Less => current = &node.left,
                cmp::Ordering::Greater => current = &node.right,
                cmp::Ordering::Equal => return true,
            }
        }
        false
    }

    pub fn in_order(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut list = Vec::new();
        Self::in_order_rec(&self.root, &mut list);

        list
    }

    pub fn in_order_rec(node: &Option<Box<NodeAVL<T>>>, list: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(current) = node {
            Self::in_order_rec(&current.left, list);
            list.push(current.value.clone());
            Self::in_order_rec(&current.right, list);
        }
    }

    pub fn height(&self) -> usize {
        Self::height_rec(&self.root)
    }

    pub fn validate_properties(&self) -> bool {
        Self::validate_properties_rec(&self.root)
    }

    fn validate_properties_rec(node: &Option<Box<NodeAVL<T>>>) -> bool {
        if let Some(n) = node {
            let hl = Self::height_rec(&n.left) as isize;
            let hr = Self::height_rec(&n.right) as isize;
            if (hl - hr).abs() > 1 {
                return false;
            }
            return Self::validate_properties_rec(&n.left)
                && Self::validate_properties_rec(&n.right);
        }
        true
    }

    fn height_rec(node: &Option<Box<NodeAVL<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + std::cmp::max(Self::height_rec(&n.left), Self::height_rec(&n.right)),
        }
    }

    fn insert_rec(
        node: Option<Box<NodeAVL<T>>>,
        value: T,
        rots: &mut usize,
    ) -> Option<Box<NodeAVL<T>>> {
        let mut current = match node {
            Some(n) => n,
            None => {
                return Some(Box::new(NodeAVL::new(value)));
            }
        };

        match value.cmp(&current.value) {
            cmp::Ordering::Less => {
                current.left = Self::insert_rec(current.left.take(), value, rots)
            }
            cmp::Ordering::Greater => {
                current.right = Self::insert_rec(current.right.take(), value, rots)
            }
            cmp::Ordering::Equal => return Some(current),
        };

        Self::rebalance(current, rots)
    }

    fn remove_rec(
        node: Option<Box<NodeAVL<T>>>,
        value: T,
        rots: &mut usize,
    ) -> Option<Box<NodeAVL<T>>> {
        let mut current = match node {
            Some(n) => n,
            None => return None,
        };

        //Busca

        match value.cmp(&current.value) {
            cmp::Ordering::Less => {
                current.left = Self::remove_rec(current.left.take(), value, rots)
            }
            cmp::Ordering::Greater => {
                current.right = Self::remove_rec(current.right.take(), value, rots)
            }
            cmp::Ordering::Equal => {
                if current.left.is_none() && current.right.is_none() {
                    return None;
                } else if current.left.is_some() && current.right.is_none() {
                    current = current.left.take().unwrap();
                } else if current.left.is_none() && current.right.is_some() {
                    current = current.right.take().unwrap();
                } else {
                    let (right_tree, succesor) =
                        Self::extract_min(current.right.take().unwrap(), rots);

                    current.value = succesor;

                    current.right = right_tree;
                }
            }
        };

        Self::rebalance(current, rots)
    }

    fn balancing_factor(node: &Box<NodeAVL<T>>) -> isize {
        Self::node_height(&node.left) - Self::node_height(&node.right)
    }

    fn node_height(node: &Option<Box<NodeAVL<T>>>) -> isize {
        match node {
            None => 0,
            Some(n) => n.height,
        }
    }

    fn rotate_right(mut y: Box<NodeAVL<T>>) -> Box<NodeAVL<T>> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();

        Self::update_height(&mut y);

        x.right = Some(y);

        Self::update_height(&mut x);
        x
    }

    fn rotate_left(mut x: Box<NodeAVL<T>>) -> Box<NodeAVL<T>> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();

        Self::update_height(&mut x);

        y.left = Some(x);
        Self::update_height(&mut y);
        y
    }

    fn update_height(node: &mut Box<NodeAVL<T>>) {
        node.height = 1 + max(
            Self::node_height(&node.left),
            Self::node_height(&node.right),
        )
    }

    fn rebalance(mut node: Box<NodeAVL<T>>, rots: &mut usize) -> Option<Box<NodeAVL<T>>> {
        Self::update_height(&mut node);
        let balance = Self::balancing_factor(&node);

        if balance > 1 {
            let left_balance = Self::balancing_factor(node.left.as_ref().unwrap());

            if left_balance >= 0 {
                *rots += 1;
                return Some(Self::rotate_right(node));
            } else {
                *rots += 2;
                let left_child = node.left.take().unwrap();
                node.left = Some(Self::rotate_left(left_child));
                return Some(Self::rotate_right(node));
            }
        }

        if balance < -1 {
            let right_balance = Self::balancing_factor(node.right.as_ref().unwrap());

            if right_balance <= 0 {
                *rots += 1;

                return Some(Self::rotate_left(node));
            } else {
                *rots += 2;

                let left_child = node.right.take().unwrap();
                node.right = Some(Self::rotate_right(left_child));
                return Some(Self::rotate_left(node));
            }
        }

        Some(node)
    }

    fn extract_min(mut node: Box<NodeAVL<T>>, rots: &mut usize) -> (Option<Box<NodeAVL<T>>>, T) {
        if node.left.is_some() {
            let (new_left, val) = Self::extract_min(node.left.take().unwrap(), rots);

            node.left = new_left;

            return (Self::rebalance(node, rots), val);
        }
        (node.right.take(), node.value)
    }
}

//houve ajuda de IA aqui
impl<T: Ord + std::fmt::Display> AVLTree<T> {
    pub fn show_tree(&self) {
        println!("--------------------------");
        Self::print_rec(&self.root, 0);
        println!("--------------------------");
    }

    fn print_rec(node: &Option<Box<NodeAVL<T>>>, depth: usize) {
        if let Some(n) = node {
            Self::print_rec(&n.right, depth + 1);

            let indent = "   ".repeat(depth);

            println!("{}|- {}", indent, n.value);

            Self::print_rec(&n.left, depth + 1);
        }
    }
}
