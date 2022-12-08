use std::io; 

fn calculate(before: f64, after: f64) -> f64{
    (after - before) / ((before + after) / 2.0) 
}

fn get_input(variable: &mut String, message_to_say: &str) -> f64{
    loop {
        println!("{}", message_to_say); 
        io::stdin()
            .read_line(variable)
            .expect("Failed to read line"); 

        let val = match variable.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue 
        }; 

        return val; 
    }
}

fn type_of_elasticity(eod: f64) {
    if eod == f64::INFINITY {
        println!("Type of elasticity: Perfectly Elastic"); 
    } else if eod > 1.0 {
        println!("Type of elasticity: Elastic"); 
    } else if eod == 1.0 {
        println!("Type of elasticity: Unitary"); 
    } else if eod == 0.0 {
        println!("Type of elasticity: Perfect Inelastic"); 
    } else {
        println!("Type of elasticity: Inelastic"); 
    }
}

fn main() {
    println!("Elasticity of Demand calculator");
    println!(""); 

    let mut qd_1 = String::new(); 
    let qd_1 = get_input(&mut qd_1, "Enter qd1:"); 

    let mut qd_2 = String::new(); 
    let qd_2 = get_input(&mut qd_2, "Enter qd2:"); 
   
    let mut p_1 = String::new(); 
    let p_1 = get_input(&mut p_1, "Enter p1:"); 

    let mut p_2 = String::new(); 
    let p_2 = get_input(&mut p_2, "Enter p2:"); 


    let percentage_quantity = calculate(qd_1, qd_2); 
    let percentage_price = calculate(p_1, p_2); 

    let eod = percentage_quantity / percentage_price; 

    println!(""); 
    println!("EOD is {eod}"); 
    type_of_elasticity(eod); 
}
