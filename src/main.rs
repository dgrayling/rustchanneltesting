use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

struct Thing {
    rx: std::sync::mpsc::Receiver<i32>,
    received: i32
}

fn getThing() -> (Sender<i32>,std::boxed::Box<Thing>) {
    let (tx, rx) = mpsc::channel::<i32>();

    let thing = Thing {
      rx: rx,
      received: 0
    };

    let boxedThing = Box::new(thing);

    return (tx,boxedThing);
}

fn startThingThread(thing: &mut Thing) -> std::thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        for received in thing.rx {
            thing.received = received;
            println!("Got: {}", received);
            println!("Got thing received: {}", thing.received);
        }
    });

    return handle;
}

fn main() {
    let (tx1, mut thing1) = getThing();
    let (tx2, mut thing2) = getThing();

    let h1 = thread::spawn(move || {
        for received in thing1.rx {
            thing1.received = received;
            println!("Got: {}", received);
            println!("Got thing received: {}", thing1.received);
        }
    });

    let h2 = thread::spawn(move || {
        for received in thing2.rx {
            println!("Got: {}", received);
        }
    });

    let h3 = thread::spawn(move || {
        tx1.send(2).unwrap();
    });

    let h4 = thread::spawn(move || {
        tx2.send(2).unwrap();
    });

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    h4.join().unwrap();
}