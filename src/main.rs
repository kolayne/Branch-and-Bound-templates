use bb::SubproblemResolution;
use branch_and_bound as bb;

#[derive(Clone, Debug)]
struct GraphNode {
    bound: i32,
    next: Vec<GraphNode>,
}

impl bb::Subproblem for GraphNode {
    type Score = i32;

    fn branch_or_evaluate(&self) -> branch_and_bound::SubproblemResolution<GraphNode, i32> {
        if self.bound < 5 {
            eprintln!("I should not be visited in Best-First-Search");
        } else {
            eprintln!("Node with bound {0} visited", self.bound)
        }
        if self.next.is_empty() {
            SubproblemResolution::Solved(self.bound)
        } else {
            SubproblemResolution::Branched(Box::new(self.next.clone().into_iter()))
        }
    }

    fn bound(&self) -> i32 {
        self.bound
    }
}

fn graph() -> GraphNode {
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

    root
}

fn main() {
    println!("Trying breadth-first search");
    println!(
        "Max node: {:#?}",
        bb::solve(graph(), bb::SearchOrder::BreadthFirst)
    );

    println!("Now trying best-first search");
    println!(
        "Max node: {:#?}",
        bb::solve(graph(), bb::SearchOrder::BestFirst)
    );
}
