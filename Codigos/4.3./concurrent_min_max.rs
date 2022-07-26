use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

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
