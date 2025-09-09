use sdk_rust::apis::examples; // Replace `my_crate` with your actual crate name

#[tokio::main]
async fn main() {
    println!("Starting the application...");
    
    // Call the asynchronous function
    examples::run_example().await;

    println!("I have run run_example");
}