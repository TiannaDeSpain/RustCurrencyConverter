use std::io;
use std::collections::HashMap;

fn convert(conversions: &mut HashMap<&str, f64>, amount: f64, letter: &str) -> f64 {
    match letter {
      "P" => amount * conversions.get("Pesos").unwrap(),
      "E" => amount * conversions.get("Euros").unwrap(),
      "Y" => amount * conversions.get("Yen").unwrap(),
      "D" => amount * conversions.get("Dollars").unwrap(),
      _ => amount
    }
  }
  
fn main(){
    println!("What currency do you currently have:");
    println!("(1) Pesos");
    println!("(2) Euros");
    println!("(3) Yen");
    println!("(4) US Dollars");
    
    let mut input_text = String::new();
    io::stdin()
      .read_line(&mut input_text)
      .expect("Failed to read input");
      
    let num = input_text.trim().parse::<u64>().expect("That's not a number");

    let mut converstions = HashMap::new();

    println!("What currency do you want to convert to:");

    if num == 1{
      println!("(E) Euros");
      println!("(Y) Yen");
      println!("(U) US Dollars");
      converstions.insert("Dollars", 20.61);
      converstions.insert("Euros", 0.89);
      converstions.insert("Yen", 114.73);
    }else if num ==2{
      println!("(P) Pesos");
      println!("(Y) Yen");
      println!("(U) US Dollars"); 
      converstions.insert("Dollars", 1.12);
      converstions.insert("Pesos", 23.18);
      converstions.insert("Yen", 129.03);
    }else if num ==3{
      println!("(P) Pesos");
      println!("(E) Euros");
      println!("(U) US Dollars");
      converstions.insert("Dollars", 0.0087);
      converstions.insert("Pesos", 0.18);
      converstions.insert("Euro", 0.0078);
    }else if num == 4{
      println!("(P) Pesos");
      println!("(E) Euros");
      println!("(Y) Yen");
      converstions.insert("Pesos", 20.61);
      converstions.insert("Euros", 0.89);
      converstions.insert("Yen", 114.73);
    }
    
    let mut letter = String::new();
    io::stdin()
      .read_line(&mut letter)
      .expect("Failed to read input");
    
    println!("Enter amount to convert:");
    let mut amount_text = String::new();
    io::stdin()
      .read_line(&mut amount_text)
      .expect("Failed to read input");

    let amount = amount_text.trim().parse::<f64>().expect("That's not a number");

    println!("{} is {:.2}", amount, convert(&mut converstions, amount, &mut letter));
}