extern crate simple_matrix;
use simple_matrix::Matrix;

fn main() {
    println!("Hello, world!");
}

pub fn secu_div_and_win(a: Matrix<f64>, _b: Matrix<f64>) -> Matrix<f64> {
    a
} 

pub fn conc_div_and_win(a: Matrix<f64>, _b: Matrix<f64>) -> Matrix<f64> {
    a
} 

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_secu_div_and_win() {
        let n = 3;
        let iterator_1 = (0..).map(|_| {
            let number = rand::thread_rng().gen_range(1..101);
            let number:f64 = (number as f64) / 10.0;
            number
        });
        let iterator_2 = iterator_1.clone();
        let a:Matrix<f64> = Matrix::from_iter(n, n, iterator_1);
        let b:Matrix<f64> = Matrix::from_iter(n, n, iterator_2);

        let c: Matrix<f64> = {
            let a = a.clone();
            let b = b.clone();
            secu_div_and_win(a, b)
        };
        let d: Matrix<f64> = {
            let a = a.clone();
            let b = b.clone();
            a * b
        };

        assert_eq!(c, d);
    }

    #[test]
    fn conc_secu_div_and_win() {
        let n = 3;
        let iterator_1 = (0..).map(|_| {
            let number = rand::thread_rng().gen_range(1..101);
            let number:f64 = (number as f64) / 10.0;
            number
        });
        let iterator_2 = iterator_1.clone();
        let a:Matrix<f64> = Matrix::from_iter(n, n, iterator_1);
        let b:Matrix<f64> = Matrix::from_iter(n, n, iterator_2);

        let c: Matrix<f64> = {
            let a = a.clone();
            let b = b.clone();
            conc_div_and_win(a, b)
        };
        let d: Matrix<f64> = {
            let a = a.clone();
            let b = b.clone();
            a * b
        };
        
        assert_eq!(c, d);
    }
}
