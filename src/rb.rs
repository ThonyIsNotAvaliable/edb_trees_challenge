use std::cmp;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub struct RBNode<T> {
    pub value: T,
    pub color: Color,
    pub left: Option<Box<RBNode<T>>>,
    pub right: Option<Box<RBNode<T>>>,
}

pub struct RBTree<T> {
    root: Option<Box<RBNode<T>>>,
    pub rotations: usize,
}

impl Color {
    pub fn toggle(&mut self) {
        *self = match &self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }

    pub fn initial(&self) -> char {
        match &self {
            Color::Red => 'R',
            Color::Black => 'B',
        }
    }
}

impl<T> RBNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            color: Color::Red,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord> RBTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            rotations: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_rec(self.root.take(), value, &mut self.rotations);

        if let Some(ref mut raiz) = self.root {
            raiz.color = Color::Black
        }
    }

    pub fn remove(&mut self, value: T) {
        if !self.contains(&value) {
            return;
        }

        self.root = Self::remove_rec(self.root.take(), value, &mut self.rotations);
        if let Some(ref mut raiz) = self.root {
            raiz.color = Color::Black
        }
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

    pub fn validate_properties(&self) -> bool {
        if let Some(ref r) = self.root {
            if r.color == Color::Red {
                return false;
            }
        }
        Self::validate_properties_rec(&self.root).is_ok()
    }

    pub fn height(&self) -> usize {
        Self::height_rec(&self.root)
    }
    fn height_rec(node: &Option<Box<RBNode<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + std::cmp::max(Self::height_rec(&n.left), Self::height_rec(&n.right)),
        }
    }

    fn validate_properties_rec(node: &Option<Box<RBNode<T>>>) -> Result<usize, ()> {
        match node {
            None => Ok(0),
            Some(n) => {
                if Self::is_red(&n.right) {
                    return Err(());
                }
                if n.color == Color::Red && Self::is_red(&n.left) {
                    return Err(());
                }

                let bh_esq = Self::validate_properties_rec(&n.left)?;
                let bh_dir = Self::validate_properties_rec(&n.right)?;
                if bh_esq != bh_dir {
                    return Err(());
                }

                Ok(bh_esq + if n.color == Color::Black { 1 } else { 0 })
            }
        }
    }

    fn in_order_rec(node: &Option<Box<RBNode<T>>>, list: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(current) = node {
            Self::in_order_rec(&current.left, list);
            list.push(current.value.clone());
            Self::in_order_rec(&current.right, list);
        }
    }

    fn remove_rec(
        node: Option<Box<RBNode<T>>>,
        value: T,
        rots: &mut usize,
    ) -> Option<Box<RBNode<T>>> {
        let mut current = match node {
            Some(n) => n,
            None => return None,
        };

        if value < current.value {
            if !Self::is_red(&current.left) && !Self::is_son_red(&current.left) {
                current = Self::move_red_left(current, rots);
            }
            current.left = Self::remove_rec(current.left.take(), value, rots);
        } else {
            if Self::is_red(&current.left) {
                current = Self::rotate_right(current);
            }

            if value == current.value && current.right.is_none() {
                return None;
            }

            if !Self::is_red(&current.right) && !Self::is_son_red(&current.right) {
                current = Self::move_red_right(current, rots);
            }

            if value == current.value {
                let (new_right, min_value) = Self::remove_min(current.right.take().unwrap(), rots);
                current.value = min_value;
                current.right = new_right;
            } else {
                current.right = Self::remove_rec(current.right.take(), value, rots);
            }
        }

        Some(Self::rebalance(current, rots))
    }

    fn insert_rec(
        node: Option<Box<RBNode<T>>>,
        value: T,
        rots: &mut usize,
    ) -> Option<Box<RBNode<T>>> {
        let mut m = match node {
            Some(n) => n,
            None => return Some(Box::new(RBNode::new(value))),
        };

        match value.cmp(&m.value) {
            cmp::Ordering::Less => m.left = Self::insert_rec(m.left.take(), value, rots),
            cmp::Ordering::Greater => m.right = Self::insert_rec(m.right.take(), value, rots),
            cmp::Ordering::Equal => return Some(m),
        }

        Some(Self::rebalance(m, rots))
    }

    fn is_red(node: &Option<Box<RBNode<T>>>) -> bool {
        match node {
            None => false,
            Some(n) => n.color == Color::Red,
        }
    }

    fn is_son_red(node: &Option<Box<RBNode<T>>>) -> bool {
        match node {
            Some(n) => Self::is_red(&n.left),
            None => false,
        }
    }

    fn flip_colors(node: &mut Box<RBNode<T>>) {
        node.color.toggle();

        if let Some(ref mut n) = node.left {
            n.color.toggle();
        }
        if let Some(ref mut n) = node.right {
            n.color.toggle();
        }
    }

    fn rotate_left(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut x = node.right.take().unwrap();
        node.right = x.left.take();
        x.color = node.color;
        node.color = Color::Red;
        x.left = Some(node);

        x
    }

    fn rotate_right(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        let mut x = node.left.take().unwrap();
        node.left = x.right.take();
        x.color = node.color;
        node.color = Color::Red;
        x.right = Some(node);

        x
    }

    fn rebalance(mut current: Box<RBNode<T>>, rots: &mut usize) -> Box<RBNode<T>> {
        if !Self::is_red(&current.left) && Self::is_red(&current.right) {
            current = Self::rotate_left(current);
            *rots += 1;
        }

        if Self::is_red(&current.left) && Self::is_son_red(&current.left) {
            current = Self::rotate_right(current);
            *rots += 1
        }

        if Self::is_red(&current.left) && Self::is_red(&current.right) {
            Self::flip_colors(&mut current);
        }
        current
    }

    fn move_red_left(mut node: Box<RBNode<T>>, rots: &mut usize) -> Box<RBNode<T>> {
        Self::flip_colors(&mut node);

        if Self::is_son_red(&node.right) {
            *rots += 2;
            node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            node = Self::rotate_left(node);
            Self::flip_colors(&mut node);
        }

        node
    }

    fn move_red_right(mut node: Box<RBNode<T>>, rots: &mut usize) -> Box<RBNode<T>> {
        Self::flip_colors(&mut node);

        if Self::is_son_red(&node.left) {
            *rots += 1;
            node = Self::rotate_right(node);
            Self::flip_colors(&mut node);
        }

        node
    }

    fn remove_min(mut node: Box<RBNode<T>>, rots: &mut usize) -> (Option<Box<RBNode<T>>>, T) {
        if node.left.is_none() {
            return (None, node.value);
        }

        if !Self::is_red(&node.left) && !Self::is_son_red(&node.left) {
            node = Self::move_red_left(node, rots);
        }

        let (new_left, min_val) = Self::remove_min(node.left.take().unwrap(), rots);
        node.left = new_left;

        (Some(Self::rebalance(node, rots)), min_val)
    }
}

impl<T: Ord + std::fmt::Display> RBTree<T> {
    pub fn show_tree(&self) {
        println!("--------------------------");
        Self::print_rec(&self.root, 0);
        println!("--------------------------");
    }

    fn print_rec(node: &Option<Box<RBNode<T>>>, depth: usize) {
        if let Some(current) = node {
            Self::print_rec(&current.right, depth + 1);

            let indent = "   ".repeat(depth);

            println!(
                "{}|- {}({})",
                indent,
                current.value,
                current.color.initial()
            );

            Self::print_rec(&current.left, depth + 1);
        }
    }
}
