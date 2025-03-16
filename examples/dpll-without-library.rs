use std::env;
use std::error::Error;
use std::fs::File;

mod dpll_common;
use dpll_common::*;

fn solve_dfs(problem: &CnfSat, mut vars_left: &[u32], assignments: &mut [i8]) -> bool {
    let mut unknown_count = 0;

    let mut to_reset: Vec<usize> = vec![];
    let do_reset = |to_reset: &Vec<usize>, assignments: &mut [i8]| {
        for &var in to_reset {
            assignments[var] = 0;
        }
    };

    // Validate the current assignments and try to eagerly assign some variables.
    // Note that after eagerly assigning a variable, the following peculiar things may happen:
    //
    // 1. Another variable can now be eagerly assigned. If that follows from an earlier caluse,
    //    we might miss that. But it's fine, as it will be checked in the following call anyway.
    //
    // 2. Another clause (which also only depended on the same variable) may become false
    //    (invalidating the whole formula). If that clause was an earlier one, we miss that.
    //
    // We could overcome both of these issues by repeating this block of checks in case a variable
    // was set eagerly. However, as tested in practice, this will worsen the performance.
    //
    // 2 could also be fixed by not applying eager assignments directly (first collect them and
    // ensure there is no conflict, then apply), but I doubt there will be a notable improvement.
    for clause in &problem.clauses {
        match clause.eval(assignments) {
            ClauseState::Known(false) => {
                do_reset(&to_reset, assignments);
                return false;
            }
            ClauseState::Known(true) => {} // Too bad we can't remove this clause easily
            ClauseState::OneLeft(literal) => {
                // Assign a variable eagerly. May break other clauses - in the worst case,
                // checked on the following iteration.
                if literal < 0 {
                    assignments[-literal as usize] = -1;
                    to_reset.push(-literal as usize);
                } else {
                    assignments[literal as usize] = 1;
                    to_reset.push(literal as usize);
                }
            }
            ClauseState::Unknown => {
                unknown_count += 1;
            }
        }
    }

    // Even though not all variables are assigned, the expression is already true :hooray:
    if unknown_count == 0 {
        return true;
    }

    while let Some((cur_var, vars_rest)) = vars_left.split_first() {
        let idx = *cur_var as usize;
        if assignments[idx] != 0 {
            // Already assigned eagerly. Skip to the next variable.
            vars_left = vars_rest;
            continue;
        }

        assignments[idx] = 1; // Try `true`
        if solve_dfs(problem, vars_rest, assignments) {
            return true;
        }

        assignments[idx] = -1; // Try `false`
        if solve_dfs(problem, vars_rest, assignments) {
            return true;
        }

        assignments[idx] = 0; // Restore "unassigned"
        do_reset(&to_reset, assignments);
        return false;
    }

    // The initial validation did not detect that the formula is decided,
    // but we ran out of variables to check. This only happens if we've
    // managed to eagerly assign the last variable. So, just perform
    // the final validation!
    solve_dfs(problem, vars_left, assignments)
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
