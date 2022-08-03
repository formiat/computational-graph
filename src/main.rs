use crate::computational_graph::Node;

mod computational_graph;

// round to decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

fn main() {
    // x1, x2, x3 are input nodes of the computational graph:
    let x1 = Node::create_input(1f32);
    let x2 = Node::create_input(2f32);
    let x3 = Node::create_input(3f32);
    let x4 = Node::create_input(3f32);

    // graph variable is the output node of the graph:
    let graph = Node::create_add(
        x1.clone(),
        Node::create_mul(
            x2.clone(),
            Node::create_sin(Node::create_add(
                x2.clone(),
                Node::create_pow(x3.clone(), x4.clone()),
            )),
        ),
    );

    let mut result = graph.borrow().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.32727);

    x1.borrow().set(2f32);
    x2.borrow().set(3f32);
    x3.borrow().set(4f32);
    x4.borrow().set(3f32);
    result = graph.borrow().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.56656);
}
