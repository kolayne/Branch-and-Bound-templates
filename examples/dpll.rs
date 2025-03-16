use std::{env, error::Error, fs::File, rc::Rc};

use branch_and_bound::{Subproblem, SubproblemResolution};

mod dpll_common;
use dpll_common::*;

pub struct Node {
    clauses: Rc<Vec<Clause>>,
    vars_left: Rc<Vec<u32>>,
    vars_left_idx: usize,
    assignments: Vec<i8>,
}

impl Subproblem for Node {
    // We never need to compare solutions to each other. We just need to find any.
    // Thus, `Score` is the unit type.
    type Score = ();

    fn bound(&self) -> Self::Score {}

    fn branch_or_evaluate(&mut self) -> SubproblemResolution<Self, Self::Score> {
        let mut unknown_count = 0;
        // Eager assignments
        for clause in self.clauses.as_ref() {
            match clause.eval(&self.assignments) {
                ClauseState::Known(false) => {
                    return SubproblemResolution::Branched(Box::new(std::iter::empty()));
                }
                ClauseState::Known(true) => {}
                ClauseState::Unknown => unknown_count += 1,
                ClauseState::OneLeft(literal) => {
                    // Assign a variable eagerly. May break other clauses - in the worst case,
                    // checked when processing children.
                    assert_ne!(literal, 0);
                    self.assignments[literal.unsigned_abs() as usize] = literal.signum() as i8;
                }
            }
        }

        if unknown_count == 0 {
            return SubproblemResolution::Solved(());
        }

        let vars_left = self.vars_left.as_ref();
        let mut var_idx = self.vars_left_idx;
        while var_idx < vars_left.len() {
            let &var = &vars_left[var_idx];

            if self.assignments[var as usize] != 0 {
                // Already eagerly assigned. Skip to the next variable
                var_idx += 1;
                continue;
            }

            let mut child_true = Node {
                clauses: self.clauses.clone(),
                vars_left: self.vars_left.clone(),
                vars_left_idx: var_idx + 1,
                assignments: self.assignments.clone(),
            };
            child_true.assignments[var as usize] = 1;

            let mut child_false = Node {
                clauses: self.clauses.clone(),
                vars_left: self.vars_left.clone(),
                vars_left_idx: var_idx + 1,
                assignments: self.assignments.clone(),
            };
            child_false.assignments[var as usize] = -1;

            return SubproblemResolution::Branched(Box::new([child_true, child_false].into_iter()));
        }

        // The initial validation did not detect that the formula is decided,
        // but we ran out of variables to check. This only happens if we've
        // managed to eagerly assign the last variable. So, just perform the
        // final validation!
        Node {
            clauses: self.clauses.clone(),
            vars_left: self.vars_left.clone(),
            vars_left_idx: var_idx,
            assignments: self.assignments.clone(),
        }
        .branch_or_evaluate()
    }
}

fn solve(parsed: &CnfSat) -> Option<Vec<i8>> {
    let problem = Node {
        clauses: Rc::new(parsed.clauses.clone()),
        vars_left: Rc::new(parsed.vars_by_frequency.clone()),
        vars_left_idx: 0,
        assignments: vec![0; 1 + parsed.vars_cnt as usize],
    };

    branch_and_bound::solve(problem, branch_and_bound::TraverseMethod::DepthFirst)
        .map(|n| n.assignments)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::from(
            "Expected filename as the (only) command-line argument",
        ));
    }

    let f = File::open(&args[1])?;
    let problem = parse_cnf(f)?;

    match solve(&problem) {
        None => println!("No solution"),
        Some(assignments) => {
            println!("Found solution!\n{:#?}", &assignments[1..]);
            assert_solves(&problem, &assignments);
        }
    }

    Ok(())
}
