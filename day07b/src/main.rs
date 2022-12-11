use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    filsz: usize,
}

fn main() {
    let mut cwd: Vec<&str> = Vec::new();
    let root = Dir { filsz: 0 };

    let mut dirs: HashMap<String, Dir> = HashMap::new();
    dirs.insert("".to_string(), root);

    include_str!("input.txt")
        .split("$")
        .skip(1)
        .map(|c| c.lines().collect::<Vec<&str>>())
        .for_each(|cc| {
            let mut c = cc.iter();
            let cmd: Vec<&str> = c.next().unwrap().split_whitespace().collect();
            match cmd[0] {
                "cd" => match cmd[1] {
                    ".." => {
                        cwd.pop();
                    }
                    "/" => {
                        cwd = Vec::new();
                    }
                    _ => {
                        cwd.push(cmd[1]);
                    }
                },
                "ls" => {
                    let fsz = c
                        .filter_map(|l| l.split_whitespace().nth(0).unwrap().parse::<usize>().ok())
                        .sum();
                    dirs.insert(cwd.join("/"), Dir { filsz: fsz });
                }
                _ => {
                    println!("Unknown command {:?}", cmd);
                }
            }
        });

    let dsz = {
        |d: &String| {
            dirs.iter()
                .filter_map(|(n, dir)| {
                    if n.starts_with(d.as_str()) {
                        Some(dir.filsz)
                    } else {
                        None
                    }
                })
                .sum()
        }
    };
    let dirsizes: Vec<(&String, usize)> = dirs.keys().map(|d| (d, dsz(d))).collect();

    let rootsz = dsz(&"".to_string());
    let to_free = rootsz - 40000000;
    let mut dd = dirsizes
        .iter()
        .filter_map(|(_, sz)| if *sz >= to_free { Some(*sz) } else { None })
        .collect::<Vec<usize>>();
    dd.sort();
    println!("size of dir to delete: {:?}", dd[0]);
}
