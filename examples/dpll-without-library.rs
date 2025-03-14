use std::env;
use std::error::Error;
use std::fs::File;

mod dpll_common;
use dpll_common::*;

// BUG: fails to find a solution for cnf-sat-samples/aim-50-1_6-yes1-4.cnf,
//                                   cnf-sat-samples/zebra_v155_c1135.cnf

// TODO: another algorithm to consider (maybe in the other example):
// this `solve_dfs` never removes clauses, even if they are known to be true.
// On the one hand, we save time on removing them, on the other hand, we lose
// time by checking them again and again for future variables.

fn solve_dfs(problem: &CnfSat, mut vars_left: &[u32], assignments: &mut [i8]) -> bool {
    let mut recheck = true;
    let mut unknown_count = 0;

    while recheck {
        recheck = false;
        unknown_count = 0;
        // Validate the current assignments and try to eagerly assign some variables
        for clause in &problem.clauses {
            match clause.eval(assignments) {
                ClauseState::Known(false) => return false,
                ClauseState::Known(true) => {}
                ClauseState::OneLeft(literal) => {
                    // Assign a variable eagerly. May happen multiple times for the same variable
                    assignments[literal.unsigned_abs() as usize] = if literal < 0 { -1 } else { 1 };
                    recheck = true;
                }
                ClauseState::Unknown => {
                    unknown_count += 1;
                }
            }
        }
    }

    // Even though not all variables are assigned, the expression is already true :hooray:
    if unknown_count == 0 {
        return true;
    }

    while let Some((cur_var, vars_rest)) = vars_left.split_first() {
        let idx = *cur_var as usize;
        if assignments[idx] == 0 {
            assignments[idx] = 1; // Try `true`
            if solve_dfs(problem, vars_rest, assignments) {
                return true;
            }

            assignments[idx] = -1; // Try `false`
            if solve_dfs(problem, vars_rest, assignments) {
                return true;
            }

            assignments[idx] = 0; // Restore "unassigned"
            return false;
        }

        // Already assigned eagerly. Skip to the following variable
        vars_left = vars_rest;
    }

    unreachable!(
        "According to the initial validation, the formula is not yet decided,
         but ran out of variables to check"
    );
}

fn solve(problem: &CnfSat) -> Option<Vec<i8>> {
    let mut assignments: Vec<i8> = vec![0; (problem.vars_cnt + 1) as usize];
    if solve_dfs(problem, &problem.vars_by_frequency, &mut assignments) {
        Some(assignments)
    } else {
        None
    }
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
