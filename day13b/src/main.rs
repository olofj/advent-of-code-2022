use serde_json::Value::{self, Array, Number};
use std::cmp::Ordering;

fn cmp(l: Value, r: Value) -> Option<bool> {

    match (l, r) {
        (Number(a), Array(b)) => cmp(Array(vec![Number(a)]), Array(b)),
        (Array(a), Number(b)) => cmp(Array(a), Array(vec![Number(b)])),
        (Array(a), Array(b)) => {
            let mut res = None;
            let mut a = a.into_iter();
            let mut b = b.into_iter();
            while res == None {
                let (ll, rr) = (a.next(), b.next());
                res = match (ll, rr) {
                    (Some(aa), Some(bb)) => cmp(aa, bb),
                    (None, Some(_)) => Some(true),
                    (Some(_), None) => Some(false),
                    (None, None) => break,
                }
            }
            res
        }
        (Number(a), Number(b)) => if a == b { None } else { Some(a.as_i64() < b.as_i64())},
        (_, _) => panic!("Unknown comparison"),
    }
}

fn cmpwrap(l: &Value, r:&Value) -> Ordering {
    match cmp(l.to_owned(),r.to_owned()) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        _ => panic!("Unknown comparison"),
    }
}

fn main() {
    let mut input = include_str!("input.txt")
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| serde_json::from_str(l).unwrap())
        .collect::<Vec<Value>>();
    input.push(serde_json::from_str("[[2]]").unwrap());
    input.push(serde_json::from_str("[[6]]").unwrap());

    input.sort_by(cmpwrap);

    let strout = input.iter().map(|v| v.to_string()).collect::<Vec<String>>();
    let two = strout.iter().position(|s| s == "[[2]]").unwrap()+1;
    let six = strout.iter().position(|s| s == "[[6]]").unwrap()+1;

    println!("two: {} six: {}", two, six);
    println!("product: {}", two * six);
}
