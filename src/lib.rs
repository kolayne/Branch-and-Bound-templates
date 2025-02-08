mod candidate;

use self::candidate::{Candidate, ScoreOrderedCandidate};
use std::collections::binary_heap::BinaryHeap;

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
    type Score;

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

pub fn solve<Score: Ord, Node: Subproblem<Score = Score>>(initial: Node) -> Option<Node> {
    let mut ans: Option<Candidate<Node, Score>> = None;

    let mut queue = BinaryHeap::new();
    queue.push(ScoreOrderedCandidate(Candidate {
        score: initial.bound(),
        node: initial,
    }));

    while let Some(ScoreOrderedCandidate(candidate)) = queue.pop() {
        if let Some(incumbent) = &ans {
            if candidate.score < incumbent.score {
                // When a candidate's _bound_ is worse than the incumbent's
                // objective score, we don't need to search any further.
                break;
                // TODO: we can only break as easily in the BeFS case
            }
        }

        match candidate.node.branch_or_evaluate() {
            // Intermediate subproblem
            SubproblemResolution::Branched(subproblems) => {
                for node in subproblems {
                    let score = node.bound();
                    queue.push(ScoreOrderedCandidate(Candidate { node, score }));
                }
            }

            // Leaf node
            SubproblemResolution::Solved(objective_score) => {
                ans = match ans {
                    None => Some(candidate),
                    Some(incumbent) => {
                        if incumbent.score < objective_score {
                            // Replace the old (boundary) score with the objective score
                            Some(Candidate {
                                score: objective_score,
                                ..candidate
                            })
                        } else {
                            Some(incumbent)
                        }
                    }
                }
            }
        }
    }

    ans.map(|candidate| candidate.node)
}
