use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    const MAX_THREADS: usize = 16;
    const INTERVAL: Duration = Duration::from_millis(5_000);

    println!("cpu-torturer!");
    println!("===");
    println!(
        "Gonna run {} threads just to make your CPU unhappy",
        MAX_THREADS
    );

    let mut threads = vec![];

    for i in 0..MAX_THREADS {
        let handle = std::thread::spawn(move || torture(i));
        threads.push(handle)
    }

    //std::thread::sleep(Duration::MAX);
    let mut sys = sysinfo::System::new_all();

    loop {
        watch(&mut sys);
        std::thread::sleep(INTERVAL);
    }
}

fn watch(sys: &mut sysinfo::System) {
    use sysinfo::*;
    sys.refresh_all();
    let current_process = sys
        .process(Pid::from_u32(std::process::id()))
        .expect("could not find current process");

    let processes = vec![current_process];

    let mut log_line = String::new();

    for process in processes {
        log_line.push_str(&format!(
            "{} CPU {}% - ",
            process.name(),
            process.cpu_usage()
        ));
    }

    for component in sys.components() {
        log_line.push_str(&format!(
            "{}={}C ",
            component.label(),
            component.temperature()
        ));
    }
    println!("{}", log_line);
}

fn torture(_i: usize) {
    //println!("Starting torture thread {}", i);
    let mut x = 1;
    loop {
        x *= -1;
    }
}
