use serde::Deserialize;
use std::collections::HashMap;
use reqwest::Error;
use std::io::{self, Write};

// Constructing a struct for storing the api response
#[derive(Deserialize, Debug)]
struct ExchangeRateResponse {
    base_code: String,
    conversion_rates: HashMap<String, f64>,
}



#[tokio::main]
async fn main() -> Result<(), Error> {
   

    println!("----------------------------------------------------");
    println!("--------------CURRENCY RATE EXCHANGE----------------");
    println!("----------------------------------------------------");
    
    // Replace YOUR_API_KEY with your real key
    let api_key = "";
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/USD", api_key);

    // Requesting Response from api

    let response = reqwest::get(&url).await?;
    let data: ExchangeRateResponse = response.json().await?;

    // Taking inputs from user
    
        // INPUT AMOUNT 
    let mut input_amount = String::new();
    print!("Enter an amount: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_amount)
        .expect("Failed to read line");

    let amount: f64 = input_amount.trim().parse()
        .expect("Please type a valid number");
    
        // FROM
    let mut from = String::new();
    print!("From: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut from)
        .expect("Failed to read line");

        // TO
    let mut to = String::new();
    print!("To: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut to)
        .expect("Failed to read ");

    let from = from.trim().to_uppercase();
    let to = to.trim().to_uppercase();
    
    // Converting 

    println!("------- OUTPUT ---------");

    println!(
    "{} {} to {} : {:.2}",
    amount, from, to, amount / data.conversion_rates[&from] * data.conversion_rates[&to]
    );

    Ok(())
}




