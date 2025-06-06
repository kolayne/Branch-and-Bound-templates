//! This library implements generic branch-and-bound and backtracking solver.
//!
//! Branch-and-bound (and backtracking, which is its special case) is the method
//! of solving an optimization problem by recursively breaking a problem down
//! to subproblems and then solving them. Unlike brute-force, branch-and-bound
//! will discard a subproblem if it discovers that the best potentially obtainable
//! solution to this subproblem is not better than the current best solution
//! (aka incumbent).
//!
//! To use the library, one shell implement a type that represents a problem
//! (subproblem) and implement the [`Subproblem`] trait for it.
//!
//! One can then [`solve`] an instance of problem using one of the predefined
//! methods (DFS, BFS, BeFS, etc) or use [`solve_with_container`], through
//! which custom strategies can be implemented.

pub mod bnb_aware_containers;

use bnb_aware_containers::BinaryHeapExt;
pub use bnb_aware_containers::BnbAwareContainer;

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

/// A problem (subproblem) to be solved with branch-and-bound
pub trait Subproblem {
    // Major TODO: Let `Subproblem` have a non-static lifetime. This will simplify
    // usage of the library a lot.
    //
    // Major major TODO: Allow `Subproblem` to return its children one by one,
    // rather than all at a time. This way, DFS could be implemented efficiently
    // by the users of the library.

    /// Return type of the boundary and the objective function.
    /// Higher score is better.
    type Score: Ord;

    /// Evaluates the subproblem space.
    ///
    /// If the space is to be broken further into subproblems, returns
    /// a sequence of subproblems (may be empty, which discards
    /// the current subspace).
    ///
    /// If the space consists of just one feasible solution to be solved
    /// directly, returns the score, which is the value of the objective
    /// function at the solution. The node is then considered a successful candidate.
    ///
    /// The method may mutate `self` as follows:
    /// - If `SubproblemResolution::Branched` is returned, the library shall
    ///   discard the object after that, so any changes to `self` are allowed, even
    ///   if after the changes it no longer represents the original subproblem;
    /// - If `SubproblemResolution::Solved` is returned, the library will use
    ///   the subproblem object as a successful candidate, so mutations to the internal
    ///   state are allowed, as long as `self` continues to represent the same
    ///   subproblem.
    fn branch_or_evaluate(&mut self) -> SubproblemResolution<Self, Self::Score>;

    /// Value of the boundary function at the subproblem space.
    ///
    /// The boundary function gives an upper-boundary of the best solution
    /// that could potentially be found in this subproblem space. The value of
    /// the boundary function must be greater than or equal to every value of
    /// the objective score of any subproblem reachable through consecutive
    /// `.branch_or_evaluate` calls.
    ///
    /// If at some point in the search process a subproblem's `.bound()` value
    /// is less than or equal to the current best solution, the subproblem is
    /// discarded (because no better solution will be found in its subtree).
    fn bound(&self) -> Self::Score;
}

/// Solve a problem with branch-and-bound / backtracking using a custom subproblem
/// container with a custom strategy.
///
/// Until the container is empty, a subproblem is popped from the container and evaluated;
/// when a subproblem is branched, the generated subnodes are put into the container to be
/// retrieved in following iterations.
///
/// A container is, thus, responsible for the order in which subproblems will be examined,
/// and can also implement additional features, such as early termination based on
/// the current best value, early termination based on the number of iterations,
/// eager or lazy evaluation, etc.
///
/// `solve_with_container` should be preferred for advanced use cases (e.g., custom order
/// or unusual early terination conditions). If you want one of the basic options,
/// use [`solve`].
pub fn solve_with_container<Node, Container>(mut container: Container) -> Option<Node>
where
    Node: Subproblem,
    Container: BnbAwareContainer<Node>,
{
    // Best candidate: its objective score and the node itself
    let mut best: Option<(Node::Score, Node)> = None;

    // `container` should initially contain the root node (or even several nodes)

    while let Some(mut candidate) = container.pop_with_incumbent(best.as_ref().map(|x| &x.0)) {
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

/// Order of traversing the subproblem tree with `solve`. See variants' docs for details.
pub enum TraverseMethod<Node> {
    /// Depth-first search (DFS): descends into every subtree until reaches the leaf node
    /// (or determines that a subtree is not worth descending into because the boundary
    /// value is not better than the incumbent's objective score).
    ///
    /// Nodes of the same layer will be processed in the order they are returned by the
    /// `Subproblem::branch_or_evaluate` method.
    ///
    /// For typical boundary functions, uses significantly less memory compared to greedy
    /// and breadth-first search.
    DepthFirst,

    /// Breadth-first search (BFS): Traverses the subproblem tree layer by layer.
    /// The processing order among nodes on the same layer is unspecified.
    ///
    /// For typical boundary functions, behaves similar to greedy search but uses
    /// a simpler internal data structure to store subproblems to be processed.
    BreadthFirst,

    /// Greedy search (also known as best-first search): traverses the tree in many
    /// directions simultaneously,
    /// on every iteration selects and evaluates the subproblem with the best value of
    /// the boundary function. All its children become candidates for the next selection
    /// (as long as their boundary value is better than the incumbent's objective score).
    ///
    /// The processing order among subproblems with the same boundary value is unspecified.
    ///
    /// For typical boundary functions, behaves similar to breadth-first search but selects
    /// subproblems more optimally.
    Greedy,

    /// Like greedy search but selects subproblems in the custom order, based on the
    /// given comparator `.cmp`.
    ///
    /// Processes subproblems in the order specified by `.cmp`: subproblems that compare
    /// *greater* are processed *first*! The processing order among subproblems that
    /// compare equal is unspecified.
    ///
    /// The processing order among nodes that compare equal according to `.cmp` is unspecified.
    ///
    /// Set `.cmp_superceeds_bound` to `true` only if `.cmp` guarantees that
    ///
    /// if `cmp(subproblem_a, subproblem_b) == Ordering::Less`
    ///
    /// then `subproblem_a.bound() < subproblem_b.bound()`
    ///
    /// (in other words, the order defined by `.cmp` is a specialized order / super-order
    /// with respect to the order defined by `Subproblem::bound`).
    ///
    /// If `.cmp_superceeds_bound` is set, the search will terminate as soon as the candidate
    /// that is best according to `.cmp` has the boundary value less (i.e., worse) than that of the
    /// current incumbent.
    Custom {
        cmp: Box<NodeCmp<Node>>,
        cmp_superceeds_bound: bool,
    },
}

/// Solve a problem with branch-and-bound / backtracking, using one of the default strategies.
///
/// Walks the subproblem tree (`initial` is the root) according to the method specified by `method`.
///
/// `solve` should be preferred for simple scenareous (i.e., a single initial node,
/// one of the default search strategy implementations). For more advanced use cases, use
/// [`solve_with_container`].
#[inline]
pub fn solve<Node: Subproblem>(initial: Node, method: TraverseMethod<Node>) -> Option<Node> {
    use TraverseMethod::*;

    match method {
        Greedy => {
            let pqueue = BinaryHeapExt {
                heap: binary_heap_plus::BinaryHeap::from_vec_cmp(
                    vec![initial],
                    |n1: &Node, n2: &Node| n1.bound().cmp(&n2.bound()),
                ),
                stop_early: true,
            };
            solve_with_container(pqueue)
        }

        Custom {
            cmp,
            cmp_superceeds_bound: stop_early,
        } => {
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
