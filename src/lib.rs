use std::collections::binary_heap::BinaryHeap;

/// Represents the set of subproblems of an intermediate problem
/// or the value of the objective function of a feasible solution (leaf node).
pub enum SubproblemsOrScore<Node, Score: Ord> {
    /// Subproblems of an intermediate problem
    Subproblems(Box<dyn Iterator<Item = Node>>),
    /// The value of the objective function of a feasible solution
    Score(Score),
}
// TODO: Consider an alternative implementation by making the iterator
// type a generic variable rather than a `dyn`

use SubproblemsOrScore::{Score, Subproblems};

/// Represents a problem that is to be solved with branch-and-bound
pub trait Problem<Node, Score: Ord> {
    /// The initial problem state
    fn initial(&self) -> Node;

    /// Processes a node.
    ///
    /// If the given node is a fesible solution, returns the value of the
    /// objective function at it.
    ///
    /// If the given node is a subproblem that is to be split further
    /// into subproblems, returns the set of its subproblems.
    fn branch_or_evaluate(&self, node: &Node) -> SubproblemsOrScore<Node, Score>;

    fn bound(&self, node: &Node) -> Score;
}

struct Candidate<Node, Score: Ord> {
    node: Node,
    /// Score is always defined.
    /// For intermediate subproblems, it is the value of the bounding function.
    /// When a node is discovered to be a leaf node, its score is to be replaced
    /// with the value of the objective function.
    score: Score,
}

/// Wraps a `Candidate` and implements `{Partial,}Eq` and `{Partial,}Ord`
/// based on the score, ignoring the candidate.
struct CandidateAsScore<Node, Score: Ord>(Candidate<Node, Score>);

impl<Node, Score: Ord> PartialEq for CandidateAsScore<Node, Score> {
    fn eq(&self, other: &Self) -> bool {
        self.0.score == other.0.score
    }
}

impl<Node, Score: Ord> Eq for CandidateAsScore<Node, Score> {}

impl<Node, Score: Ord> PartialOrd for CandidateAsScore<Node, Score> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.score.cmp(&other.0.score))
    }
}

impl<Node, Score: Ord> Ord for CandidateAsScore<Node, Score> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve<Node, Score: Ord>(problem: &impl Problem<Node, Score>) -> Option<Node> {
    let mut ans: Option<Candidate<Node, Score>> = None;

    let mut queue = BinaryHeap::new();
    queue.push(CandidateAsScore(Candidate {
        node: problem.initial(),
        score: problem.bound(&problem.initial()),
    }));

    while let Some(CandidateAsScore(candidate)) = queue.pop() {
        if let Some(incumbent) = &ans {
            if candidate.score < incumbent.score {
                // When a candidate's _bound_ is worse than the incumbent's
                // objective score, we don't need to search any further.
                break;
                // TODO: we can only break as easily in the BeFS case
            }
        }

        match problem.branch_or_evaluate(&candidate.node) {
            // Intermediate subproblem
            Subproblems(subproblems) => {
                for node in subproblems {
                    let score = problem.bound(&node);
                    queue.push(CandidateAsScore(Candidate { node, score }));
                }
            }

            // Leaf node
            Score(objective_score) => {
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

    ans.and_then(|candidate| Some(candidate.node))
}
