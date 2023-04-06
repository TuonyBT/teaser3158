use itertools::{Itertools};


fn main() {

    let mut three_digs: Vec<Vec<usize>> = Vec::new();

    for p in [100, 10, 1].into_iter().permutations(3) {
        three_digs.push(p);

    }

    let mut sequences: Vec<Vec<&Vec<usize>>> = Vec::new();
    let mut rejects: Vec<Vec<&Vec<usize>>> = Vec::new();
    let dot_comp = vec![3, 2, 1];

    for n1 in &three_digs[..2] {
        for n3 in &three_digs[..4] {
            if gen_num(n1, &dot_comp) > gen_num(n3, &dot_comp) {
                for n5 in &three_digs[2..] {
                    if gen_num(n3, &dot_comp) > gen_num(n5, &dot_comp) {
                        sequences.push(vec![n1, n3, n5]);   
                    }         
                    else {
                        rejects.push(vec![n1, n3, n5]);   
                    }         
                }
            }
//            else {
//                rejects.push(vec![n1, n3]);   
//            }
        }

    }


//    println!("Sequences: {:?}", sequences.len());
//    println!("Rejects: {:?}", rejects);

    for sequence in sequences {
//        println!("Sequence: {:?}", sequence);
//        let mut abc: Vec<usize> = Vec::new();
        for a in 3usize..10usize {
            let b_lower = match sequence[1][0] != sequence[0][0] {
                true => 2,
                   _ => a - 1,
            };
            for b in b_lower..a {
                let c_lower = match sequence[1][0] != sequence[2][0] {
                    true => 1,
                       _ => b - 1,
                };
                for c in c_lower..b {
                    let abc = vec![a , b , c ];
                    //println!("a: {}, b: {}, c: {}, abc: {:?}", a, b, c, abc);
                    let n_1 = gen_num(sequence[0], &abc);
                    let n_3 = gen_num(sequence[1], &abc);
                    let n_5 = gen_num(sequence[2], &abc);
                    let n_2 = n_1 - n_3;
                    let n_4 = n_3 - n_5;

                    if n_2 < 100 && n_4 < 100 && n_2 > 9 && n_4 > 10 {
                        let mut diff_digs: Vec<usize> = Vec::new();
                        let n_1_first = n_1/100;
                        let n_2_first = n_2/10;
                        let n_2_second = n_2%10;
                        let n_4_first = n_4/10;
                        let n_4_second = n_4%10;
                        if n_1_first == n_2_first {continue}
                        if !abc.contains(&n_2_first) {continue}
                        if !abc.contains(&n_2_second) {continue}
                        if !abc.contains(&n_4_first) {continue}
                        if !abc.contains(&n_4_second) {continue}

                        diff_digs.push(n_2/10);
                        diff_digs.push(n_2%10);
                        diff_digs.push(n_4/10);
                        diff_digs.push(n_4%10);
                        println!("abc: {:?}", abc);
                        println!("N1: {}, N2: {}, N3: {}, N4: {}, N5: {}", n_1, n_2, n_3, n_4, n_5);
                    }
                    
                    

                }
            }
        }

    }


}

fn gen_num(vctr: &Vec<usize>, mp: &Vec<usize>) -> usize {
    let dot = vctr[0] * mp[0] + vctr[1] * mp[1] + vctr[2] * mp[2];
//    println!("Vector: {:?}, multiplier: {:?}, result: {}", vctr, mp, dot);

    dot

}