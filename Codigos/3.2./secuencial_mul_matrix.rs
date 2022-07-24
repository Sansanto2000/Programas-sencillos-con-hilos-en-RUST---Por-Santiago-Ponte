pub fn mul_matrix(n: usize, m: usize, a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut sum:f64;
    let mut c:Vec<f64> = Vec::with_capacity(n*m);
    for i in 0..n {
        for j in 0..m {
            sum = 0.0;
            for k in 0..m {
                sum += &a[i*m+k] * &b[j*m+k];   //Santiago> Si 'b' no estubiese transpuesta, esta linea seria: 'sum += &a[i*m+k] * &b[k*m+j];'
            }
            c.push(sum);
        }
    }
    c
} 
