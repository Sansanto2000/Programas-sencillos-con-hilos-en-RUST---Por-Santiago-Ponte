pub fn min_max(vector: &[isize]) -> (isize, isize) {
    (0, 0)
}
 
pub fn concurrent_min_max(vector: Vec<isize>, threads: usize) -> (isize, isize) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        let threads = 2;

        let vector:Vec<isize> = Vec::from([7, 8, 2, 34, 2, 23, 1, 23, 5, 56, 207, 12]);

        let (min, max) = min_max(&vector);
        assert_eq!((min, max), (1, 207));

        let (min, max) = concurrent_min_max(vector, threads); //No me agrada que tenga que consumir el vector la vercion concurrente
        assert_eq!((min, max), (1, 207));
    }
}
