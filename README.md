# SuperMQ Rust SDK

## Introduction

Welcome to the SuperMQ Rust SDK! This SDK provides various functionalities, including APIs, models, and configuration management that you can integrate into your Rust projects. This guide will show you how to add the SDK to your project and use it effectively.


## Adding the SDK to Your Project

Follow these steps to add `sdk_rust` as a dependency:

1. **Add the SDK as a Git dependency**:

   Open your `Cargo.toml` file in the `my_rust_project` directory, and add the following dependency under `[dependencies]`:

   ```toml
   [dependencies]
   sdk_rust = { git = "https://github.com/absmach/sdk-rust", branch = "rust-sdk-dorcas" }
Replace your-username with your actual GitHub username or organization name, and sdk_rust with the path to the SDK crate within the supermq repository if it's not in the root.

2. **Build and run your project:**

With the SDK added as a dependency, you can now build and run your project using Cargo:

First, build the project:

```bash
cargo build 
```

Then, run the application:
```bash
cargo run 
```

## Using the SDK in Your Code
Once you've added the SDK to your project, you can begin using its functionality in your code.

**Example Project Flow**

1. Initialization: Configure the SDK for use (e.g., set up the API base URL, authentication token).
2. Function Calls: Use provided functions like run_example() to interact with the SDK’s functionality.
3. Error Handling: Catch errors and handle them (e.g., failed API requests).
Folder Structure of the SDK (sdk_rust)



Here’s an example of how you can use the users_api::create_user function to create a user in your main.rs file in your project:

```
use sdk_rust::apis::users_api::create_user;
use sdk_rust::models::{UserReqObj, UserReqObjCredentials};
use sdk_rust::configuration::Configuration;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Construct the configuration with the API base URL and bearer token
    let mut config = Configuration::new();
    config.base_path = "http://localhost:9002".to_string();
    config.bearer_access_token = Some("Put your bearer token here".to_string());

    // Create credentials for the user
    let credentials = Box::new(UserReqObjCredentials {
        username: Some("admin".to_string()),
        secret: Some("password".to_string()),
    });

    let metadata = Some(json!({
        "domain": "example.com",
    }));
    
    // Construct the user request object
    let user_req_obj = UserReqObj {
        first_name: Some("Njeri".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("njeri@example.com".to_string()),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        credentials: credentials,
        metadata: metadata,
        profile_picture: Some("https://example.com/profile.jpg".to_string()),
        status: Some("enabled".to_string()),
    };

    // Call the create_user function
    match create_user(&config, user_req_obj).await {
        Ok(user) => {
            println!("User created successfully: {:?}", user);
        }
        Err(err) => {
            eprintln!("Error creating user: {:?}", err);
        }
    }
}


```




## Getting Help
For additional details on how to use the SDK, refer to the SDK documentation in the apis.md and models.md files in the sdk-rust repository.

