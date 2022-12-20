fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let sample: Vec<Vec<isize>> = vec![
        vec![1, 2, -3, 3, -2, 0, 4],
        vec![2, 1, -3, 3, -2, 0, 4],
        vec![1, -3, 2, 3, -2, 0, 4],
        vec![1, 2, 3, -2, -3, 0, 4],
        vec![1, 2, -2, -3, 0, 3, 4],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 4, 0, 3, -2],
    ];
    let mut list = input.into_iter().enumerate().collect::<Vec<(usize, isize)>>();

    let len = list.len();

//    println!("list:                     {:?}", list.iter().map(|a| a.1).collect::<Vec<isize>>());
    for i in 0..len {
        let mut idx = 0;
        for j in 0..len {
            if list[j].0 == i {
                idx = j;
                break;
            }
        }
        let val: isize = list[idx].1;
        list.remove(idx);
        let mut newidx: usize = (4*len as isize - 4 + idx as isize + val) as usize % (len-1);
        if newidx == 0 {
            newidx = len-1;
        }
        list.insert(newidx as usize, (i, val));

//        println!("list:                     {:?}", list.iter().map(|a| a.1).collect::<Vec<isize>>());
//        println!("samp:                     {:?}\n", sample[i+1]);
    }

    let mut idx: usize = 0;
    for i in 0..len {
        if list[i].1 == 0 {
            idx = i;
        }
    }

    println!("0 is at {}", idx);

    let mut res = 0;

    for offset in [1000, 2000, 3000] {
        let val = list[(idx + offset) % len].1;
        println!("{}", val);
        res += val;
    }
    println!("res: {}", res);

    //println!("input: {:?}", input);
}

