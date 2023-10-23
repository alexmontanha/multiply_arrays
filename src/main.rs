// Multiplicação de Matrizes: Um algoritmo para multiplicar duas matrizes.
fn main() {
    println!("Hello, world!");
}

fn multiply_matrix(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut c = vec![vec![0; a.len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a.len() {
            for k in 0..a.len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}