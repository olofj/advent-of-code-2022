use serde_json::Value::{self, Array, Number};

fn cmp(l: Value, r: Value) -> Option<bool> {
    match (l, r) {
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
        (Number(a), Array(b)) => cmp(Array(vec![Number(a)]), Array(b)),
        (Array(a), Number(b)) => cmp(Array(a), Array(vec![Number(b)])),
        (Number(a), Number(b)) => if a == b { None } else { Some(a.as_i64() < b.as_i64())},
        (_, _) => panic!("Unknown comparison"),
    }
}

fn main() {
    let input = include_str!("input.txt")
        .split("\n\n")
        .map(|p| p.split_once("\n").unwrap())
        .map(|(l, r)| {
            (
                serde_json::from_str(l).unwrap(),
                serde_json::from_str(r).unwrap(),
            )
        })
        .map(|(l, r)| match (l, r) {
            (Value::Number(x), Value::Array(y)) => {
                (Value::Array(vec![Value::Number(x)]), Value::Array(y))
            }
            (Value::Array(x), Value::Number(y)) => {
                (Value::Array(x), Value::Array(vec![Value::Number(y)]))
            }
            x => x,
        })
        .map(|(l, r)| cmp(l, r).unwrap())
        .collect::<Vec<bool>>();

    println!("input: {:?}", input);
    let out: usize = input.iter()
        .enumerate()
        .filter_map(|(a,b)| if *b { Some(a+1) } else { None })
        .sum();

    println!("result: {:?}", out);
}
