use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Bounds {
    pub fn new(x: f32, y: f32, max_x: f32, max_y: f32) -> Self {
        Bounds { x, y, max_x, max_y }
    }

    pub fn new_simple(max_x: f32, max_y: f32) -> Self {
        Bounds {
            x: 0.,
            y: 0.,
            max_x,
            max_y,
        }
    }
}

#[derive(Resource, Debug)]
pub enum QuadTree<TKey: Eq + Hash + Clone, T: Clone> {
    Leaf {
        bounds: Bounds,
        items: HashMap<TKey, Arc<Mutex<T>>>,
    },
    Node {
        bounds: Bounds,
        children: [Box<QuadTree<TKey, T>>; 4],
    },
}

impl<TKey: Eq + Hash + Clone, T: Clone> QuadTree<TKey, T> {
    pub fn new(bounds: Bounds, deep: Option<usize>) -> Self {
        let deep = deep.unwrap_or(4);

        Self::build_deep(bounds, deep)
    }

    fn find_index(bounds: &Bounds, x: f32, y: f32) -> usize {
        let x_mid = (bounds.x + bounds.max_x) / 2.0;
        let y_mid = (bounds.y + bounds.max_y) / 2.0;
        let index = if x < x_mid {
            if y < y_mid {
                0
            } else {
                2
            }
        } else {
            if y < y_mid {
                1
            } else {
                3
            }
        };

        index
    }

    fn find_quad_bounds(bounds: &Bounds) -> [Bounds; 4] {
        let x_mid = (bounds.x + bounds.max_x) / 2.0;
        let y_mid = (bounds.y + bounds.max_y) / 2.0;
        [
            Bounds {
                x: bounds.x,
                y: bounds.y,
                max_x: x_mid,
                max_y: y_mid,
            },
            Bounds {
                x: x_mid,
                y: bounds.y,
                max_x: bounds.max_x,
                max_y: y_mid,
            },
            Bounds {
                x: bounds.x,
                y: y_mid,
                max_x: x_mid,
                max_y: bounds.max_y,
            },
            Bounds {
                x: x_mid,
                y: y_mid,
                max_x: bounds.max_x,
                max_y: bounds.max_y,
            },
        ]
    }

    fn build_node(bounds: Bounds, deep: usize) -> QuadTree<TKey, T> {
        let four_bounds = Self::find_quad_bounds(&bounds)
            .map(move |inner_bound| Box::new(Self::build_deep(inner_bound, deep - 1)));

        QuadTree::Node {
            bounds,
            children: four_bounds,
        }
    }

    fn build_deep(bounds: Bounds, deep: usize) -> QuadTree<TKey, T> {
        match deep {
            0 => QuadTree::Leaf {
                bounds,
                items: HashMap::new(),
            },
            _ => Self::build_node(bounds, deep),
        }
    }

    pub fn insert(&mut self, key: TKey, item: T, position: Vec2) {
        if let Some(mut items) = self.find(position) {
            items.insert(key, Arc::new(Mutex::new(item)));
            return;
        }
    }

    pub fn delete(&mut self, key: TKey, position: Vec2) {
        if let Some(mut items) = self.find(position) {
            items.remove(&key);
            return;
        }
    }
    pub fn find_id(&mut self, key: TKey, pos: Vec2) -> Option<&mut Arc<Mutex<T>>> {
        if let Some(mut items) = self.find(pos) {
            items.get_mut(&key)
        } else {
            None
        }
    }

    pub fn find(&mut self, pos: Vec2) -> Option<&mut HashMap<TKey, Arc<Mutex<T>>>> {
        let (x, y) = (pos.x, pos.y);

        match self {
            QuadTree::Leaf { bounds, items } => {
                if x >= bounds.x && x < bounds.max_x && y >= bounds.y && y < bounds.max_y {
                    Some(items)
                } else {
                    None
                }
            }
            QuadTree::Node { bounds, children } => {
                let index = QuadTree::<TKey, T>::find_index(&bounds, x, y);

                children[index].find(pos)
            }
        }
    }
    fn all_items(&self) -> HashMap<TKey, Arc<Mutex<T>>> {
        match self {
            QuadTree::Leaf { items, .. } => items.clone(),
            QuadTree::Node { children, .. } => {
                let mut items = HashMap::<TKey, Arc<Mutex<T>>>::new();
                for child in children.iter() {
                    for x in child.all_items() {
                        items.insert(x.0, x.1);
                    }
                }

                items
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_tree_test() {
        let mut tree = QuadTree::<u32, u32>::new(Bounds::new_simple(100., 100.), Some(4));

        tree.insert(1, 1, Vec2::new(10., 10.));

        println!("{:#?}", tree.all_items());
    }
}
