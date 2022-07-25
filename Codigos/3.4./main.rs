use rand::Rng;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let n = 2048;
    let m = n;
    let threads = vec![1,2,4];
    let iterator = (0..(n*m)).map(|_| {
        let mut rng = rand::thread_rng();
        let number:f64 = rng.gen(); // generates a float between 0 and 100
        number * 100.0
    });
    let a = Vec::from_iter(iterator.clone());
    let b = Vec::from_iter(iterator);
    let (_, _, b) = transpose_matrix(n, m, &b); //Santiago>  Al transponer previamente b se puede aprovechar mejor la localidad de los datos

    let now = Instant::now();
    let c = mul_matrix(n, m, &a, &b);
    let secuencial_time = now.elapsed().as_secs_f32();

    let mut concurrent_times = Vec::new();
    for &t in threads.iter() {
        let a = a.clone();
        let b = b.clone();

        let now = Instant::now();
        let d = concurrent_mul_matrix(t, n, m, a, b);
        let duration = now.elapsed().as_secs_f32();
        concurrent_times.push(duration);

        assert_eq!(c, d);
    }

    println!("---------- Secuencial ----------");
    println!("Tiempo secuencial:    {} segundos", secuencial_time);
    for i in 0..threads.len() {
        let speedup = secuencial_time / concurrent_times[i];
        let efficiency = speedup / threads[i] as f32;
        println!("---------- Hilos = {} ----------", threads[i]);
        println!("Tiempo concurrente:   {} segundos", concurrent_times[i]);
        println!("Speedup:              {}", speedup);
        println!("Eficiencia:           {}", efficiency);
    }
}

pub fn transpose_matrix(n: usize, m: usize, a: &[f64]) -> (usize, usize, Vec<f64>) {
    let mut b:Vec<f64> = vec![0.0; n*m];
    for i in 0..n {
        for j in 0..m {
            b[i*m+j] = a[j*m+i];
        }
    }
    (m, n, b)
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

fn copy_aux(a: &mut [f64], a_aux: &[f64]) {
    for i in 0..a_aux.len() {
        a[i] = a_aux[i];
    }
}
