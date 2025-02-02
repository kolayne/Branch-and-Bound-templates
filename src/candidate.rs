pub(crate) struct Candidate<Node, Score> {
    pub node: Node,
    /// Score is always defined.
    /// For intermediate subproblems, it is the value of the bounding function.
    /// When a node is discovered to be a leaf node, its score is to be replaced
    /// with the value of the objective function.
    pub score: Score,
}

/// Wraps a `Candidate` and implements `{Partial,}Eq` and `{Partial,}Ord`
/// based on the score, ignoring the candidate.
///
/// Used for Best-First-Search.
pub(crate) struct ScoreOrderedCandidate<Node, Score: Ord>(pub Candidate<Node, Score>);

impl<Node, Score: Ord> PartialEq for ScoreOrderedCandidate<Node, Score> {
    fn eq(&self, other: &Self) -> bool {
        self.0.score == other.0.score
    }
}

impl<Node, Score: Ord> Eq for ScoreOrderedCandidate<Node, Score> {}

impl<Node, Score: Ord> PartialOrd for ScoreOrderedCandidate<Node, Score> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.score.cmp(&other.0.score))
    }
}

impl<Node, Score: Ord> Ord for ScoreOrderedCandidate<Node, Score> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// We will also have `DepthOrderedCandidate` that we will use for DFS and BFS.
