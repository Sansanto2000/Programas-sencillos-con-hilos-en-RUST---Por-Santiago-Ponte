// Estos tests compilaran, pero fallaran, ya que en este punto del desarrollo faltaba definir correctamente un par de metodos importantes.
// Si es de interes puede encontrar estos mismos tests, con los metodos correctamente implementados en 'Codigos/3./main.rs'

use rand::Rng; // Agregar 'rand = "0.8.3"' en cargo.toml

fn main() {
    println!("Hello world");
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

pub fn round_two_decimals(a: &[f64]) -> Vec<f64> {
    let mut b:Vec<f64> = Vec::new();
    for i in a.iter() {
        b.push((*i* 100.0).round() / 100.0);
    }
    b
}

pub fn mul_matrix(n: usize, m: usize, a: &[f64], b: &[f64]) -> Vec<f64> {
    vec![0.0]
}
 
pub fn concurrent_mul_matrix(threads:usize ,n: usize, m: usize, a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    vec![0.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_matrix() {
        let n = 4;
        let m = 4;
        let t = 2;

        let a = Vec::from([
            2.5, 3.2, 1.1, 2.0,
            2.4, 1.2, 4.2, 0.0,
            3.0, 2.6, 0.2, 2.0,
            5.0, 2.0, 1.2, 2.0
        ]);
        let b = Vec::from([
            1.4, 2.0, 2.0, 6.0,
            5.0, 2.0, 1.0, 2.7,
            8.0, 1.2, 1.0, 1.0,
            4.0, 4.0, 0.2, 2.0
        ]);
        let (_, _, b) = transpose_matrix(n, m, &b); //Santiago>  Al transponer previamente b se puede aprovechar mejor la localidad de los datos

        let c = mul_matrix(n, m, &a, &b);
        let c = round_two_decimals(&c);

        let d = concurrent_mul_matrix(t, n, m, a, b);
        let d = round_two_decimals(&d);

        let e = Vec::from([
            36.3, 20.72, 9.7, 28.74,
            42.96, 12.24, 10.2, 21.84,
            26.8, 19.44, 9.2, 29.22,
            34.6, 23.44, 13.6, 40.6
        ]);
        assert_eq!(e, c);
        assert_eq!(e, d);
    }

    #[test]
    fn test_rand_mul_matrix() {
        let n = 16;
        let m = 16;
        let threads = 2;
        let iterator = (0..(n*m)).map(|_| {
            let mut rng = rand::thread_rng();
            let number:f64 = rng.gen(); // generates a float between 0 and 100
            number * 100.0
        });
        let a = Vec::from_iter(iterator.clone());
        let b = Vec::from_iter(iterator);
        let (_, _, b) = transpose_matrix(n, m, &b); //Santiago>  Al transponer previamente b se puede aprovechar mejor la localidad de los datos

        let c = mul_matrix(n, m, &a, &b);       
        let d = concurrent_mul_matrix(threads, n, m, a, b);
        assert_eq!(c, d);
    }
}
