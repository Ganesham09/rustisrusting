use std::collections::HashSet;

fn main(){
    
    let matrix = vec! [
    vec![2,3,4],
    vec![3,4,1],
    vec![6,8,9],
    ];

    
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut unique_numbers = HashSet::new();

    // let mut even_count = 0;
    
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] % 2 == 0 {
                 unique_numbers.insert(matrix[i][j]);

            }
        }
    }
    println!("the total count of even number is {}", unique_numbers.len());
}