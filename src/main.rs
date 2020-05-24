use memoise::memoise;

mod mao;
use mao::{Program, Rule};

#[memoise(n <= 100_000_000)]
fn intpair(n: usize) -> (usize, usize) {
    let mut i = 0;
    let mut n = n;
    while n > i {
        i += 1;
        n -= i;
    }
    (n, i - n)
}

#[memoise(n <= 100_000_000)]
fn intstring(n: usize, words: &[&str]) -> String {
    if n == 0 {
        return String::new();
    }
    let m = words.len();
    let mut r = String::new();
    let mut index = n - 1;
    loop {
        r.push_str(words[index % m]);
        index /= m;
        if index == 0 {
            break;
        }
        index -= 1;
    }
    r
}

#[memoise(n <= 100_000_000)]
fn intrule(n: usize, words: &[&str]) -> Rule {
    let (x, y) = intpair(n / 2);
    let a = intstring(x, &words);
    let b = intstring(y, &words);
    if n % 2 == 0 {
        Rule::Replace(a, b)
    } else {
        Rule::ReplaceEnd(a, b)
    }
}

fn intprogram(lines: usize, n: usize, words: &[&str]) -> Program {
    let mut data = vec![];
    let mut index = n;
    for _ in 1..lines {
        let (x, y) = intpair(index);
        data.push(intrule(y, &words));
        index = x;
    }
    data.push(intrule(index, &words));
    Program(data)
}

/// 実行前に無駄であることを判定する
fn prune(prg: &Program) -> bool {
    for rule in prg.0.iter() {
        let (s, t) = rule.unwrap();
        if s == t {
            return true;
        }
    }
    let n = prg.0.len();
    for i in 0..n {
        for j in 0..i {
            if prg.0[i].unwrap().0 == prg.0[j].unwrap().0 {
                return true;
            }
        }
    }
    false
}

fn test(prg: &Program, testset: &[(String, String)], max_steps: usize, max_len: usize) -> bool {
    for (i, o) in testset.iter() {
        if prg.eval(i.clone(), max_steps, max_len) != Some(o.clone()) {
            return false;
        }
    }
    true
}

fn main() {
    let ws = vec!["AB", "BA", "AC", "CA", "BC", "CB"];
    let max_steps = 5000;
    let max_len = 5000;

    for idx in 0..100_000_000_000 {
        for lines in 3..=3 {
            let prg = intprogram(lines, idx, &ws);
            if prune(&prg) {
                continue;
            }
            let testset = vec![
                (String::from("ABC"), String::from("ABC")),
                (String::from("ACB"), String::from("ABC")),
                (String::from("CAB"), String::from("ABC")),
                (String::from("BCABBA"), String::from("AABBBC")),
            ];
            if test(&prg, &testset, max_steps, max_len) {
                println!("\x1b[2KFound!! lines={} idx={}", lines, idx);
                for rule in prg.0.iter() {
                    println!("\x1b[2K{}", &rule);
                }
                return;
            }

            println!("\x1b[2KTrying lines={} idx={}", lines, idx);
            for rule in prg.0.iter() {
                println!("\x1b[2K{}", &rule);
            }
            println!("\x1b[{}F", lines + 2);
        }
    }
}
