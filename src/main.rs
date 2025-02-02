use bb::SubproblemsOrScore::{Score, Subproblems};
use branch_and_bound as bb;

#[derive(Clone, Debug)]
struct GraphNode {
    bound: i32,
    next: Vec<GraphNode>,
}

struct Problem {
    root: GraphNode,
}

impl bb::Problem<GraphNode, i32> for Problem {
    fn initial(&self) -> GraphNode {
        self.root.clone()
    }

    fn branch_or_evaluate(
        &self,
        node: &GraphNode,
    ) -> branch_and_bound::SubproblemsOrScore<GraphNode, i32> {
        if node.bound < 5 {
            eprintln!("I should not be visited in Best-First-Search");
        } else {
            eprintln!("Node with bound {0} visited", node.bound)
        }
        if node.next.is_empty() {
            Score(node.bound)
        } else {
            Subproblems(Box::new(node.next.clone().into_iter()))
        }
    }

    fn bound(&self, node: &GraphNode) -> i32 {
        node.bound
    }
}

fn main() {
    let leaf0 = GraphNode {
        bound: 0,
        next: vec![],
    };
    let leaf1 = GraphNode {
        bound: 1,
        next: vec![],
    };
    let leaf2 = GraphNode {
        bound: 2,
        next: vec![],
    };
    let leaf3 = GraphNode {
        bound: 3,
        next: vec![],
    };
    let leaf4 = GraphNode {
        bound: 4,
        next: vec![],
    };
    let leaf5 = GraphNode {
        bound: 5,
        next: vec![],
    };

    let parent23 = GraphNode {
        bound: 4,
        next: vec![leaf2, leaf3],
    };
    let parent1p23 = GraphNode {
        bound: 5,
        next: vec![leaf1, parent23],
    };

    let parent45 = GraphNode {
        bound: 6,
        next: vec![leaf4, leaf5],
    };
    let parent0p45 = GraphNode {
        bound: 7,
        next: vec![leaf0, parent45],
    };

    let root = GraphNode {
        bound: 8,
        next: vec![parent1p23, parent0p45],
    };

    println!("Max node: {:#?}", bb::solve(&Problem { root }));
}
