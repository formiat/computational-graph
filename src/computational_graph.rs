use std::cell::RefCell;
use std::rc::Rc;

pub type NodeCelled = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub enum Node {
    Input {
        x: RefCell<f32>,
        dependents: RefCell<Vec<NodeCelled>>,
    },
    Add {
        a: NodeCelled,
        b: NodeCelled,
        cache: RefCell<Option<f32>>,
        dependents: RefCell<Vec<NodeCelled>>,
    },
    Mul {
        a: NodeCelled,
        b: NodeCelled,
        cache: RefCell<Option<f32>>,
        dependents: RefCell<Vec<NodeCelled>>,
    },
    Sin {
        x: NodeCelled,
        cache: RefCell<Option<f32>>,
        dependents: RefCell<Vec<NodeCelled>>,
    },
    Pow {
        a: NodeCelled,
        b: NodeCelled,
        cache: RefCell<Option<f32>>,
        dependents: RefCell<Vec<NodeCelled>>,
    },
}

impl Node {
    pub fn create_input(x: f32) -> NodeCelled {
        let res = Self::Input {
            x: RefCell::new(x),
            dependents: RefCell::new(Vec::new()),
        };

        Rc::new(RefCell::new(res))
    }

    pub fn create_add(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        let res = Self::Add {
            a: a.clone(),
            b: b.clone(),
            cache: RefCell::new(None),
            dependents: RefCell::new(Vec::new()),
        };
        let res = Rc::new(RefCell::new(res));

        a.borrow_mut().add_dependent(res.clone());
        b.borrow_mut().add_dependent(res.clone());

        res
    }

    pub fn create_mul(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        let res = Self::Mul {
            a: a.clone(),
            b: b.clone(),
            cache: RefCell::new(None),
            dependents: RefCell::new(Vec::new()),
        };
        let res = Rc::new(RefCell::new(res));

        a.borrow_mut().add_dependent(res.clone());
        b.borrow_mut().add_dependent(res.clone());

        res
    }

    pub fn create_sin(x: NodeCelled) -> NodeCelled {
        let res = Self::Sin {
            x: x.clone(),
            cache: RefCell::new(None),
            dependents: RefCell::new(Vec::new()),
        };
        let res = Rc::new(RefCell::new(res));

        x.borrow_mut().add_dependent(res.clone());

        res
    }

    pub fn create_pow(a: NodeCelled, b: NodeCelled) -> NodeCelled {
        let res = Self::Pow {
            a: a.clone(),
            b: b.clone(),
            cache: RefCell::new(None),
            dependents: RefCell::new(Vec::new()),
        };
        let res = Rc::new(RefCell::new(res));

        a.borrow_mut().add_dependent(res.clone());
        b.borrow_mut().add_dependent(res.clone());

        res
    }

    pub fn compute(&self) -> f32 {
        self.get_cached_value().unwrap_or_else(|| {
            let new_value = match self {
                Node::Input { x, .. } => *x.borrow(),
                Node::Add { a, b, .. } => Self::add(&*a.borrow(), &*b.borrow()),
                Node::Mul { a, b, .. } => Self::mul(&*a.borrow(), &*b.borrow()),
                Node::Sin { x, .. } => Self::sin(&*x.borrow()),
                Node::Pow { a, b, .. } => Self::pow(&*a.borrow(), &*b.borrow()),
            };

            // To disable cache comment this line
            self.save_to_cache(new_value);

            new_value
        })
    }

    pub fn set(&self, new_value: f32) {
        if let Self::Input { x, .. } = self {
            *x.borrow_mut() = new_value;

            // To break cache comment this line
            self.clear_cache();
        } else {
            panic!("Can only set to \"Input\"");
        }
    }

    fn add_dependent(&self, node: NodeCelled) {
        let dependents = &mut *self.get_dependents().borrow_mut();

        // To break cache comment this line
        dependents.push(node);
    }

    fn clear_cache(&self) {
        match self {
            Node::Input { .. } => {}
            Node::Add { cache, .. }
            | Node::Mul { cache, .. }
            | Node::Sin { cache, .. }
            | Node::Pow { cache, .. } => *cache.borrow_mut() = None,
        }

        for dependent in &*self.get_dependents().borrow() {
            dependent.borrow().clear_cache();
        }
    }

    fn get_dependents(&self) -> &RefCell<Vec<NodeCelled>> {
        match self {
            Node::Input { dependents, .. }
            | Node::Add { dependents, .. }
            | Node::Mul { dependents, .. }
            | Node::Sin { dependents, .. }
            | Node::Pow { dependents, .. } => dependents,
        }
    }

    fn get_cached_value(&self) -> Option<f32> {
        match self {
            Node::Input { x, .. } => Some(*x.borrow()),
            Node::Add { cache, .. }
            | Node::Mul { cache, .. }
            | Node::Sin { cache, .. }
            | Node::Pow { cache, .. } => *cache.borrow(),
        }
    }

    fn save_to_cache(&self, new_value: f32) {
        match self {
            Node::Input { .. } => {}
            Node::Add { cache, .. }
            | Node::Mul { cache, .. }
            | Node::Sin { cache, .. }
            | Node::Pow { cache, .. } => *cache.borrow_mut() = Some(new_value),
        }
    }

    fn add(a: &Self, b: &Self) -> f32 {
        a.compute() + b.compute()
    }

    fn mul(a: &Self, b: &Self) -> f32 {
        a.compute() * b.compute()
    }

    fn sin(a: &Self) -> f32 {
        a.compute().sin()
    }

    fn pow(a: &Self, b: &Self) -> f32 {
        a.compute().powf(b.compute())
    }
}
