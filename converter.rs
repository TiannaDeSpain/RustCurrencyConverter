use std::io;
use std::collections::HashMap;

// function to convert oridonal amount to new currency value
fn convert(conversions: HashMap<&str, f64>, amount: f64, num2: u64) -> f64{
  if num2 == 1 {
    return amount * conversions.get("Pesos").unwrap();
  } else if num2 == 2{
    return amount * conversions.get("Euros").unwrap();
  } else if num2 == 3{
    return amount * conversions.get("Yen").unwrap();
  } else if num2 == 4{
    return amount * conversions.get("Dollars").unwrap();
  } else {
    return amount;
  }
}

// a function to output text decorations
fn decor(){ 

  // create a varriable to increase in loop
  let mut i = 0;

  // print user friendly text decorations in loop
  while i < 2 {
      println!("************************************");
      i = i + 1;
  }
}

fn main(){

  //print user friendly message
  decor();
  println!(" Welcome to Rust Currency Converter!");
  decor();

  // Prompt for what currency they currently have 
  println!("What currency do you have:");
  println!("(1) Pesos");
  println!("(2) Euros");
  println!("(3) Yen");
  println!("(4) US Dollars");
  
  // store the answer in a variable "num"
  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");
  // parsing the string to a number to be used for comparisons 
  let num = input_text.trim().parse::<u64>().expect("That's not a number");

  // create data structure to hold conversion numbers
  let mut conversions = HashMap::new();

  // prompt for what currency they would like to convert to
  println!("What currency to convert to:");

  if num == 1{

    // prompting only logical answers
    println!("(2) Euros");
    println!("(3) Yen");
    println!("(4) US Dollars");

    // adding corresponding conversion rates to hashmap
    conversions.insert("Dollars", 20.61);
    conversions.insert("Euros", 0.89);
    conversions.insert("Yen", 114.73);
    conversions.insert("Pesos", 0.0);

  }else if num ==2{

    // prompting only logical answers
    println!("(1) Pesos");
    println!("(3) Yen");
    println!("(4) US Dollars"); 

    // adding corresponding conversion rates to hashmap
    conversions.insert("Dollars", 1.12);
    conversions.insert("Pesos", 23.18);
    conversions.insert("Yen", 129.03);
    conversions.insert("Euros", 0.0);

  }else if num ==3{

    // prompting only logical answers
    println!("(1) Pesos");
    println!("(2) Euros");
    println!("(4) US Dollars");

    // adding corresponding conversion rates to hashmap
    conversions.insert("Dollars", 0.0087);
    conversions.insert("Pesos", 0.18);
    conversions.insert("Euro", 0.0078);
    conversions.insert("Yen", 0.0);

  }else if num == 4{

    // prompting only logical answers
    println!("(1) Pesos");
    println!("(2) Euros");
    println!("(3) Yen");

    // adding corresponding conversion rates to hashmap
    conversions.insert("Pesos", 20.61);
    conversions.insert("Euros", 0.89);
    conversions.insert("Yen", 114.73);
    conversions.insert("Dollars", 0.0);
  }
    
  // pulling input from question in if statment - what to convert to
  let mut input_text = String::new();
  io::stdin()
    .read_line(&mut input_text)
    .expect("Failed to read input");
  // parsing the string to a number to be used for comparisons 
  let num2 = input_text.trim().parse::<u64>().expect("That's not a number");
  

  // orgiginal amount to convert 
  println!("Enter amount to convert:");
  let mut amount_text = String::new();
  io::stdin()
    .read_line(&mut amount_text)
    .expect("Failed to read input");
  // parsing the string to a number to be used for calculations 
  let amount = amount_text.trim().parse::<f64>().expect("That's not a number");

  //printing result
  decor();
  println!("That is about {:.2} converted!", convert(conversions, amount, num2));
  decor();
}