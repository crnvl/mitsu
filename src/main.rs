use std::time::SystemTime;

fn main() {
    println!("Scanning the web... (this may take weeks to complete, online servers will be printed into the console)");

    let pre_calc_time = SystemTime::now();
    scan_ips();
    println!(
        "Scanned the internet in {:?} seconds",
        SystemTime::now().duration_since(pre_calc_time).unwrap()
    );
}

fn scan_ips() {
    for mon_one in (1..223).rev() {
        for mon_two in (0..255).rev() {
            for mon_three in (0..255).rev() {
                for mon_four in (0..255).rev() {
                    // println!(
                    //     "Requesting on: {}.{}.{}.{}:25565\r",
                    //     mon_one, mon_two, mon_three, mon_four
                    // );
                    if request_ip(&format!(
                        "{}.{}.{}.{}",
                        mon_one, mon_two, mon_three, mon_four
                    )) {
                        println!(
                            "{}.{}.{}.{} is online",
                            mon_one, mon_two, mon_three, mon_four
                        );
                    }
                }
            }
        }
    }
}

fn request_ip(ip: &str) -> bool {
    let resp = reqwest::blocking::get(&format!("https://api.mcsrvstat.us/2/{}", ip));
    let response = resp.unwrap().text().unwrap();
    response.contains("online\":true")
}
