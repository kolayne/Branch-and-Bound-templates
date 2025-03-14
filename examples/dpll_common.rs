use std::collections::HashMap;
use std::error::Error;
use std::io;

pub struct Clause {
    pub literals: Vec<i32>,
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

// BUG: fails to parse cnf-sat-samples/par8-1-c.cnf
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

    // TODO: could implement another heuristic: check if some variable only
    // appears with the same sign. Then can infer its value outright.
    //
    // While a practically useful heuristic, it isn't related to the library
    // testing, so I don't want to spend time on this.

    let mut vars_freq: HashMap<u32, u64> = HashMap::new();

    // Now that the problem declaration is parsed, parse the problem itself
    let mut literals: Vec<i32> = vec![];
    for line in &mut lines {
        if clauses_left == 0 {
            return Err(Box::from("Unexpectedly many clauses in the file"));
        }
        for word in line?.split_whitespace() {
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

    let mut vars_freq: Vec<(u32, u64)> = vars_freq.into_iter().collect();
    vars_freq.sort_by(|(_v1, f1), (_v2, f2)| f2.cmp(f1));
    ans.vars_by_frequency = vars_freq.into_iter().map(|(v, _f)| v).collect();

    Ok(ans)
}

pub fn assert_solves(problem: &CnfSat, assignments: &[i8]) {
    for clause in &problem.clauses {
        match clause.eval(assignments) {
            ClauseState::Known(true) => {}

            // Using an `assert!` rather than `panic`/`unreachable` because the
            // intention of this function is to assert.
            #[allow(clippy::assertions_on_constants)]
            _ => assert!(false),
        }
    }
}
