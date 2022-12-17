use std::collections::{HashMap, HashSet, VecDeque};

//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

fn main() {
    let mut valvemap: HashMap<String, usize> = HashMap::new();
    let input = include_str!("sample.txt")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let l = l.strip_prefix("Valve ").unwrap();
            let (valve, l) = l.split_once(" has flow rate=").unwrap();
            valvemap.insert(valve.to_string(), i);
            let (rate, l) = l
                .split_once("; tunnel leads to valve ")
                .or_else(|| l.split_once("; tunnels lead to valves "))
                .unwrap();
            let rate = rate.parse::<usize>().unwrap();
            let conn = l.split(", ").collect::<Vec<&str>>();
            (i, (rate, conn))
        })
        .collect::<Vec<(usize, (usize, Vec<&str>))>>()
        .into_iter()
        .map(|(a, (b, c))| {
            (
                a,
                (
                    b,
                    c.into_iter()
                        .map(|c| *valvemap.get(c).unwrap())
                        .collect::<Vec<usize>>(),
                ),
            )
        })
        .collect::<HashMap<usize, (usize, Vec<usize>)>>();

    for (valve, (rate, next)) in input.iter() {
        println!(" valve {} rate {} next {:?}", valve, rate, next);
    }

    let nodes = valvemap.len();
    let connections = (0..nodes)
        .map(|n| input[&n].1.clone())
        .collect::<Vec<Vec<usize>>>();

    let mut distances: Vec<Vec<usize>> = vec![vec![]; nodes];
    for node in 0..nodes {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut dist: Vec<isize> = vec![-1; nodes];
        dist[node] = 0;
        q.push_back(node);
        while let Some(n) = q.pop_front() {
            let d = dist[n];
            for conn in connections[n].iter() {
                if dist[*conn] == -1 || dist[*conn] > d + 1 {
                    dist[*conn] = d + 1;
                    q.push_back(*conn);
                }
            }
        }
        distances[node] = dist.iter().map(|i| *i as usize).collect::<Vec<usize>>();
    }

    fn fwd(level: usize, cur: usize, timeleft: usize, mut rem: HashSet<usize>, input: &HashMap<usize, (usize, Vec<usize>)>, distances: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
//    let fwd = |cur: usize, timeleft: usize, rem: HashSet<usize>| {
        rem.remove(&cur);
        let ret = rem.iter().filter_map(|r| {
            let time = distances[cur][*r] + 1;
            if time >= timeleft {
                None 
            } else {
                println!("{}cur {} r {}, time {} timeleft {} rem {:?}", String::from_iter(vec![" | "; level]), cur, r, time, timeleft, rem);
                let bestnext = fwd(level + 1, *r, timeleft-time, rem.clone(), input, distances);
                Some(((timeleft - time) * input[r].0 + bestnext.0, bestnext.1))
            }
        }).max_by_key(|v| v.0);
        println!("{}fwd cur {} timeleft {} ret {:?}", String::from_iter(vec![" | "; level]), cur, timeleft, ret);
        if let Some(ret) = ret {
            let mut retvec = vec![cur];
            retvec.append(&mut ret.1.clone());
            (ret.0, retvec)
        } else {
            (0, vec![cur])
        }
    }

    let mut timeleft = 30;
    let mut rem: HashSet<usize> = (0..nodes)
        .filter(|n| input[n].0 > 0)
        .collect::<HashSet<usize>>();
    let mut cur: usize = *valvemap.get("AA").unwrap();
    let new = fwd(0, cur, timeleft, rem, &input, &distances);
    println!("{} by visiting: {:?}", new.0, new.1);
}
