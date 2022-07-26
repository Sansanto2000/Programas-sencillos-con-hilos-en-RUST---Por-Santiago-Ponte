use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Instant;
use rand::Rng;

fn main() {
    let n = 1024;
    let threads = vec![1,2,4];

    let iter = (0..n).map(|i| {
        let number:isize;
        if i==2 { number = 1; }
        else if i==5 { number = 303; }
        else { number = rand::thread_rng().gen_range(1..=100); }
        number
    });

    let vector:Vec<isize> = Vec::from_iter(iter);

    let now = Instant::now();
    let (min, max) = min_max(&vector);
    let secuencial_time = now.elapsed().as_secs_f32();
    assert_eq!((min, max), (1, 303));

    let mut concurrent_times = Vec::new();
    for &t in threads.iter() {
        let vector = vector.clone();

        let now = Instant::now();
        let (min, max) = concurrent_min_max(vector, t);
        let duration = now.elapsed().as_secs_f32();
        concurrent_times.push(duration);

        assert_eq!((min, max), (1, 303));
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

pub fn min_max(vector: &[isize]) -> (isize, isize) {
    let mut min = 9999999;
    let mut max = -1;
    for num in vector.iter() {
        if *num < min { min = *num; }
        if *num > max { max = *num; }
    }
    (min, max)
}

pub fn concurrent_min_max(vector: Vec<isize>, threads: usize) -> (isize, isize) {
    let batch = vector.len() / threads;
    let vector_arc = Arc::new(vector);
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for thread in 0..threads {
        let vector = Arc::clone(&vector_arc);
        let tx = tx.clone();
        let begin = thread * batch;
        let end = begin + batch;
        handles.push(
            thread::spawn(move || {   
                let (min, max) = min_max(&vector[begin..end]);
                tx.send((min, max)).unwrap();
            })
        )
    }
    let mut min = 9999999;
    let mut max = -1;
    for _ in 0..threads {
        let (recv_min, recv_max) = rx.recv().unwrap();
        if recv_min < min { min = recv_min; }
        if recv_max > max { max = recv_max; }
    }
    (min, max)
}
