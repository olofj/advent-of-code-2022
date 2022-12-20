fn main() {
    let input = include_str!("sample.txt")
        .lines()
        .map(|l| l.parse::<isize>().unwrap() * 811589153)
        .collect::<Vec<isize>>();

    let sample: Vec<Vec<isize>> = vec![
        vec![811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612],
        vec![0, -2434767459, 3246356612, -1623178306, 2434767459, 1623178306, 811589153],
        vec![0, 2434767459, 1623178306, 3246356612, -2434767459, -1623178306, 811589153],
        vec![0, 811589153, 2434767459, 3246356612, 1623178306, -1623178306, -2434767459],
        vec![0, 1623178306, -2434767459, 811589153, 2434767459, 3246356612, -1623178306],
        vec![0, 811589153, -1623178306, 1623178306, -2434767459, 3246356612, 2434767459],
        vec![0, 811589153, -1623178306, 3246356612, -2434767459, 1623178306, 2434767459],
        vec![0, -2434767459, 2434767459, 1623178306, -1623178306, 811589153, 3246356612],
        vec![0, 1623178306, 3246356612, 811589153, -2434767459, 2434767459, -1623178306],
        vec![0, 811589153, 1623178306, -2434767459, 3246356612, 2434767459, -1623178306],
        vec![0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153],
    ];
    let mut list = input.into_iter().enumerate().collect::<Vec<(usize, isize)>>();

    let len = list.len();

//    println!("list:                     {:?}", list.iter().map(|a| a.1).collect::<Vec<isize>>());
//    println!("samp:                     {:?}", sample[0]);
    for iter in 1..=10 {
        for i in 0..len {
            let mut idx = 0;
            for j in 0..len {
                if list[j].0 == i {
                    idx = j;
                    break;
                }
            }
            let val: isize = list[idx].1;
            let mut newidx: usize = ((8115891530*len-8115891530) as isize + idx as isize + val) as usize % (len-1);
            if idx != newidx {
                list.remove(idx);
                if newidx == 0 {
                    newidx = len-1;
                }
                list.insert(newidx as usize, (i, val));
            }
    
//            println!("val {} idx {} newidx {}", val, idx, newidx);
        }
//        println!("list:                     {:?}", list.iter().map(|a| a.1).collect::<Vec<isize>>());
//        println!("samp:                     {:?}\n", sample[iter]);
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

