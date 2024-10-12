# Building a Single-Page Rust Web Application with CRUD Functionality Using MySQL

![Project Structure](https://github.com/omacox/Learning_Rust/blob/main/rust_mysql_crud/static/index.png)

## Example

---

## **Introduction**

This guide outlines the steps to build a single-page web application in Rust that performs CRUD (Create, Read, Update, Delete) operations using a MySQL database. The application will use the **Actix-web** framework for handling HTTP requests and **Diesel** ORM for database interactions. This example is not complete configuration setup as to code the code files are complete.  This example is to compare the differences of examples and final code.

---

## **Prerequisites**

- **Rust**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).
- **MySQL Database**: Install and run a MySQL server on your machine.
- **Basic Knowledge**: Familiarity with Rust, web development concepts, and SQL.

---

## **Outline**

1. [Project Setup](#1-project-setup)
2. [Setting Up the Database](#2-setting-up-the-database)
3. [Configuring Diesel ORM](#3-configuring-diesel-orm)
4. [Defining the Data Model](#4-defining-the-data-model)
5. [Implementing CRUD Operations](#5-implementing-crud-operations)
6. [Creating HTTP Handlers](#6-creating-http-handlers)
7. [Setting Up Routing](#7-setting-up-routing)
8. [Building the Frontend](#8-building-the-frontend)
9. [Running the Application](#9-running-the-application)
10. [Testing and Debugging](#10-testing-and-debugging)
11. [Best Practices and Considerations](#11-best-practices-and-considerations)
12. [Deployment](#12-deployment)

---

## **1. Project Setup**

### **1.1. Create a New Rust Project**

Open your terminal and create a new Cargo project:

```bash
cargo new rust_mysql_crud --bin
cd rust_mysql_crud
```

### **1.2. Update `Cargo.toml`**

Add the necessary dependencies in your `Cargo.toml` file:

```toml
[package]
name = "rust_mysql_crud"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
actix-rt = "2.6"
diesel = { version = "2.0.0", features = ["mysql"] }
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.2", features = ["serde", "v4"] }
```

---

## **2. Setting Up the Database**

### **2.1. Install MySQL**

If you haven't already, install MySQL on your machine.

### **2.2. Create a Database and Table**

Log in to the MySQL server and create a new database:

```sql
CREATE DATABASE rust_crud;
USE rust_crud;
```

Create a table for your application. For example, let's create a simple `users` table:

```sql
CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);
```

---

## **3. Configuring Diesel ORM**

### **3.1. Install Diesel CLI**

Install the Diesel command-line tool, which helps manage migrations:

```bash
cargo install diesel_cli --no-default-features --features mysql
```

### **3.2. Set Up `.env` File**

Create a `.env` file in the root of your project to store the database URL:

```env
DATABASE_URL=mysql://username:password@localhost/rust_crud
```

Replace `username` and `password` with your MySQL credentials.

### **3.3. Run Diesel Setup**

Initialize Diesel in your project:

```bash
diesel setup
```

This command creates a `migrations` directory and a `diesel.toml` file.

### **3.4. Create a Migration**

Generate a new migration:

```bash
diesel migration generate create_users_table
```

Edit the generated migration files in `migrations/<timestamp>_create_users_table/` to match your table schema.

**Up Migration (`up.sql`):**

```sql
CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);
```

**Down Migration (`down.sql`):**

```sql
DROP TABLE users;
```

Run the migration:

```bash
diesel migration run
```

---

## **4. Defining the Data Model**

### **4.1. Create a `models.rs` File**

In the `src` directory, create a `models.rs` file to define your data structures.

```rust
// src/models.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub email: String,
}
```

### **4.2. Create a `schema.rs` File**

Run the following command to generate the `schema.rs` file automatically:

```bash
diesel print-schema > src/schema.rs
```

---

## **5. Implementing CRUD Operations**

### **5.1. Establish Database Connection**

Create a `db.rs` file to manage the database connection:

```rust
// src/db.rs
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
```

### **5.2. Implement CRUD Functions**

In your `models.rs`, implement functions for each CRUD operation.

**Create a New User:**

```rust
// src/models.rs
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

impl User {
    pub fn create(conn: &MysqlConnection, name: &str, email: &str) -> QueryResult<User> {
        let new_user = User {
            id: Uuid::new_v4().as_bytes().to_vec(),
            name: name.to_string(),
            email: email.to_string(),
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
        Ok(new_user)
    }
}
```

**Read Users:**

```rust
impl User {
    pub fn read(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }
}
```

**Update a User:**

```rust
impl User {
    pub fn update(conn: &MysqlConnection, user_id: Vec<u8>, name: &str, email: &str) -> QueryResult<User> {
        diesel::update(users::table.find(user_id.clone()))
            .set((users::name.eq(name), users::email.eq(email)))
            .execute(conn)?;
        users::table.find(user_id).first(conn)
    }
}
```

**Delete a User:**

```rust
impl User {
    pub fn delete(conn: &MysqlConnection, user_id: Vec<u8>) -> QueryResult<usize> {
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}
```

---

## **6. Creating HTTP Handlers**

### **6.1. Create a `handlers.rs` File**

Create a new file `handlers.rs` to define your HTTP request handlers.

```rust
// src/handlers.rs
use crate::db;
use crate::models::User;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn get_users() -> HttpResponse {
    let conn = db::establish_connection();
    match User::read(&conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

// Implement other handlers: create_user, update_user, delete_user
```

### **Example: Create a New User Handler**

```rust
pub async fn create_user(user_data: web::Json<User>) -> HttpResponse {
    let conn = db::establish_connection();
    match User::create(&conn, &user_data.name, &user_data.email) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
```

---

## **7. Setting Up Routing**

### **7.1. Modify `main.rs`**

Update your `main.rs` to set up Actix-web and define routes.

```rust
// src/main.rs
mod db;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(handlers::get_users))
            .route("/users", web::post().to(handlers::create_user))
            // Add routes for update_user and delete_user
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

## **8. Building the Frontend**

### **8.1. Serve Static Files**

If you have a frontend built with HTML/CSS/JavaScript, you can serve static files using Actix-web.

Add the `actix-files` crate to your `Cargo.toml`:

```toml
actix-files = "0.6"
```

### **8.2. Update `main.rs` to Serve Static Files**

```rust
use actix_files::Files;

HttpServer::new(|| {
    App::new()
        .service(Files::new("/", "./static").index_file("index.html"))
        // Other routes
})
```

### **8.3. Create the Frontend**

Place your `index.html` and other static files in the `./static` directory.

**Example `index.html`:**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rust CRUD App</title>
</head>
<body>
    <h1>User Management</h1>
    <div id="app"></div>
    <script src="app.js"></script>
</body>
</html>
```

**Example `app.js`:**

Use JavaScript to interact with your backend API via AJAX requests.

```javascript
async function fetchUsers() {
    const response = await fetch('/users');
    const users = await response.json();
    // Render users in the DOM
}

fetchUsers();
```

---

## **9. Running the Application**

### **9.1. Start the Server**

In your project directory, run:

```bash
cargo run
```

### **9.2. Access the Application**

Open your web browser and navigate to `http://127.0.0.1:8080` to view the application.

---

## **10. Testing and Debugging**

### **10.1. Write Tests**

Create unit tests for your models and integration tests for your HTTP handlers.

**Example Test in `tests/integration_test.rs`:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_get_users() {
        let mut app = test::init_service(App::new().route("/users", web::get().to(handlers::get_users))).await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
```

### **10.2. Use Logging**

Add logging to your application for debugging purposes.

Add the `env_logger` crate to your `Cargo.toml`:

```toml
env_logger = "0.10"
log = "0.4"
```

Initialize the logger in `main.rs`:

```rust
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // Rest of the code
}
```

Use logging macros in your code:

```rust
use log::{info, error};

pub async fn get_users() -> HttpResponse {
    info!("Fetching users from database");
    // Rest of the code
}
```

---

## **11. Best Practices and Considerations**

### **11.1. Security**

- **Input Validation**: Always validate and sanitize user inputs.
- **Prepared Statements**: Diesel ORM uses prepared statements by default to prevent SQL injection.
- **HTTPS**: Use HTTPS in production environments to secure data in transit.

### **11.2. Error Handling**

- Provide meaningful error messages to the client.
- Use proper HTTP status codes (e.g., 404 for not found, 500 for internal server error).

### **11.3. Configuration Management**

- Use environment variables for configuration (e.g., database credentials).
- Consider using a crate like `config` to manage configurations across environments.

### **11.4. Code Organization**

- Keep your code modular by separating concerns (e.g., models, handlers, routes).
- Follow Rust's idiomatic practices for code readability and maintenance.

### **11.5. Testing**

- Write tests for each component.
- Use mocking for database interactions to test handlers without affecting the actual database.

---

## **12. Deployment**

### **12.1. Build for Release**

Compile your application in release mode for performance optimizations:

```bash
cargo build --release
```

### **12.2. Environment Configuration**

Ensure that your application reads configurations from environment variables or configuration files suited for the production environment.

### **12.3. Set Up a Web Server**

Consider using a reverse proxy like **NGINX** to serve your application and handle SSL termination.

### **12.4. Database Migration in Production**

Run database migrations on the production database before starting the application.

---

## **Conclusion**

You now have a basic single-page Rust web application that performs CRUD operations using a MySQL database. This application can serve as a foundation for more complex projects. Continue to enhance the application by adding features such as authentication, pagination, search functionality, and deploying it to a cloud platform.

---

## **Additional Resources**

- **Actix-Web Documentation**: [https://actix.rs/docs/](https://actix.rs/docs/)
- **Diesel ORM Documentation**: [https://diesel.rs/guides/getting-started](https://diesel.rs/guides/getting-started)
- **MySQL Rust Crate**: [https://github.com/blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple)
- **Serde Serialization Framework**: [https://serde.rs/](https://serde.rs/)

---
