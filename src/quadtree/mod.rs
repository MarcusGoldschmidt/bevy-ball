#[derive(Debug)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Bounds {
    pub fn new(x: f64, y: f64, max_x: f64, max_y: f64) -> Self {
        Bounds { x, y, max_x, max_y }
    }

    pub fn new_simple(max_x: f64, max_y: f64) -> Self {
        Bounds {
            x: 0.,
            y: 0.,
            max_x,
            max_y,
        }
    }
}

#[derive(Debug)]
pub struct QuadTree<T> {
    deep: usize,
    tree: QuadTreeEnum<T>,
}

#[derive(Debug)]
enum QuadTreeEnum<T> {
    Leaf {
        bounds: Bounds,
        items: Vec<Box<T>>,
    },
    Node {
        bounds: Bounds,
        children: [Box<QuadTreeEnum<T>>; 4],
    },
}

impl<T> QuadTree<T> {
    pub fn new(bounds: Bounds, deep: Option<usize>) -> Self {
        let deep = deep.unwrap_or(4);

        QuadTree {
            deep,
            tree: Self::build_deep(bounds, deep),
        }
    }

    fn find_index(bounds: &Bounds, x: f64, y: f64) -> usize {
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

    fn build_node(bounds: Bounds, deep: usize) -> QuadTreeEnum<T> {
        let four_bounds = Self::find_quad_bounds(&bounds)
            .map(move |inner_bound| Box::new(Self::build_deep(inner_bound, deep - 1)));

        QuadTreeEnum::Node {
            bounds,
            children: four_bounds,
        }
    }

    fn build_deep(bounds: Bounds, deep: usize) -> QuadTreeEnum<T> {
        match deep {
            0 => QuadTreeEnum::Leaf {
                bounds,
                items: Vec::new(),
            },
            _ => Self::build_node(bounds, deep),
        }
    }

    pub fn insert(&mut self, x: f64, y: f64, item: T) {
        if let Some(items) = self.tree.find(x, y) {
            items.push(Box::new(item));
            return;
        }
    }
}

impl<T> QuadTreeEnum<T> {
    pub fn find(&mut self, x: f64, y: f64) -> Option<&mut Vec<Box<T>>> {
        match self {
            QuadTreeEnum::Leaf {
                bounds,
                ref mut items,
            } => {
                if x >= bounds.x && x < bounds.max_x && y >= bounds.y && y < bounds.max_y {
                    Some(items)
                } else {
                    None
                }
            }
            QuadTreeEnum::Node {
                bounds,
                ref mut children,
            } => {
                let index = QuadTree::<T>::find_index(bounds, x, y);
                children[index].find(x, y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_curve_test() {
        let mut tree = QuadTree::new(Bounds::new_simple(100., 100.), Some(4));

        tree.insert(10., 10., 1);

        println!("{:#?}", tree);
    }
}
