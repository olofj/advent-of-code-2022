use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Dir {
    files: Vec<(String, usize)>,
    dirs: Vec<String>,
}

fn main() {
    let mut cwd: Vec<&str> = Vec::new();
    let root = Dir {
        files: Vec::new(),
        dirs: Vec::new(),
    };

    let input: Vec<Vec<&str>> = include_str!("input.txt")
        .split("$")
        .skip(1)
        .map(|c| c.lines().collect::<Vec<&str>>())
        .collect();
//    println!("{:?}", input);

    let mut dirs: HashMap<String, Dir> = HashMap::new();
    dirs.insert("".to_string(), root);

    for cc in input {
        let mut c = cc.iter();
        let cmd: Vec<&str> = c
            .next()
            .unwrap()
            .split_whitespace()
            .collect();
        match cmd[0] {
            "cd" => {
//                println!("CD {:?}", cmd);
                match cmd[1] {
                    ".." => {
                        cwd.pop();
                    },
                    "/" => {
                        cwd = Vec::new();
                    },
                    _ => {
                        let parent = cwd.join("/");
                        cwd.push(cmd[1]);
                        dirs.get_mut(&parent).unwrap().dirs.push(cwd.join("/"));
                        dirs.insert(cwd.join("/"), Dir {
                            files: Vec::new(),
                            dirs: Vec::new(),
                        });
                    },
                }
//                println!("CWD {:?}", cwd.join("/"));
            },
            "ls" => {
//                println!("LS {:?}", cmd);
                let files: HashMap<String, usize> = c
                    .map(|e| e
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                    )
                    .filter(|l| !l[0].eq("dir"))
                    .map(|l| (l[0].to_string(), l[0].parse::<usize>().unwrap()))
                .collect();
                dirs.get_mut(&cwd.join("/")).unwrap().files.extend(files);
            },
            _ => {
                println!("Unknown command {:?}", cmd);
            },
        }
    }

    let alldirs = dirs.keys().collect::<Vec<&String>>();

    let dsz = { |d: &String|  dirs.keys()
        .filter(|n| n.starts_with(d.as_str()))
        .map(|sd| dirs
             .get(sd)
             .unwrap()
             .files
             .iter()
             .map(|(_, fsize)| fsize)
             .sum::<usize>()
        )
        .sum()
    };
    let dirsz: Vec<(&String, usize)> = alldirs.iter().map(|d| (*d, dsz(d))).collect();

    let rootsz = dsz(&"".to_string());
    let to_free = rootsz-40000000;
    println!("root {} to_free {}", rootsz, to_free);
    let mut dd = dirsz.iter().filter(|(_, sz)| *sz >= to_free).map(|(_, sz)| *sz).collect::<Vec<usize>>();
    dd.sort();
    println!("dd {:?}", dd);
}
