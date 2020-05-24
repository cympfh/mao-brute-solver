#[derive(Clone)]
enum RuleResult {
    Continue(String),
    End(String),
    NotApplied,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Rule {
    Replace(String, String),
    ReplaceEnd(String, String),
}

impl Rule {
    pub fn unwrap(&self) -> (&String, &String) {
        match self {
            Rule::Replace(s, t) => (s, t),
            Rule::ReplaceEnd(s, t) => (s, t),
        }
    }
    fn apply(&self, line: &str) -> RuleResult {
        match self {
            Rule::ReplaceEnd(s, t) => {
                let a: Vec<&str> = line.splitn(2, s).collect();
                if a.len() != 2 {
                    RuleResult::NotApplied
                } else {
                    RuleResult::End(String::from(a[0]) + t + a[1])
                }
            }
            Rule::Replace(s, t) => {
                let a: Vec<&str> = line.splitn(2, s).collect();
                if a.len() != 2 {
                    RuleResult::NotApplied
                } else {
                    RuleResult::Continue(String::from(a[0]) + t + a[1])
                }
            }
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rule::ReplaceEnd(s, t) => write!(f, "{}::{}", s, t),
            Rule::Replace(s, t) => write!(f, "{}:{}", s, t),
        }
    }
}

pub struct Program(pub Vec<Rule>);
impl Program {
    pub fn eval(&self, input: String, max_steps: usize, max_len: usize) -> Option<String> {
        let mut buf = input;
        for time in 0..=max_steps {
            if time == max_steps {
                // return Err(EvalErr::StepLimitExceeded);
                return None;
            }
            if buf.len() > max_len {
                return None;
                // return Err(EvalErr::LengthLimitExceeded);
            }
            let mut live = false;
            for rule in self.0.iter() {
                match rule.apply(&buf) {
                    RuleResult::Continue(buf_applied) => {
                        buf = buf_applied;
                        live = true;
                        break;
                    }
                    RuleResult::End(buf_applied) => {
                        buf = buf_applied;
                        break;
                    }
                    RuleResult::NotApplied => continue,
                }
            }
            if !live {
                break;
            }
        }
        Some(buf)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for rule in self.0.iter() {
            let _ = writeln!(f, "{}", &rule);
        }
        Ok(())
    }
}
