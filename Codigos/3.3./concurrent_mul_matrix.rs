use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

pub fn concurrent_mul_matrix(threads:usize ,n: usize, m: usize, a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut handles = Vec::new();
    let c:Vec<f64> = vec![0.0; n*m];
    
    let a_arc = Arc::new(a);
    let b_arc = Arc::new(b);
    let c_lock_arc = Arc::new(Mutex::new(c));
    let row_batch = n / threads;

    for thread in 0..threads {
        let a = Arc::clone(&a_arc);
        let b = Arc::clone(&b_arc);
        let c_lock = Arc::clone(&c_lock_arc);
        let row_begin = thread * row_batch;
        let row_end = row_begin + row_batch;
        handles.push(
            thread::spawn(move || {   
                let row_aux = mul_matrix(row_batch, m, &a[row_begin*m..row_end*m], &b[..]);
                let mut c = c_lock.lock().unwrap();
                copy_aux(&mut c[row_begin*m..row_end*m], &row_aux);
            })
        )
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let c_lock = Arc::try_unwrap(c_lock_arc).unwrap();
    let c:Vec<f64> = c_lock.into_inner().unwrap();
    c
}

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

fn copy_aux(a: &mut [f64], a_aux: &[f64]) {
    for i in 0..a_aux.len() {
        a[i] = a_aux[i];
    }
}
