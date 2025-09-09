/* This is a sample document to show how to implement the SDK. */

use crate::models::*;
use serde_json::json;

//use crate::apis::users_api::*;

//use super::users_api::create_user;

use crate::apis::users_api::create_user;

use super::configuration::Configuration;


pub async fn run_example() {
    // Construct the configuration with the actual API base URL and authorization token
    println!("I am inside run_example in examples.rs");
    let mut config = Configuration::new();
    config.base_path = "http://localhost:9002".to_string();
    config.bearer_access_token = Some("Put your bearer token here".to_string());

    // Create the credentials, wrapping each field inside Some
    let credentials = Box::new(UserReqObjCredentials {
        username: Some("admin".to_string()), // Wrap in Some()
        secret: Some("password".to_string()),    // Wrap in Some()
    });

    let metadata = Some(json!({
        "domain": "example.com",
    }));
    // let cloned_credentials = credentials.clone();

    // Create the user request object
    let user_req_obj = UserReqObj {
        first_name: Some("Njeri".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("Njeri@example.com".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        credentials: credentials, // Boxed credentials
        metadata: metadata,
        profile_picture: Some("https://example.com/profile.jpg".to_string()),
        status: Some("enabled".to_string()),
    };
    // let token_request = IssueToken::new(
    //     cloned_credentials.username.as_ref().unwrap_or(&"".to_string()).clone(),
    //     cloned_credentials.secret.as_ref().unwrap_or(&"".to_string()).clone(),
    // );

    // Call the `create_user` function
    match create_user(&config, user_req_obj).await {
        Ok(user) => {
            // Success: The user has been created, and `user` contains the response data
            println!("User created successfully: {:?}", user);
        }
        Err(err) => {
            // Handle the error (e.g., logging, retrying, etc.)
            eprintln!("Error creating user: {:?}", err);
        }
    }

    //Obtain token
    // match issue_token(&config, token_request).await {
    //     Ok(user) => {
    //         // Success: The user has been created, and `user` contains the response data
    //         println!("Token generated successfully: {:?}", user);
    //     }
    //     Err(err) => {
    //         // Handle the error (e.g., logging, retrying, etc.)
    //         eprintln!("Error generating token: {:?}", err);
    //     }
    // }

    //Get user
    // let user_response: User =  match get_profile(&config).await {
    //     Ok(user) => {
    //         // Success: The user has been created, and `user` contains the response data
    //         return user;
    //     }
    //     Err(err) => {
    //         // Handle the error (e.g., logging, retrying, etc.)
    //         eprintln!("Error obtaining: {:?}", err);
    //     }
    // }
  
}

