use dotenv::dotenv;
use project::{ThreadPool, db::Database, handlers};
use std::fs;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    //load the env file
    dotenv().ok();
    // Initialize database
    let db = Database::new("project.db").expect("Failed to initialize database");
    println!("Database initialized successfully");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let db_clone = db.clone();

        pool.execute(move || {
            handle_connection(stream, db_clone);
        })
    }
}

fn handle_connection(mut stream: TcpStream, db: Database) {
    let mut buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.by_ref().lines().next().unwrap().unwrap();

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();

    let mut content_length = 0;
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap_or(0);
        }
    }

    let mut request_body = vec![0; content_length];
    buf_reader.read_exact(&mut request_body).unwrap();
    let request_body = String::from_utf8(request_body).unwrap();

    let (status_line, contents, content_type) = match (method, path) {
        // API endpoints
        //user apis
        ("POST", "/api/signup") => {
            let (status, body) = handlers::handle_signup(&request_body, &db);
            (status, body, "application/json")
        }
        ("POST", "/api/login") => {
            let (status, body) = handlers::handle_login(&request_body, &db);
            (status, body, "application/json")
        }
        ("GET", "/api/users") => {
            let (status, body) = handlers::handle_fetch_users(&db);
            (status, body, "application/json")
        }
        //book apis CRUD operations
        ("POST", "/api/books") => {
            let (status, body) = handlers::handle_add_book(&request_body, &db);
            (status, body, "application/json")
        }
        ("GET", "/api/books") => {
            let (status, body) = handlers::handle_fetch_books(&db);
            (status, body, "application/json")
        }
        ("GET", path) if path.starts_with("/api/books/") => {
            let id_part = path.trim_start_matches("/api/books/");
            match id_part.parse::<i64>() {
                Ok(book_id) => {
                    let (status, body) = handlers::handle_fetch_book(book_id, &db);
                    (status, body, "application/json")
                }
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    r#"{ "success": false, "message": "Invalid ID" }"#.to_string(),
                    "application/json",
                ),
            }
        }
        ("PATCH", path) if path.starts_with("/api/books/") => {
            let id_part = path.trim_start_matches("/api/books/");
            match id_part.parse::<i64>() {
                Ok(book_id) => {
                    let (status, body) = handlers::handle_edit_book(book_id, &request_body, &db);
                    (status, body, "application/json")
                }
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    r#"{ "success": false, "message": "Invalid ID" }"#.to_string(),
                    "application/json",
                ),
            }
        }
        ("DELETE", path) if path.starts_with("/api/books/") => {
            let id_part = path.trim_start_matches("/api/books/");
            match id_part.parse::<i64>() {
                Ok(book_id) => {
                    let (status, body) = handlers::handle_delete_book(book_id, &db);
                    (status, body, "application/json")
                }
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    r#"{ "success": false, "message": "Invalid ID" }"#.to_string(),
                    "application/json",
                ),
            }
        }
        //borrow book apis
        ("POST", "/api/borrow") => {
            let (status, body) = handlers::handle_borrow_book(&request_body, &db);
            (status, body, "application/json")
        }
        ("GET", path) if path.starts_with("/api/borrow/") => {
            let id_part = path.trim_start_matches("/api/borrow/");
            match id_part.parse::<i64>() {
                Ok(user_id) => {
                    let (status, body) = handlers::handle_fetch_borrowed_books(user_id, &db);
                    (status, body, "application/json")
                }
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    r#"{ "success": false, "message": "Invalid ID" }"#.to_string(),
                    "application/json",
                ),
            }
        }
        ("GET", "/api/borrow") => {
            let (status, body) = handlers::handle_fetch_all_borrowed_books(&db);
            (status, body, "application/json")
        }
        ("DELETE", path) if path.starts_with("/api/borrow/") => {
            let parts: Vec<&str> = path.trim_start_matches("/api/borrow/").split("/").collect();
            if parts.len() == 2 {
                if let (Ok(borrowed_id), Ok(book_id)) =
                    (parts[0].parse::<i64>(), parts[1].parse::<i64>())
                {
                    let (status, body) = handlers::handle_return_book(borrowed_id, book_id, &db);
                    (status, body, "application/json")
                } else {
                    let error = r#"{"success":false, "message":"Invalid IDs"}"#;
                    (
                        "HTTP/1.1 400 Bad Request",
                        error.to_string(),
                        "application/json",
                    )
                }
            } else {
                let error = r#"{"success": false, "message": "Invalid path"}"#;
                (
                    "HTTP/1.1 400 Bad Request",
                    error.to_string(),
                    "application/json",
                )
            }
        }
        // HTML pages
        //login and signup
        ("GET", "/login.html") => match fs::read_to_string("frontend/login.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/signup.html") => match fs::read_to_string("frontend/signup.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        //admin functions CRUD operations
        ("GET", "/admin_dashboard.html") => {
            match fs::read_to_string("frontend/admin_dashboard.html") {
                Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
                Err(_) => (
                    "HTTP/1.1 404 NOT FOUND",
                    "<h1>404 Page Not Found</h1>".to_string(),
                    "text/html",
                ),
            }
        }
        ("GET", "/manage_books.html") => match fs::read_to_string("frontend/manage_books.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/create_book.html") => match fs::read_to_string("frontend/create_book.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/edit_book.html") => match fs::read_to_string("frontend/edit_book.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/users.html") => match fs::read_to_string("frontend/users.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/borrow_details_admin.html") => {
            match fs::read_to_string("frontend/borrow_details_admin.html") {
                Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
                Err(_) => (
                    "HTTP/1.1 404 NOT FOUND",
                    "<h1>404 Page Not Found</h1>".to_string(),
                    "text/html",
                ),
            }
        }
        //user functions
        ("GET", "/dashboard.html") => match fs::read_to_string("frontend/dashboard.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/browse_books.html") => match fs::read_to_string("frontend/browse_books.html") {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        ("GET", "/borrow_details.html") => match fs::read_to_string("frontend/borrow_details.html")
        {
            Ok(html) => ("HTTP/1.1 200 OK", html, "text/html"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "<h1>404 Page Not Found</h1>".to_string(),
                "text/html",
            ),
        },
        //js files
        //login and signup
        ("GET", "/js/signup.js") => match fs::read_to_string("frontend/js/signup.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        ("GET", "/js/login.js") => match fs::read_to_string("frontend/js/login.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        //admin functions CRUD operations
        ("GET", "/js/manage_books.js") => match fs::read_to_string("frontend/js/manage_books.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        ("GET", "/js/create_book.js") => match fs::read_to_string("frontend/js/create_book.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        ("GET", "/js/edit_book.js") => match fs::read_to_string("frontend/js/edit_book.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        //user function and CRUD operations
        ("GET", "/js/users.js") => match fs::read_to_string("frontend/js/users.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        ("GET", "/js/browse_books.js") => match fs::read_to_string("frontend/js/browse_books.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        ("GET", "/js/borrow_details.js") => {
            match fs::read_to_string("frontend/js/borrow_details.js") {
                Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
                Err(_) => (
                    "HTTP/1.1 404 NOT FOUND",
                    "console.error('JS file not found');".to_string(),
                    "application/javascript",
                ),
            }
        }
        ("GET", "/js/borrow_details_admin.js") => {
            match fs::read_to_string("frontend/js/borrow_details_admin.js") {
                Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
                Err(_) => (
                    "HTTP/1.1 404 NOT FOUND",
                    "console.error('JS file not found');".to_string(),
                    "application/javascript",
                ),
            }
        }
        ("GET", "/js/auth.js") => match fs::read_to_string("frontend/js/auth.js") {
            Ok(js) => ("HTTP/1.1 200 OK", js, "application/javascript"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "console.error('JS file not found');".to_string(),
                "application/javascript",
            ),
        },
        //styles
        ("GET", "/styles.css") => match fs::read_to_string("frontend/styles.css") {
            Ok(css_content) => ("HTTP/1.1 200 OK", css_content, "text/css"),
            Err(_) => (
                "HTTP/1.1 404 NOT FOUND",
                "/* CSS file not found */".to_string(),
                "text/css",
            ),
        },
        //default case
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            r#"{"error": "Not Found"}"#.to_string(),
            "application/json",
        ),
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
