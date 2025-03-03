pub mod bnb_aware_containers;

use bnb_aware_containers::{BinaryHeapExt, BnbAwareContainer};

/// Represents the set of subproblems of an intermediate problem
/// or the value of the objective function of a feasible solution (leaf node).
pub enum SubproblemResolution<Node: ?Sized, Score> {
    /// Subproblems of an intermediate problem
    Branched(Box<dyn Iterator<Item = Node>>),
    /// The value of the objective function of a feasible solution
    Solved(Score),
}
// TODO: Consider an alternative implementation by making the iterator
// type a generic variable rather than a `dyn`

/// Represents a problem (subproblem) to be solved with branch-and-bound
pub trait Subproblem {
    // Higher score is better.
    type Score: Ord;

    /// Evaluates a problem space.
    ///
    /// If the space is to be broken further into subproblems, returns
    /// a sequence of subproblems (may be empty, which discards
    /// the current subspace).
    ///
    /// If the space consists of just one feasible solution to be solved
    /// directly, returns the score, which is the value of the objective
    /// function at the solution.
    fn branch_or_evaluate(&self) -> SubproblemResolution<Self, Self::Score>;

    /// Value of the boundary function at the problem space.
    fn bound(&self) -> Self::Score;
}

/// Solve the optimization problem initially specified by subproblem(s) in the `container`.
///
/// Until the container is empty, every subproblem in the container is evaluated; when
/// a subproblem is branched, the generated subnodes are put into the container to be
/// retrieved later.
///
/// A container is, thus, responsible for the order in which subproblems will be examined,
/// and can also implement additional features, such as early termination based on
/// the current best value, early termination based on the number of iterations,
/// eager or lazy evaluation, etc.
///
/// `solve_with_container` should be preferred for advanced use cases (e.g., custom order
/// or unusual early terination conditions). If you want one of the basic options,
/// use `solve`.
fn solve_with_container<Node, Container>(mut container: Container) -> Option<Node>
where
    Node: Subproblem,
    Container: BnbAwareContainer<Node>,
{
    // Best candidate: its objective score and the node itself
    let mut best: Option<(Node::Score, Node)> = None;

    // `container` should initially contain the root node (or even several nodes)

    while let Some(candidate) = container.pop_with_incumbent(best.as_ref().map(|x| &x.0)) {
        match candidate.branch_or_evaluate() {
            // Intermediate subproblem
            SubproblemResolution::Branched(subproblems) => {
                for node in subproblems {
                    container.push_with_incumbent(node, best.as_ref().map(|x| &x.0));
                }
            }

            // Leaf node
            SubproblemResolution::Solved(candidate_score) => {
                best = match best {
                    None => Some((candidate_score, candidate)),
                    Some((incumbent_score, incumbent)) => {
                        if incumbent_score < candidate_score {
                            // Replace the old (boundary) score with the objective score
                            Some((candidate_score, candidate))
                        } else {
                            Some((incumbent_score, incumbent))
                        }
                    }
                }
            }
        }
    }

    best.map(|(_, incumbent)| incumbent)
}

type NodeCmp<Node> = dyn Fn(&Node, &Node) -> std::cmp::Ordering;

// TODO: document `SearchOrder` variants in detail.
pub enum SearchOrder<Node> {
    BestFirst,
    DepthFirst,
    BreadthFirst,
    Custom {
        cmp: Box<NodeCmp<Node>>,
        stop_early: bool,
    }, // *larger* elements are visited *first*
}

/// Solve the optimization problem specified by `initial`.
///
/// Walks the subproblem tree in the order specified by `order`, which imply default containers
/// and default walk strategies (see the docs on `SearchOrder` variants for details).
///
/// `solve` should be preferred for simple scenareous (i.e., a single initial node,
/// one of the default search strategy implementations). For more advanced use cases, use
/// `solve_with_container`.
#[inline]
pub fn solve<Node: Subproblem>(initial: Node, order: SearchOrder<Node>) -> Option<Node> {
    use SearchOrder::*;

    match order {
        BestFirst => {
            let pqueue = BinaryHeapExt {
                heap: binary_heap_plus::BinaryHeap::from_vec_cmp(
                    vec![initial],
                    |n1: &Node, n2: &Node| n1.bound().cmp(&n2.bound()),
                ),
                stop_early: true,
            };
            solve_with_container(pqueue)
        }

        Custom { cmp, stop_early } => {
            let pqueue = BinaryHeapExt {
                heap: binary_heap_plus::BinaryHeap::from_vec_cmp(vec![initial], cmp),
                stop_early,
            };
            solve_with_container(pqueue)
        }

        BreadthFirst => {
            let queue = std::collections::VecDeque::from_iter([initial]);
            solve_with_container(queue)
        }

        DepthFirst => {
            let stack = vec![initial];
            solve_with_container(stack)
        }
    }
}
