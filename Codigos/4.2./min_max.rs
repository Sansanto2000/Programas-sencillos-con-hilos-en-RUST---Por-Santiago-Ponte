pub fn min_max(vector: &[isize]) -> (isize, isize) {
    let mut min = 9999999;
    let mut max = -1;
    for num in vector.iter() {
        if *num < min { min = *num; }
        if *num > max { max = *num; }
    }
    (min, max)
}
