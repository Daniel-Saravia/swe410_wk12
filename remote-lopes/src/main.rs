use std::process::Command;  // Import Command at the top

fn main() {
    println!("Daniel Saravia - April 5th");
    let ping_result = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg("8.8.8.8")
        .output();

    match ping_result {
        Ok(output) => {
            if output.status.success() {
                if let Ok(output_string) = String::from_utf8(output.stdout) {
                    println!("Ping Stats: \n{}", output_string);  // Corrected string interpolation
                } else {
                    println!("Failed to parse ping output");
                }
                println!("Internet is reachable");
            } else {
                println!("Internet is not reachable");
            }
        }
        Err(_) => {
            println!("Error executing the ping command");
        }
    }

    let curl_result = Command::new("curl")
        .arg("-Is")
        .arg("http://google.com")
        .output();

    match curl_result {
        Ok(output) => {
            if output.status.success() {
                println!("Successfully reached the website");
            } else {
                println!("Failed to reach the webpage");
            }
        }
        Err(_) => {
            println!("Error executing the curl command");
        }
    }
}
