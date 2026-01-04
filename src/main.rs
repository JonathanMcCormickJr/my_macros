use tokio::*;
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() {

    let mut handles = vec![];
    for i in 1..=4 {
        let handle = task::spawn(async move {
            execute_macros(i).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn execute_macros(i: usize) {
    rust!(i);
    liberty!(i);
    equipment!(i);
}

mod macros {
    #[macro_export]
    macro_rules! rust {
        ($i:expr) => {
            println!("{} Ferris", $i);
            sleep(Duration::from_millis(50));
            println!("{} Rustacean", $i);
            sleep(Duration::from_millis(50));
            println!("{} zero-cost abstractions", $i);
            sleep(Duration::from_millis(50));
            println!("{} fearless concurrency", $i);
            sleep(Duration::from_millis(50));
            println!("{} memory safety", $i);
            sleep(Duration::from_millis(50));
            println!("{} performance", $i);
            sleep(Duration::from_millis(50));
            println!("{} open source", $i);
            sleep(Duration::from_millis(50));
            println!("{} code", $i);
        };
    }

    #[macro_export]
    macro_rules! liberty {
        ($i:expr) => {
            println!("{} Bill of Rights", $i);
            sleep(Duration::from_millis(50));
            println!("{} Freedom of Speech", $i);
            sleep(Duration::from_millis(50));
            println!("{} Right to Bear Arms", $i);
            sleep(Duration::from_millis(50));
            println!("{} Freedom of Religion", $i);
            sleep(Duration::from_millis(50));
            println!("{} Right to Privacy", $i);
            sleep(Duration::from_millis(50));
            println!("{} Due Process", $i);
            sleep(Duration::from_millis(50));
            println!("{} Equal Protection under the Law", $i);
            sleep(Duration::from_millis(50));
            println!("{} Liberland", $i);
            sleep(Duration::from_millis(50));
            println!("{} New Hampshire", $i);
            sleep(Duration::from_millis(50));
            println!("{} Thomas Jefferson", $i);
            sleep(Duration::from_millis(50));
            println!("{} Thomas Massie", $i);
        };
    }

    #[macro_export]
    macro_rules! equipment {
        ($i:expr) => {
            println!("{} modem", $i);
            sleep(Duration::from_millis(50));
            println!("{} router", $i);
            sleep(Duration::from_millis(50));
            println!("{} switch", $i);
            sleep(Duration::from_millis(50));
            println!("{} firewall", $i);
            sleep(Duration::from_millis(50));
            println!("{} access point", $i);
            sleep(Duration::from_millis(50));
            println!("{} network interface card", $i);
            sleep(Duration::from_millis(50));
            println!("{} cables", $i);
            sleep(Duration::from_millis(50));
            println!("{} server", $i);
            sleep(Duration::from_millis(50));
            println!("{} storage device", $i);
            sleep(Duration::from_millis(50));
            println!("{} backup power supply", $i);
        };
    }
}