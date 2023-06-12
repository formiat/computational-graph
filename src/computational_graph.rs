use std::cell::RefCell;
use std::rc::Rc;

pub type NodeCelled = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct NodeData {
    cache: RefCell<Option<f32>>,
    dependents: RefCell<Vec<NodeCelled>>,
}

impl NodeData {
    fn clear_cache(&self) {
        for dependent in self.dependents.borrow().iter() {
            dependent.borrow_mut().data_mut().clear_cache();
        }
        *self.cache.borrow_mut() = None;
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Input {
        x: RefCell<f32>,
        data: NodeData,
    },
    Binary {
        op: BinaryOp,
        a: NodeCelled,
        b: NodeCelled,
        data: NodeData,
    },
    Unary {
        op: UnaryOp,
        x: NodeCelled,
        data: NodeData,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Mul,
    Pow,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Sin,
}

impl Node {
    pub fn create_input(x: f32) -> NodeCelled {
        Rc::new(RefCell::new(Self::Input {
            x: RefCell::new(x),
            data: NodeData {
                cache: RefCell::new(Some(x)),
                dependents: RefCell::new(Vec::new()),
            },
        }))
    }

    pub fn create_add(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        Self::create_binary_node(BinaryOp::Add, a, b)
    }

    pub fn create_mul(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        Self::create_binary_node(BinaryOp::Mul, a, b)
    }

    pub fn create_sin(x: NodeCelled) -> NodeCelled {
        Self::create_unary_node(UnaryOp::Sin, x)
    }

    pub fn create_pow(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        Self::create_binary_node(BinaryOp::Pow, a, b)
    }

    fn create_binary_node(op: BinaryOp, a: NodeCelled, b: NodeCelled) -> NodeCelled {
        let res = Rc::new(RefCell::new(Self::Binary {
            op,
            a: a.clone(),
            b: b.clone(),
            data: NodeData {
                cache: RefCell::new(None),
                dependents: RefCell::new(Vec::new()),
            },
        }));

        a.borrow_mut().add_dependent(res.clone());
        b.borrow_mut().add_dependent(res.clone());

        res
    }

    fn create_unary_node(op: UnaryOp, x: NodeCelled) -> NodeCelled {
        let res = Rc::new(RefCell::new(Self::Unary {
            op,
            x: x.clone(),
            data: NodeData {
                cache: RefCell::new(None),
                dependents: RefCell::new(Vec::new()),
            },
        }));

        x.borrow_mut().add_dependent(res.clone());

        res
    }

    pub fn compute(&self) -> f32 {
        match self {
            Self::Input { x, .. } => *x.borrow(),
            Self::Binary { op, a, b, data } => {
                let cached = *data.cache.borrow();

                if let Some(cached) = cached {
                    cached
                } else {
                    let computed = match op {
                        BinaryOp::Add => a.borrow().compute() + b.borrow().compute(),
                        BinaryOp::Mul => a.borrow().compute() * b.borrow().compute(),
                        BinaryOp::Pow => a.borrow().compute().powf(b.borrow().compute()),
                    };
                    *data.cache.borrow_mut() = Some(computed);

                    computed
                }
            }
            Self::Unary { op, x, data } => {
                let cached = *data.cache.borrow();

                if let Some(cached) = cached {
                    cached
                } else {
                    let computed = match op {
                        UnaryOp::Sin => x.borrow().compute().sin(),
                    };
                    *data.cache.borrow_mut() = Some(computed);

                    computed
                }
            }
        }
    }

    pub fn set(&self, new_value: f32) {
        if let Self::Input { x, data } = self {
            *x.borrow_mut() = new_value;
            data.clear_cache();
        } else {
            panic!("Can only set to \"Input\"");
        }
    }

    fn add_dependent(&mut self, node: NodeCelled) {
        self.data().dependents.borrow_mut().push(node);
    }

    fn data(&self) -> &NodeData {
        match self {
            Self::Input { data, .. } | Self::Binary { data, .. } | Self::Unary { data, .. } => data,
        }
    }

    fn data_mut(&mut self) -> &mut NodeData {
        match self {
            Self::Input { data, .. } | Self::Binary { data, .. } | Self::Unary { data, .. } => data,
        }
    }
}
