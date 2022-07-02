/// Los subprecesos no siempre tienen que actuar como emisores de los mensajes, tambien pueden ser emisores de los mismos
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("Hilo> Hola, acabo de recibir el mensaje de Main [mensaje = {}]", received);
    });
    let val = String::from("hi");
    tx.send(val).unwrap();
    println!("Main> Hola, acabo de enviar un mensaje");
    handle.join().unwrap();
    println!("Main> Acabo de ejecutar un join sobre Hilo y ahora voy a finalizar mi ejecucion");
}
