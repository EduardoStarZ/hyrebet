use std::process::{Child, Command};
use ctrlc;
use std::sync::mpsc;
use std::thread;
use common::env;

fn main() {
    let (tx, rx) = mpsc::channel::<()>();

    ctrlc::set_handler(move || {
        println!("Shutdown initiated, killing programs");

        tx.send(()).expect("Could not send information");

    }).expect("Error setting up ctrlc");

    let args : Vec<String> = env::get_args();

    let worker = thread::spawn( move || {

        let mut auth : Child;
        let mut api : Child;
        let mut static_fs : Child;
        let mut vue_launcher : Child;

        if args[0].contains("bin") {

            auth = match Command::new("./bin/auth").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for auth not built! ")
            };

            static_fs = match Command::new("./bin/static-fs").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for static filesystem not built! ")
            };

            api = match Command::new("./bin/api").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for API not built! ")
            };

            vue_launcher = match Command::new("./bin/vue-launcher").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for Vue not built! ")
            };


        } else {
            auth = match Command::new("./dev/auth").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for auth not built! ")
            };

            static_fs = match Command::new("./dev/static-fs").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for static filesystem not built! ")
            };

            api = match Command::new("./dev/api").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for API not built! ")
            };

            vue_launcher = match Command::new("./dev/vue-launcher").spawn() {
                Ok(value) => value,
                Err(_) => panic!("Binary for Vue not built! ")
            };
        }

        loop {
            if rx.try_recv().is_ok() {
                api.kill().unwrap();
                auth.kill().unwrap();
                static_fs.kill().unwrap();
                vue_launcher.kill().unwrap();

                break;
            } 
        }

    });

    worker.join().expect("Could not execute paralel thread");
    println!("Program ended successfully"); 

}
