# Recommended Resources for Learning Rust

## Outline

1. **The Rust Programming Language (The Book)**: Often referred to simply as "The Book," this is the official guide to Rust and a great place to start. It covers Rust’s fundamental concepts like ownership, borrowing, and lifetimes, in a detailed, structured way. You’ll be building projects throughout the book to solidify your understanding. This is the go-to resource for most beginners.

   - **Best for**: Conceptual deep dives with practical examples.

   - [Rust Programming Language](https://doc.rust-lang.org/book/)

2. **Rustlings**: If you prefer a more hands-on approach, Rustlings is a collection of small exercises that helps you learn Rust by making you fix and modify Rust code. It works well alongside "The Book" and is great for reinforcing your understanding through practice.

   - **Best for**: Interactive, hands-on learning.

   - [Rustlings](https://github.com/rust-lang/rustlings)

3. **Tour of Rust**: This is an interactive course that gives a high-level overview of Rust, including its syntax and basic concepts. It’s a great starting point if you want to get familiar with Rust's core concepts before diving deeper into projects.

   - **Best for**: Interactive, lightweight introduction.

   - [Tour of Rust](https://tourofrust.com/)

4. **A Half-Hour to Learn Rust**: A fast-paced, no-fluff introduction to Rust concepts. It’s perfect if you’re short on time but want a quick overview of Rust’s syntax and mechanics.

   - **Best for**: Quick and dense learning.

   - [A Half-Hour to Learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)

5. **Crust of Rust**: A YouTube series by Jon Gjengset that dives deep into Rust's more advanced topics like lifetimes, iterators, and concurrency. It's useful when you're ready to tackle intermediate-to-advanced topics.

   - **Best for**: Video learners who want to dive deeper into Rust.

   - [Crust of Rust YouTube](https://www.youtube.com/c/JonGjengset)

6. **Exercism - Rust Track**: If you want structured, mentored exercises to practice Rust, Exercism’s Rust track provides real-world coding challenges with feedback from experienced developers.

   - **Best for**: Guided exercises with community feedback.

   - [Exercism Rust Track](https://exercism.org/tracks/rust)

By starting with "The Book" and complementing your learning with interactive tools like Rustlings and the Tour of Rust, you’ll build a solid understanding of the key Rust concepts before moving to more hands-on coding projects. Once you're comfortable with the basics, try resources like "Crust of Rust" to deepen your understanding.

Each of these resources focuses on building your knowledge of how Rust works conceptually before you write more complex programs.

If you're looking to understand the **core concepts of Rust** and its **use in various applications** before diving into code, several resources and approaches are available that emphasize learning how Rust operates conceptually:

1. **Rust's Ownership and Borrowing**: Rust's unique memory management model is central to its design. The language ensures memory safety without a garbage collector by enforcing strict rules around **ownership**. Each value in Rust has a single owner, and when that owner goes out of scope, the value is automatically deallocated. Rust’s **borrowing** feature allows you to access values without taking ownership, ensuring safe concurrent programming. This is key to understanding Rust’s safety guarantees before you start coding.

2. **Concurrency and Parallelism**: Rust is designed for **fearless concurrency**, which ensures that data races are caught at compile time. This means that Rust’s compiler checks your code for concurrency issues before it runs, making parallel programming much safer. Rust's ownership model plays a significant role in how concurrency works, allowing you to understand how to safely write concurrent applications.

3. **Error Handling**: Rust uses **Result** and **Option** types to handle errors, avoiding the need for exceptions. Understanding how Rust deals with errors at a conceptual level helps you build robust applications that are easy to debug and maintain.

4. **Real-World Applications**: Rust excels in **systems programming** (such as operating systems and game engines), **web assembly**, and **embedded systems**. Its low-level control combined with memory safety makes it a great choice for high-performance applications where both speed and safety are critical.

5. **Lifetimes and Traits**: These concepts define how long references should live and allow you to define shared behavior for different types. Understanding **lifetimes** and **traits** helps prevent common issues like dangling pointers and enhances code flexibility.

### Resources for Concept-Driven Learning

- **Rust Fundamentals on Coursera**: This course covers ownership, memory management, and more advanced topics like concurrency and error handling. It's great for understanding how Rust works before you dive into coding.

- **Ultimate Rust Programming Guide**: A comprehensive resource that introduces key concepts like memory safety, lifetimes, and concurrency in detail, explaining why Rust is designed the way it is.

- **Rapid Innovation's Rust Guide**: This guide explores Rust’s core features, from ownership to advanced concurrency, making it a good fit if you want to learn how to structure applications in Rust.

These resources focus on the **underlying concepts** of Rust and provide a solid foundation before you start writing code. Understanding these core principles will help you make better design choices when building Rust applications.

### Concept for a Full-Stack Web Application with Rust

Building a web application using Rust can provide high performance, memory safety, and concurrency guarantees that are hard to achieve with other languages. Rust is increasingly being adopted for both **backend** and **frontend** development due to its efficiency and the growing ecosystem. Here's a breakdown of how Rust can be used in a full-stack web application, including both backend and frontend components, with examples and explanations of features.

### Backend with Rust (using **Rocket** or **Actix**)

For the backend of your web application, Rust can be used to create a powerful, safe, and performant API. Rust web frameworks like **Rocket** or **Actix** are popular choices, offering different paradigms and use cases. Below is a typical structure and explanation of how the backend would be set up:

#### 1. **Web Framework**: **Rocket** or **Actix**

Rust frameworks like Rocket and Actix make it easy to build web servers that handle HTTP requests, manage routes, and connect to databases. They provide features such as request routing, middleware, and state management.

#### Example (using Rocket)

```rust

#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}

#[get("/user/<id>")]
fn get_user(id: u32) -> Json<User> {
    let user = User {
        id,
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
    };
    Json(user)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_user])
}

```

In this example:

- **Route Definition**: `#[get("/user/<id>")]` defines a route to fetch a user by ID.

- **Data Serialization**: Using `rocket::serde`, the `User` struct is serialized and deserialized to/from JSON.

- **Performance**: Rocket and Rust's async features provide great performance, allowing thousands of requests to be handled concurrently.

#### 2. **Database Integration**: Using **Diesel** or **SQLx**

Rust's strict type system helps prevent database-related errors. Libraries like **Diesel** or **SQLx** are often used to interact with databases.

#### Example with Diesel

```rust
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::Serialize;

#[derive(Queryable, Serialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

fn get_user_by_id(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(user_id)).first(conn)
}
```

In this example:

- **Diesel ORM** is used to perform database queries.

- **Rust's Type Safety**: Diesel’s query builder ensures that SQL queries are type-safe at compile time.

#### Key Features for the Backend

- **Memory Safety**: Rust’s ownership model prevents common memory-related issues like null pointer dereferences or use-after-free.

- **Concurrency**: Rust’s async/await model enables efficient handling of multiple requests concurrently, with no data races.

- **Error Handling**: With Rust's `Result` and `Option` types, errors are handled explicitly, avoiding the pitfalls of exceptions in other languages.

---

### Frontend with Rust (using **Yew** or **Seed**)

Rust is increasingly being used for frontend development, particularly for **WebAssembly (Wasm)**-powered apps. Frameworks like **Yew** or **Seed** allow you to write frontend code in Rust, which compiles to WebAssembly, offering a high-performance alternative to JavaScript-based frameworks.

#### 1. **Yew Framework**: A Rust-based frontend framework similar to React

Yew uses a component-based architecture, similar to React or Vue.js, but it compiles to WebAssembly, providing near-native performance in the browser.

#### Example in Yew

```rust
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    count: i64,
}

enum Msg {
    Increment,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { link: ctx.link().clone(), count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ self.count }</p>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "Increment" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```

In this example:

- **Component-based**: The `Model` struct represents a Yew component that renders HTML.

- **State Management**: The state (in this case, `count`) is managed inside the component.

- **WebAssembly Performance**: Since the app compiles to WebAssembly, you get near-native performance in the browser.

#### 2. **Integration with Backend**

The frontend can communicate with the Rust backend (Rocket or Actix) using standard HTTP APIs or WebSockets, which are fully supported in Yew.

#### Example of Fetching Data from the Backend

```rust
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct User {
    id: u32,
    username: String,
    email: String,
}

struct Model {
    users: Vec<User>,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let users = vec![];
        let fetch_task = None;

        Self {
            users,
            link: ctx.link().clone(),
            fetch_task,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let request = Request::get("/api/users").body(Ok("".to_string())).expect("Could not build request.");
            let callback = ctx.link().callback(|response: Response<Json<Result<Vec<User>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                // Handle data...
            });
            let task = FetchService::fetch(request, callback).expect("Failed to start request");
            self.fetch_task = Some(task);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "User List" }</h1>
                <ul>
                    { for self.users.iter().map(|user| html! { <li>{ &user.username }</li> }) }
                </ul>
            </div>
        }
    }
}
```

In this example:

- The frontend sends a request to the backend `/api/users` endpoint.

- The backend responds with JSON data, which is then rendered into the HTML.

#### Key Features for the Frontend

- **WebAssembly Performance**: Rust compiles to WebAssembly, offering better performance than JavaScript for CPU-intensive operations.

- **Type Safety**: The strong type system of Rust ensures that you catch errors at compile time.

- **Concurrency**: Even in the frontend, Rust’s async/await and strong memory guarantees make it efficient and reliable.

---

### Conclusion

By leveraging Rust for both the **backend** and **frontend** of a web application, you gain significant advantages in **performance**, **safety**, and **concurrency**. Rust’s strong type system and memory safety guarantees reduce bugs and crashes, while its WebAssembly support allows you to run efficient frontend code in the browser.

- **Backend**: Use **Rocket** or **Actix** for high-performance APIs, with the safety of Rust’s memory management.

- **Frontend**: Use **Yew** or **Seed** to build component-based WebAssembly applications that interact seamlessly with the backend.

This makes Rust a powerful option for full-stack web applications.

To create a **login** and **register** page using Rust with MySQL as the database backend, you can use a combination of the **Rocket** web framework, **Diesel** as the ORM (Object-Relational Mapper) for MySQL, and **Argon2** for password hashing. The goal is to implement a system that supports registration with fields such as username, email, password (with confirmation), PIN (with confirmation), first name, last name, and phone number for SMS recovery or two-factor authentication.

Here’s a breakdown of the process, including **Rust code snippets** for the key parts.

---

### 1. **Set Up MySQL Database**

You will need the following MySQL schema to manage the users:

#### MySQL Schema

```sql
CREATE TABLE users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(40) UNIQUE,
    email VARCHAR(100) UNIQUE,
    password_hash VARCHAR(255),
    pin_hash VARCHAR(255),
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    phone VARCHAR(15),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

```

This table includes:

- **username**: Unique identifier for the user.

- **email**: Used for confirmation and recovery.

- **password_hash**: Hashed password using Argon2 or similar.

- **pin_hash**: Hashed PIN for an extra layer of security.

- **phone**: For SMS-based authentication or recovery.
  
---

### 2. **Dependencies Setup (Cargo.toml)**

To use **Rocket** and **Diesel** with MySQL, add the following dependencies in your `Cargo.toml` file:

```toml

[dependencies]
rocket = { version = "0.5.0-rc.1", features = ["json", "tls"] }
diesel = { version = "1.4.8", features = ["mysql"] }
dotenv = "0.15"
serde = { version = "1.0", features = ["derive"] }
argon2 = "0.2"

```

- **Rocket**: Web framework for building web applications.

- **Diesel**: ORM for interacting with MySQL.

- **Argon2**: For secure password and PIN hashing.

- **Serde**: For handling JSON data.

---

### 3. **Database Configuration**

You will need to configure Diesel to connect to your MySQL database. Create a `.env` file to store the connection string:

`.env`:

```bash

DATABASE_URL=mysql://username:password@localhost/database_name

```

Run Diesel migrations:

```bash

diesel setup
diesel migration generate create_users_table

```

---

### 4. **Register Page Implementation**

#### Struct Definitions

Define the Rust structs for handling user input and database interaction.

```rust
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use argon2::{self, Config};

#[derive(Serialize, Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
    password_confirmation: String,
    pin: String,
    pin_confirmation: String,
    first_name: String,
    last_name: String,
    phone: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    username: &'a str,
    email: &'a str,
    password_hash: &'a str,
    pin_hash: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    phone: &'a str,
}
```

#### Password & PIN Hashing

Use **Argon2** to hash both the password and the PIN before saving them to the database.

```rust
fn hash_password(password: &str) -> String {
    let config = Config::default();
    let salt = b"randomsalt"; // You should generate a random salt per user.
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}
```

#### Registration Logic

The logic ensures that the password and PIN are hashed, and the user is inserted into the database.

```rust
#[post("/register", data = "<form>")]
fn register(conn: DbConn, form: Json<RegisterForm>) -> Result<String, String> {
    if form.password != form.password_confirmation {
        return Err("Password confirmation doesn't match".to_string());
    }
    if form.pin != form.pin_confirmation {
        return Err("PIN confirmation doesn't match".to_string());
    }

    let password_hash = hash_password(&form.password);
    let pin_hash = hash_password(&form.pin);

    let new_user = NewUser {
        username: &form.username,
        email: &form.email,
        password_hash: &password_hash,
        pin_hash: &pin_hash,
        first_name: &form.first_name,
        last_name: &form.last_name,
        phone: &form.phone,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&*conn)
        .map_err(|_| "Error inserting user".to_string())?;

    Ok("User registered successfully".to_string())
}
```

#### HTML for Registration Page

The front-end can be a simple HTML form for user registration.

```html
<form action="/register" method="post">
    <label>Username:</label>
    <input type="text" name="username">
    
    <label>Email:</label>
    <input type="email" name="email">
    
    <label>Password:</label>
    <input type="password" name="password">
    
    <label>Confirm Password:</label>
    <input type="password" name="password_confirmation">
    
    <label>PIN:</label>
    <input type="password" name="pin">
    
    <label>Confirm PIN:</label>
    <input type="password" name="pin_confirmation">
    
    <label>First Name:</label>
    <input type="text" name="first_name">
    
    <label>Last Name:</label>
    <input type="text" name="last_name">
    
    <label>Phone:</label>
    <input type="text" name="phone">
    
    <input type="submit" value="Register">
</form>
```

---

### 5. **Login Page Implementation**

#### Struct Definition for Login

The `LoginForm` will accept the username and password from the user.

```rust
#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
    pin: String,
}
```

#### Login Logic

You’ll need to validate the password and PIN against the hashed values stored in the database.

```rust
#[post("/login", data = "<form>")]
fn login(conn: DbConn, form: Json<LoginForm>) -> Result<String, String> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(&form.username))
        .first::<User>(&*conn)
        .map_err(|_| "User not found".to_string())?;

    let password_match = argon2::verify_encoded(&user.password_hash, form.password.as_bytes())
        .map_err(|_| "Invalid password".to_string())?;
    let pin_match = argon2::verify_encoded(&user.pin_hash, form.pin.as_bytes())
        .map_err(|_| "Invalid PIN".to_string())?;

    if password_match && pin_match {
        Ok("Login successful".to_string())
    } else {
        Err("Invalid credentials".to_string())
    }
}
```

#### HTML for Login Page

A simple form for login.

```html
<form action="/login" method="post">
    <label>Username:</label>
    <input type="text" name="username">
    
    <label>Password:</label>
    <input type="password" name="password">
    
    <label>PIN:</label>
    <input type="password" name="pin">
    
    <input type="submit" value="Login">
</form>
```

---

## Final Thoughts

Using Rust, you can implement a secure registration and login system with MySQL as the database backend. This example covers the key features of the system:

- **Password and PIN hashing** using Argon2.

- **User registration** with validation for username, email, password, and PIN.

- **Login system** with password and PIN verification.

This approach ensures security (with hashed passwords and PINs) and allows for future expansion, such as adding two-factor authentication via SMS (using the phone number field).

To handle **session information** and **cookies** in a Rust web application, especially for creating **login/registration pages** and securing access to a **dashboard page**, we will use the **Rocket** framework in combination with session or cookie management libraries like **rocket_contrib::cookies** or **jsonwebtoken** (for JWT-based sessions). Here’s a breakdown of how to handle this process:

### Key Features

1. **Session Management with Cookies**.

2. **Login and Registration Handling**.

3. **Redirection to Dashboard if Authenticated**.

4. **Display Welcome Message on Dashboard**.

---

### 1. **Setup**

First, ensure the following dependencies are included in your `Cargo.toml`:

```toml
[dependencies]
rocket = { version = "0.5.0-rc.1", features = ["json", "tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
argon2 = "0.2"
jsonwebtoken = "8.1"
```

### 2. **Session Management using Cookies**

We’ll use cookies to handle session information. A user will receive a session cookie when they successfully log in, and this cookie will be checked on subsequent requests to ensure the user is authenticated.

#### Creating a Login Handler with Cookies

Here’s how we’ll handle login:

```rust
#[macro_use] extern crate rocket;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, Serialize, json::Json};
use argon2::{self, Config};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<login_form>")]
fn login(jar: &CookieJar<'_>, login_form: Json<LoginForm>) -> Json<HashMap<&'static str, &'static str>> {
    let valid_username = "admin";  // Example data (should come from the database)
    let valid_password_hash = argon2::hash_encoded(b"password", b"somesalt", &Config::default()).unwrap();

    // Validate user credentials
    if login_form.username == valid_username && 
       argon2::verify_encoded(&valid_password_hash, login_form.password.as_bytes()).unwrap() {
        // Create a session by setting a cookie
        jar.add(Cookie::new("user_id", "admin"));

        Json(json!({ "message": "Login successful!" }))
    } else {
        Json(json!({ "message": "Invalid credentials!" }))
    }
}
```

- **Cookie Setup**: On successful login, a session cookie (`user_id`) is set in the user's browser.

- **Password Verification**: Passwords are hashed and verified using **argon2**.

#### Dashboard Handler with Session Check

The dashboard route will check whether the user is authenticated by checking the `user_id` cookie.

```rust
#[get("/dashboard")]
fn dashboard(jar: &CookieJar<'_>) -> Json<HashMap<&'static str, &'static str>> {
    if let Some(user_cookie) = jar.get("user_id") {
        let username = user_cookie.value();
        Json(json!({ "message": format!("Welcome to the dashboard, {}!", username) }))
    } else {
        Json(json!({ "error": "Unauthorized access, redirecting to login." }))
    }
}
```

- **Session Check**: If the `user_id` cookie exists, the user is considered authenticated.

- **Dashboard Access**: If the user is not authenticated, the dashboard will not load, and the user is informed to log in.

#### Redirecting Unauthorized Users

If the user is unauthorized (i.e., the session cookie is not set), redirect them to the login page with an error message:

```rust
use rocket::response::Redirect;

#[get("/dashboard")]
fn dashboard(jar: &CookieJar<'_>) -> Result<Json<HashMap<&'static str, String>>, Redirect> {
    if jar.get("user_id").is_none() {
        // If no valid session, redirect to the login page
        Err(Redirect::to(uri!(login_page)))
    } else {
        let mut response = HashMap::new();
        response.insert("message", "Welcome to the dashboard!".to_string());
        Ok(Json(response))
    }
}
```

In this case, the dashboard redirects unauthorized users to the login page.

#### Logout Handler

To log the user out, remove the session cookie:

```rust
#[get("/logout")]
fn logout(jar: &CookieJar<'_>) -> Json<HashMap<&'static str, &'static str>> {
    jar.remove(Cookie::named("user_id"));
    Json(json!({ "message": "Logged out successfully!" }))
}
```

---

### 3. **HTML Login Page**

You can set up a simple HTML form for login:

```html
<form action="/login" method="post">
    <label>Username:</label>
    <input type="text" name="username" required>
    
    <label>Password:</label>
    <input type="password" name="password" required>
    
    <input type="submit" value="Login">
</form>
```

This form sends a POST request to the `/login` route.

---

### 4. **HTML Dashboard**

Once logged in, users will be able to access the dashboard. You can show a welcome message:

```html
<div>
    <h1>Welcome to the Dashboard!</h1>
    <p>Welcome, user! You are now logged in.</p>
    <a href="/logout">Logout</a>
</div>
```

---

### 5. **Conclusion**

- **Login**: Handles user authentication by setting a session cookie on successful login.

- **Session Management**: Cookies are used to persist the session across requests.

- **Redirecting Unauthenticated Users**: If users attempt to access the dashboard without being authenticated, they are redirected to the login page.

- **Logout**: Session cookies are removed, and users are logged out.

This Rust setup ensures secure session management and protects routes based on authentication status. You can further enhance this with middleware, encryption, and multi-factor authentication using the provided phone number.
