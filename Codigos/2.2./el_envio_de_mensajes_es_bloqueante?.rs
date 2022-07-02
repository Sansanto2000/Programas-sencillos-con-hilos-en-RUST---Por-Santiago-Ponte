use core::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("Hilo> Hola, acabo de enviar un mensaje que el progrma principal solo va recibir luego de esperar 10 segundos");
    });
    thread::sleep(Duration::from_secs(10));
    println!("Main> Hola, termine de esperar");
    let received = rx.recv().unwrap();
    println!("Main> Acabo de recibir el mensaje [mensaje = {}]", received);
    println!("Main> ¿Que opinas, piensas que tx.send() es bloqueante?");
}
