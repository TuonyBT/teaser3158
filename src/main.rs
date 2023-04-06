use itertools::{Itertools};


fn main() {


    // For any three digits a, b, c, N1, N3 and N5 will be the respective dot products of distinct permutations
    // of a 3-element vector [a, b, c] and a vector of powers of 10 [100, 10, 1].
    // Computationally it will be more efficient to keep each a, b, c in descending order and test permutations of the powers vector
    let three_digs = [100, 10, 1].into_iter().permutations(3).collect::<Vec<Vec<usize>>>();

    // From these permutations, take each 3-fold combination in which N1 > N3 > N5, given any descending series a, b, c
    // These will be stored in the vector 'sequences'
    let mut sequences: Vec<Vec<&Vec<usize>>> = Vec::new();

    // For testing valid ordering, we take the dot product of each element of a (N1, N3, N5) combination
    // with an arbitrary vector of descending values
    let dot_comp = vec![3, 2, 1];

    // The permutations method outputs in lexigraphical order so the first two begin with 100, the next with 10 and the last with 1
    // For N1 to be the largest value, it must have come from a powers vector with 100 as its first element
    // For N5 to be the largest value, it must not have come from a powers vector with 100 as its first element
    // N3 can have a first powers element equal to 100 or 10 and still be less than N1 but greater than N5

    for n1 in &three_digs[..2] {
        for n3 in &three_digs[..4] {
            if dot_prod(n1, &dot_comp) > dot_prod(n3, &dot_comp) {
                for n5 in &three_digs[2..] {
                    if dot_prod(n3, &dot_comp) > dot_prod(n5, &dot_comp) {
                        sequences.push(vec![n1, n3, n5]);   
                    }         
                }
            }
        }
    }

//  Alternative approach generates all possible combinations and then filters out invalid ones
//    let sequences = three_digs.iter().combinations(3)
//                                            .filter(|v| dot_prod(v[0], &dot_comp) > dot_prod(v[1], &dot_comp) 
//                                            && dot_prod(v[1], &dot_comp) > dot_prod(v[2], &dot_comp)
//                                            && v[0][0] == 100usize
//                                            && v[1][0] > 1usize
//                                            && v[2][0] < 100usize)    
//                                            .collect::<Vec<Vec<&Vec<usize>>>>();
//    println!("Sequences: {:?}", sequences);

//  Finally we take each valid sequence of powers vectors and apply it to every possible combination of 3 descending digits a, b, c
//  We can reduce the amount of looping with the following insights:
//  - a must have at least two possible digits that are smaller, so a must be greater than 2
//  - for a difference between N1 and N3 which is less than 100, either the coefficient of 100 in N3 is equal to a or it is one less 
//  - for a difference between N3 and N5 which is less than 100, either the coefficient of 100 in N5 is equal to the equivalent 
//      coefficient in N3 or it is one less 

//  Let's test all possible values of N1, N3 and N5 according to these constraints, add the corresponding N2 and N4 and check the
//  additional requirements for these latter values

    for sequence in sequences {
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
                    let n_1 = dot_prod(sequence[0], &abc);
                    let n_3 = dot_prod(sequence[1], &abc);
                    let n_5 = dot_prod(sequence[2], &abc);
                    let n_2 = n_1 - n_3;
                    let n_4 = n_3 - n_5;

                    // N2 and N4 contain two digits
                    if n_2 > 99 || n_4 > 99 || n_2 < 10 || n_4 < 10 {continue}

                    // N2 and N1 do not share a first digit
                    if n_1/100 == n_2/10 {continue}

                    // all digits in N2 and N4 come from the three used by the odd Ns
                    if !abc.contains(&(n_2/10)) {continue}
                    if !abc.contains(&(n_2%10)) {continue}
                    if !abc.contains(&(n_4/10)) {continue}
                    if !abc.contains(&(n_4%10)) {continue}
                    println!("abc: {:?}", abc);
                    println!("N1: {}, N2: {}, N3: {}, N4: {}, N5: {}", n_1, n_2, n_3, n_4, n_5);
                }
            }
        }
    }
}

fn dot_prod(vctr: &Vec<usize>, mp: &Vec<usize>) -> usize {
    let dot: usize = vctr.iter().zip(mp).map(|(x, y)| x * y).sum();
    dot
}