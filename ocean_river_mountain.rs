// FILE:store.rs

// This file implements the 'An eco-friendly commercial cleaning service'

// Imports
use std::io;
use std::string::String;

// Structs

// This struct represents a cleaning service
struct CleaningService {
    name: String,
    eco_friendly: bool,
    cleaning_products: Vec<String>,
    services_offered: Vec<String>,
}

// This struct represents a customer
struct Customer {
    name: String,
    address: String,
    cleaning_needs: Vec<String>,
    budget: usize,
}

// This struct represents an order
struct Order {
    customer: Customer,
    cleaning_service: CleaningService,
    cleaning_needs: Vec<String>,
    price: usize,
}

// Functions

// This function creates a new cleaning service
fn create_cleaning_service(name: &str, eco_friendly: bool) -> CleaningService {
    CleaningService {
        name: name.to_string(),
        eco_friendly,
        cleaning_products: Vec::new(),
        services_offered: Vec::new(),
    }
}

// This function adds a cleaning product to a cleaning service
fn add_cleaning_product(cleaning_service: &mut CleaningService, product_name: &str) {
    cleaning_service.cleaning_products.push(product_name.to_string());
}

// This function adds a service to a cleaning service
fn add_service(cleaning_service: &mut CleaningService, service_name: &str) {
    cleaning_service.services_offered.push(service_name.to_string());
}

// This function creates a customer
fn create_customer(name: &str, address: &str) -> Customer {
    Customer {
        name: name.to_string(),
        address: address.to_string(),
        cleaning_needs: Vec::new(),
        budget: 0,
    }
}

// This function adds a cleaning need to a customer
fn add_cleaning_need(customer: &mut Customer, cleaning_need: &str) {
    customer.cleaning_needs.push(cleaning_need.to_string());
}

// This function adds a budget to a customer
fn add_budget(customer: &mut Customer, budget: usize) {
    customer.budget = budget;
}

// This function creates an order
fn create_order(
    customer: &Customer,
    cleaning_service: &CleaningService,
    cleaning_needs: &Vec<String>,
    price: usize,
) -> Order {
    Order {
        customer: customer.clone(),
        cleaning_service: cleaning_service.clone(),
        cleaning_needs: cleaning_needs.clone(),
        price,
    }
}

// This function prints the info of an order
fn print_order_info(order: &Order) {
    println!(
        "Customer: {}\nCleaning Service: {}\nCleaning Needs: {:?}\nPrice: {}",
        order.customer.name, order.cleaning_service.name, order.cleaning_needs, order.price
    );
}

// This function prints the info of a cleaning service
fn print_cleaning_service_info(cleaning_service: &CleaningService) {
    println!(
        "Name: {}\nEco-friendly: {}\nCleaning Products: {:?}\nServices Offered: {:?}",
        cleaning_service.name,
        cleaning_service.eco_friendly,
        cleaning_service.cleaning_products,
        cleaning_service.services_offered
    );
}

// This function processes orders
fn process_order(order: &mut Order) {
    println!("Processing order...");
    println!("Name: {}\nPrice: {}", order.customer.name, order.price);
}

// This function calculates the price for an order
fn calculate_price(order: &mut Order) {
    let mut price = 0;
    for cleaning_need in &order.cleaning_needs {
        match cleaning_need.as_str() {
            "Carpet Cleaning" => price += 200,
            "Window Cleaning" => price += 100,
            "Floor Cleaning" => price += 150,
            _ => println!("Invalid cleaning need"),
        }
    }
    order.price = price;
}

// Main
fn main() {
    // Create the cleaning service
    let mut cleaning_service = create_cleaning_service("Eco-Friendly Cleaners", true);
    // Add cleaning products to the cleaning service
    add_cleaning_product(&mut cleaning_service, "Eco-Friendly Cleaning Solution");
    add_cleaning_product(&mut cleaning_service, "Broom and Dustpan");
    add_cleaning_product(&mut cleaning_service, "Microfiber Cloths");
    // Add services to the cleaning service
    add_service(&mut cleaning_service, "Carpet Cleaning");
    add_service(&mut cleaning_service, "Window Cleaning");
    add_service(&mut cleaning_service, "Floor Cleaning");
    // Print the cleaning service info
    print_cleaning_service_info(&cleaning_service);
    // Get customer info from the user
    println!("Enter customer info:");
    let mut customer_name = String::new();
    println!("Name:");
    io::stdin().read_line(&mut customer_name).expect("Error reading name");
    let mut customer_address = String::new();
    println!("Address:");
    io::stdin().read_line(&mut customer_address).expect("Error reading address");
    let mut customer_cleaning_needs = String::new();
    println!("Cleaning needs:");
    io::stdin().read_line(&mut customer_cleaning_needs).expect("Error reading cleaning needs");
    let customer_cleaning_needs: Vec<_> =
        customer_cleaning_needs.split_whitespace().map(String::from).collect();
    let mut customer_budget = String::new();
    println!("Budget:");
    io::stdin().read_line(&mut customer_budget).expect("Error reading budget");
    let customer_budget: usize = customer_budget.trim().parse().expect("Invalid budget");
    // Create the customer
    let mut customer = create_customer(&customer_name, &customer_address);
    // Add the cleaning needs to the customer
    for cleaning_need in &customer_cleaning_needs {
        add_cleaning_need(&mut customer, cleaning_need);
    }
    // Add the budget to the customer
    add_budget(&mut customer, customer_budget);
    // Create the order
    let mut order = create_order(&customer, &cleaning_service, &customer_cleaning_needs, 0);
    // Calculate the price
    calculate_price(&mut order);
    // Print the order info
    print_order_info(&order);
    // Process the order
    process_order(&mut order);
}