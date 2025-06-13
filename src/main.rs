use project::{ThreadPool, db::Database, handlers};
use std::fs;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
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
        //user functions
        ("GET", "/dashboard.html") => match fs::read_to_string("frontend/dashboard.html") {
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
