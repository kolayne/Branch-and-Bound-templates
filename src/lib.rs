pub mod candidate;

use self::candidate::OrderedCandidate;
use std::collections::binary_heap::BinaryHeap;

/*
 * There are three similar concepts: Node, Subproblem, and Candidate.
 *
 * `Node` is a type defined by user that must implement `Subproblem`.
 *
 * `Subproblem` (trait) acts as a search space: a `Subproblem` can
 * either break down further into subproblems, or indicate a solution
 * (the value of the objective function). We can also estimate an upper
 * boundary of the solution in the search space.
 *
 * `OrderedCandidate` (trait) wraps a `Node` to add order (a particular order
 * depends on the implementation of the trait).
 */

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
    type Score: Ord;

    /// Evaluates a problem space.
    ///
    /// If the space is to be broken fruther into subproblems, returns
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

/// Solves the optimization problem specified by the tree that grows
/// from `initial`.
///
/// Uses the generic type `Candidate` to determine the order in which
/// to visit candidates in the tree.
pub fn ordered_solve<Node, Candidate>(initial: Node) -> Option<Node>
where
    Node: Subproblem + 'static,
    Candidate: OrderedCandidate<Node = Node>,
{
    // Best candidate: its objective score and the node itself
    let mut best: Option<(Candidate::Score, Node)> = None;

    let mut queue = BinaryHeap::new();
    queue.push(Candidate::new(initial));

    while let Some(candidate) = queue.pop() {
        // TODO: how to implement early break for a polymorphic `Candidate`?
        /*
        if let Some((score, _incumbent)) = &best {
            if &candidate.bound() < score {
                // When a candidate's _bound_ is worse than the incumbent's
                // objective score, we don't need to search any further.
                break;
                // TODO: we can only break as easily in the BeFS case
            }
        }
        */

        match candidate.branch_or_evaluate() {
            // Intermediate subproblem
            SubproblemResolution::Branched(subproblems) => {
                for node in subproblems {
                    queue.push(node);
                }
            }

            // Leaf node
            SubproblemResolution::Solved(candidate_score) => {
                best = match best {
                    None => Some((candidate_score, candidate.into_node())),
                    Some((incumbent_score, incumbent)) => {
                        if incumbent_score < candidate_score {
                            // Replace the old (boundary) score with the objective score
                            Some((candidate_score, candidate.into_node()))
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

pub enum SearchOrder {
    BestFirst,
    // TODO: implement depth-first-search
    //DepthFirst,
    BreadthFirst,
}

#[inline]
pub fn solve<Node: Subproblem + 'static>(initial: Node, order: SearchOrder) -> Option<Node> {
    use candidate::{BoundOrderedCandidate, DepthOrderedCandidate};
    use SearchOrder::*;

    match order {
        BestFirst => ordered_solve::<_, BoundOrderedCandidate<_, _>>(initial),
        BreadthFirst => ordered_solve::<_, DepthOrderedCandidate<_, _>>(initial),
        /*
        DepthFirstSearch => ordered_solve::<_, DepthOrderedCandidate<std::cmp::Reverse<Node>, _>>(
            std::cmp::Reverse(initial),
        )
        .map(|rev| rev.0),
        */
    }
}
