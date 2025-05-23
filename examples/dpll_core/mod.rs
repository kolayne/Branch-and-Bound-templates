use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::{env, io};

#[derive(Clone)]
pub struct Clause {
    literals: Vec<i32>,
}

pub enum ClauseState {
    Known(bool),
    OneLeft(i32),
    Unknown,
}

impl Clause {
    pub fn eval(&self, assignments: &[i8]) -> ClauseState {
        let mut left_literal = 0;
        let mut left_literal_cnt = 0;

        for &literal in &self.literals {
            if literal < 0 && assignments[-literal as usize] == -1
                || literal > 0 && assignments[literal as usize] == 1
            {
                return ClauseState::Known(true);
            }
            if assignments[literal.unsigned_abs() as usize] == 0 {
                left_literal = literal;
                left_literal_cnt += 1;
            }
        }

        match left_literal_cnt {
            0 => ClauseState::Known(false),
            1 => ClauseState::OneLeft(left_literal),
            _ => ClauseState::Unknown,
        }
    }
}

pub struct CnfSat {
    pub vars_cnt: u64,
    pub clauses: Vec<Clause>,
    // `vars_by_frequency[0]` is the most frequently used var
    pub vars_by_frequency: Vec<u32>,
}

/// Parses a CNF file.
///
/// The function code is horribly large and quite hard to read. Sorry about that.
/// Because it is only here to parse files for examples, I'm too lazy to
/// refactor it, it's sufficient for me that it works.
pub fn parse_cnf<R: io::Read>(read: R) -> Result<CnfSat, Box<dyn Error>> {
    let mut ans = CnfSat {
        vars_cnt: 0,
        clauses: vec![],
        vars_by_frequency: vec![],
    };

    let mut clauses_left = 0;

    let mut lines = io::BufRead::lines(io::BufReader::new(read));

    // Parse the `p cnf ... ...` problem declaration
    for line in &mut lines {
        let line = line?;
        let mut words = line.split_whitespace();
        match words.next() {
            None => continue,
            Some(word) => {
                match word {
                    "" | "c" => continue,

                    "p" => {
                        let ptype = String::from(words.next().unwrap_or(""));
                        if ptype != "cnf" {
                            return Err(Box::from(format!("Invalid problem type {}", ptype)));
                        }

                        let literal_cnt_s = words.next().unwrap_or("");
                        match literal_cnt_s.parse() {
                            Ok(val) => ans.vars_cnt = val,
                            Err(err) => {
                                return Err(Box::from(format!(
                                    "Failed to parse problem (number of literaliables) '{literal_cnt_s}': {err}"
                                )))
                            }
                        }

                        let clauses_cnt_s = words.next().unwrap_or("");
                        match clauses_cnt_s.parse() {
                            Ok(val) => clauses_left = val,
                            Err(err) => {
                                return Err(Box::from(format!(
                                "Failed to parse problem (number of clauses) '{clauses_cnt_s}': {err}"
                            )))
                            }
                        }

                        let extra_word = words.next();
                        if let Some(word) = extra_word {
                            return Err(Box::from(format!("Failed to parse problem: unexepcted word {word} found on the same line")));
                        }

                        // If successfully parsed problem, break!
                        break;
                    },

                    // The line is neither empty, nor comment, nor a problem declaration
                    _ => return Err(Box::from(format!("Unexpected line starting with {word}: expected a comment line or a problem declaration")))
                }
            }
        }
    }

    if clauses_left == 0 {
        return Err(Box::from("Problem is not declared"));
    }

    ans.clauses.reserve(clauses_left);

    // Here I could implement another heuristic: check if some variable only
    // appears with the same sign. Then can infer its value outright.
    //
    // While a practically useful heuristic, it isn't related to the library
    // testing, so I don't want to spend time on this.

    let mut vars_freq: HashMap<u32, u64> = HashMap::new();

    // Now that the problem declaration is parsed, parse the problem itself
    let mut literals: Vec<i32> = vec![];
    for line in &mut lines {
        for word in line?.split_whitespace() {
            if clauses_left == 0 {
                return Err(Box::from("Unexpectedly many clauses in the file"));
            }
            match word.parse() {
                Ok(0) => {
                    clauses_left -= 1;
                    ans.clauses.push(Clause { literals });
                    literals = vec![];
                }

                Ok(literal) => {
                    literals.push(literal);
                    *vars_freq.entry(literal.unsigned_abs()).or_insert(0) += 1;
                }

                Err(err) => return Err(Box::from(format!("Failed to parse number {word}: {err}"))),
            }
        }
    }

    // If there are vars in the formula that are never used as literals, they haven't made it
    // into `vars_freq`. That's okay: the solver will terminate as soon as the formula is decided.

    // Note: the order of traversal of elements of a `HashMap` is undeterminate. When measuring
    // performance, you might want to use a deterministic comparator (uncomment below).
    // It also seems like there are some lucky orders and there may be good heuristics to find them
    // but that's not my goal, so I'm not implementing this.
    let mut vars_freq: Vec<(u32, u64)> = vars_freq.into_iter().collect();
    vars_freq.sort_by(|(_v1, f1), (_v2, f2)| /*if f1 == f2 { _v1.cmp(_v2) } else*/ { f2.cmp(f1) });
    ans.vars_by_frequency = vars_freq.into_iter().map(|(v, _f)| v).collect();

    Ok(ans)
}

pub fn assert_solves(problem: &CnfSat, assignments: &[i8]) {
    for clause in &problem.clauses {
        match clause.eval(assignments) {
            ClauseState::Known(true) => {}

            // Using an `assert!` rather than `panic!`/`unreachable!` because the
            // intention of this function is to assert.
            //
            // Note: `assert!` does not get optimized out, unlike `debug_assert!`.
            #[allow(clippy::assertions_on_constants)]
            _ => assert!(false),
        }
    }
}

pub fn examples_main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::from(
            "Expected filename as the (only) command-line argument",
        ));
    }

    let f = File::open(&args[1])?;
    let problem = parse_cnf(f)?;

    match super::solve(&problem) {
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
    assert_solves(&problem, &super::solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_quinn_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("quinn")?;
    assert_solves(&problem, &super::solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_hole6_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("hole6")?;
    assert!(super::solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_par8_1_c_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("par8-1-c")?;
    assert_solves(&problem, &super::solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_dubois20_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois20")?;
    assert!(super::solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_dubois21_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois21")?;
    assert!(super::solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_dubois22_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("dubois22")?;
    assert!(super::solve(&problem).is_none());
    Ok(())
}

#[test]
fn test_zebra_v155_c1135_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("zebra_v155_c1135")?;
    assert_solves(&problem, &super::solve(&problem).unwrap());
    Ok(())
}

#[test]
fn test_aim_50_1_6_yes1_4_cnf() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("aim-50-1_6-yes1-4")?;
    assert_solves(&problem, &super::solve(&problem).unwrap());
    Ok(())
}

#[ignore] // Takes too long. I have never seen it finish.
#[test]
fn test_aim_100_1_6_no_1() -> Result<(), Box<dyn Error>> {
    let problem = open_sample_problem("aim-100-1_6-no-1")?;
    assert!(super::solve(&problem).is_none());
    Ok(())
}
