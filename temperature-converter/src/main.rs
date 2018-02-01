use std::io;

fn main() {
    loop {
        // Prompt
        println!("Select a conversion option: \n");

        println!("(1) F -> C");
        println!("(2) C -> F");

        println!("\nEnter the corisponding number or 'exit' else to leave:");
        
        // Get option
        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line");

        let option = option.trim();

        if option != "1" && option != "2" {
            break;
        }

        // get temp
        println!("Enter temperature: ");

        let mut temp = String::new();
        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");
        
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        
        // Convert temp
        if option == "1" {
            println!("\nResult: {}C\n", (temp - 32.0) * (5.0/9.0));
        } else {
            println!("\nResult: {}F\n", (temp + 32.0) / (5.0/9.0));
        }
    
    }
}
