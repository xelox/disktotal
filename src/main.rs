use std::process::Command;

fn main() {
    let command = Command::new("lsblk")
        .arg("--output")
        .arg("FSUSED,FSSIZE")
        .arg("--bytes")
        .output()
        .expect("failed attempt to run lsblk command");

    assert!(command.status.success());

    //parsing lines of stdout
    let total_free_bytes: u64 = String::from_utf8_lossy(&command.stdout)
        .lines()
        .skip(1)
        .filter_map(|line|{
            //checking that line is not blank
            if !line.chars().all(|c| c.is_whitespace()) {
                let disk_numbers: Vec<u64> = line.split_ascii_whitespace()
                    .filter(|str|{
                        !str.chars().all(|c| c.is_whitespace())
                    }).map(|str|{
                        str.parse::<u64>().expect("failed to parse expected number in stdout")
                    }).collect();

                //calculating available space on each disk
                    //FSUSED           //FSSIZE
                Some(disk_numbers[1] - disk_numbers[0])
            }
            else {
                //ignoring blank lines
                None
            }
        }).sum();

    //calculating GB of free space with 2 decimals of precision
    let total_free_gb = (total_free_bytes / 10u64.pow(7)) as f64 / 100.0;

    println!("{total_free_gb}");
}
