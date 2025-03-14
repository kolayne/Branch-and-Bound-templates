use std::env;
use std::error::Error;
use std::fs::File;

mod dpll_common;
use dpll_common::*;

// TODO: another algorithm to consider (maybe in the other example):
// this `solve_dfs` never removes clauses, even if they are known to be true.
// On the one hand, we save time on removing them, on the other hand, we lose
// time by checking them again and again for future variables.

fn solve_dfs(problem: &CnfSat, mut vars_left: &[u32], assignments: &mut [i8]) -> bool {
    let mut recheck = true;
    let mut unknown_count = 0;

    let mut to_reset: Vec<usize> = vec![];
    let do_reset = |to_reset: &Vec<usize>, assignments: &mut [i8]| {
        for &var in to_reset {
            assignments[var] = 0;
        }
    };

    // It may be more efficient to only run this check once rather than in a loop.
    // Although sometimes we may miss an eager variable assignment that could become
    // possible after another eager variable assignment, we save time on repeated checks.
    // TODO: check.
    while recheck {
        recheck = false;
        unknown_count = 0;
        // Validate the current assignments and try to eagerly assign some variables
        for clause in &problem.clauses {
            match clause.eval(assignments) {
                ClauseState::Known(false) => {
                    do_reset(&to_reset, assignments);
                    return false;
                }
                ClauseState::Known(true) => {}
                ClauseState::OneLeft(literal) => {
                    // Assign a variable eagerly. May happen multiple times for the same variable
                    if literal < 0 {
                        assignments[-literal as usize] = -1;
                        to_reset.push(-literal as usize);
                    } else {
                        assignments[literal as usize] = 1;
                        to_reset.push(literal as usize);
                    }
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

#[cfg(test)]
fn open_sample_problem(name: &str) -> Result<CnfSat, Box<dyn Error>> {
    let path = format!("examples/cnf-sat-samples/{name}.cnf");
    Ok(parse_cnf(File::open(path)?)?)
}

#[test]
fn test_simple_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("simple")?;
    assert_solves(&problem, &solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_quinn_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("quinn")?;
    assert_solves(&problem, &solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_hole6_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("hole6")?;
    assert!(solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_par8_1_c_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("par8-1-c")?;
    assert_solves(&problem, &solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_dubois20_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois20")?;
    assert!(solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_dubois21_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois21")?;
    assert!(solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_dubois22_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois22")?;
    assert!(solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_zebra_v155_c1135_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("zebra_v155_c1135")?;
    assert_solves(&problem, &solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_aim_50_1_6_yes1_4_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("aim-50-1_6-yes1-4")?;
    assert_solves(&problem, &solve(&problem).unwrap());
    Ok(())
}

#[ignore] // Takes too long. I have never seen it finish.
#[test]
fn test_aim_100_1_6_no_1() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("aim-100-1_6-no-1")?;
    assert!(solve(&problem).is_none());
    Ok(())
}
