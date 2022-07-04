use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    
    println!("Main> Contenido del Mutex m = {:?}", m);
    println!("Main> A continuacion se ejecutara un bloque de codigo que modificara su valor a 10");
    // Los simbolos { y } delimitan bloques de codigo
    {
        let mut num = m.lock().unwrap();  // Se bloquea el mutex obteniendo de ello una referencia mutable que sera guardada en num
        *num = 10;  // Se modifica el valor protegido
        drop(num);  // Se libera la referencia mutable, perdiendola y liberando el mutex en el proceso
        // A partir de aqui no se puede usar la variable num, ya que fue desechada.
        // Si alguna intruccion hiciera referencia a esta entonces el codigo no compilaria.
        // Si se quiere obtener otra referencia mutable para modificar la variable protegida entonces se debe bloquear otra vez el mutex
    } // El bloque de codigo finaliza por lo que automaticamente se llama a drop(num), en caso de que no haya sido llamada antes
    println!("Main> Bloque de codigo ejecutado");
    println!("Main> Contenido del Mutex m = {:?}", m);
    // Luego de ejecutar este codigo que los mutex capaz noto que ademas de data la estructura tambien contiene una 
    // variable llamada poisoned, esta determina si otro hilo hilo entro en panico mientras tenia el mutex bloqueado.
    // Si este fuera el caso entonces la ejecucion de "m.lock().unwrap();" provocaran que nuestro hilo tambien entre
    // en panico.
}
