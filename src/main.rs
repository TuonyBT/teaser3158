use itertools::{Itertools};


fn main() {

    let mut three_digs: Vec<Vec<usize>> = Vec::new();

    for p in [0, 1, 2].into_iter().permutations(3) {
        three_digs.push(p);

    }

    let mut sequences: Vec<Vec<&Vec<usize>>> = Vec::new();

    for n1 in &three_digs[..2] {
        for n3 in &three_digs[..4] {
            if gen_num(n1) < gen_num(n3) {
                for n5 in &three_digs[2..] {
                    if gen_num(n3) < gen_num(n5) {
                        sequences.push(vec![n1, n3, n5]);   
                    }         
                }
            }
        }

    }


    println!("{:?}", sequences.len());
}

fn gen_num(vctr: &Vec<usize>) -> usize {
    vctr[0] * 100 + vctr[1] * 10 + vctr[2]

}