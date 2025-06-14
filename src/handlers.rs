use crate::db::Database;
use serde_json;
use serde_json::Value;

pub fn handle_signup(request_body: &str, db: &Database) -> (&'static str, String) {
    // Parse JSON
    let signup_data: serde_json::Value = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(_) => {
            let response = r#"{"success": false, "message": "Invalid JSON"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    let username = signup_data["username"].as_str().unwrap_or("");
    let password = signup_data["password"].as_str().unwrap_or("");

    // Validate input
    if username.trim().is_empty() || password.trim().is_empty() {
        let response = r#"{"success": false, "message": "Please provide all the fields"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Validate username length and characters
    if username.len() < 3 || username.len() > 50 {
        let response =
            r#"{"success": false, "message": "Username must be between 3 and 50 characters"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Validate password length
    if password.len() < 6 {
        let response =
            r#"{"success": false, "message": "Password must be at least 6 characters long"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Try to create user
    match db.create_user(username, password) {
        Ok(true) => {
            let response = format!(
                r#"{{"success": true, "message": "User '{}' created successfully"}}"#,
                username
            );
            ("HTTP/1.1 201 Created", response)
        }
        Ok(false) => {
            let response = r#"{"success": false, "message": "Username already exists"}"#;
            ("HTTP/1.1 409 Conflict", response.to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = r#"{"success": false, "message": "Internal server error"}"#;
            ("HTTP/1.1 500 Internal Server Error", response.to_string())
        }
    }
}

pub fn handle_login(request_body: &str, db: &Database) -> (&'static str, String) {
    // Parse JSON
    let login_data: serde_json::Value = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(_) => {
            let response = r#"{"success": false, "message": "Invalid JSON"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    let username = login_data["username"].as_str().unwrap_or("");
    let password = login_data["password"].as_str().unwrap_or("");

    // Validate input
    if username.trim().is_empty() || password.trim().is_empty() {
        let response = r#"{"success": false, "message": "Please provide username and password"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Verify user credentials
    match db.verify_user(username, password) {
        Ok(Some((user_id, username, role, jwt))) => {
            let response = format!(
                r#"{{"success": true, "message": "Login successful", "userId": "{}", "username": "{}", "role": "{}", "jwt": "{}"}}"#,
                user_id, username, role, jwt
            );
            ("HTTP/1.1 200 OK", response)
        }
        Ok(None) => {
            let response = r#"{"success": false, "message": "Invalid username or password"}"#;
            ("HTTP/1.1 401 Unauthorized", response.to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = r#"{"success": false, "message": "Internal server error"}"#;
            ("HTTP/1.1 500 Internal Server Error", response.to_string())
        }
    }
}

pub fn handle_add_book(request_body: &str, db: &Database) -> (&'static str, String) {
    let book_data: serde_json::Value = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(_) => {
            let response = r#"{"success": false, "message": "Invalid JSON"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    let title = book_data["title"].as_str().unwrap_or("");
    let author = book_data["author"].as_str().unwrap_or("");
    let isbn = book_data["isbn"].as_str().unwrap_or("");
    let publication_year_str = book_data["publication_year"].as_str().unwrap_or("");
    let genre = book_data["genre"].as_str().unwrap_or("");
    let number_of_copies = book_data["number_of_copies"].as_i64().unwrap_or(-1);

    if title.trim().is_empty()
        || author.trim().is_empty()
        || isbn.trim().is_empty()
        || publication_year_str.trim().is_empty()
        || genre.trim().is_empty()
        || number_of_copies <= 0
    {
        let response = r#"{"success": false, "message": "Please provide all the fields"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Validate ISBN is exactly 13 digits
    if isbn.len() != 13 || !isbn.chars().all(|c| c.is_ascii_digit()) {
        let response = r#"{"success": false, "message": "ISBN must be exactly 13 digits"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    // Validate publication year
    let publication_year: u16 = match publication_year_str.parse() {
        Ok(year) if year >= 1500 && year <= 2024 => year,
        _ => {
            let response = r#"{"success": false, "message": "Publication year must be a valid year between 1500 and 2024"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    // Validate number of copies
    if number_of_copies <= 0 || number_of_copies > i32::MAX as i64 {
        let response =
            r#"{"success": false, "message": "Number of copies must be a positive integer"}"#;
        return ("HTTP/1.1 400 Bad Request", response.to_string());
    }

    match db.add_book(
        title,
        author,
        isbn,
        &publication_year.to_string(),
        genre,
        number_of_copies as i32,
    ) {
        Ok(true) => {
            let response = r#"{"success": true, "message": "Book added successfully"}"#;
            ("HTTP/1.1 201 Created", response.to_string())
        }
        Ok(false) => {
            let response = r#"{"success": false, "message": "Book with this ISBN already exists"}"#;
            ("HTTP/1.1 409 Conflict", response.to_string())
        }
        Err(_) => {
            let response = r#"{"success": false, "message": "Database error occurred"}"#;
            ("HTTP/1.1 500 Internal Server Error", response.to_string())
        }
    }
}

pub fn handle_fetch_books(db: &Database) -> (&'static str, String) {
    match db.fetch_books() {
        Ok(books) => {
            let json = serde_json::to_string(&books).unwrap_or("[]".to_string());
            ("HTTP/1.1 200 OK", json)
        }
        Err(_) => {
            let error = r#"{"success": false, "message": "Could not fetch books"}"#;
            ("HTTP/1.1 500 Internal Server Error", error.to_string())
        }
    }
}

pub fn handle_fetch_book(book_id: i64, db: &Database) -> (&'static str, String) {
    match db.fetch_book(book_id) {
        Ok(Some(book)) => match serde_json::to_string(&book) {
            Ok(json) => ("HTTP/1.1 200 OK", json),
            Err(_) => (
                "HTTP/1.1 500 Internal Server Error",
                r#"{"success": false, "message": "Failed to serialize book"}"#.to_string(),
            ),
        },
        Ok(None) => (
            "HTTP/1.1 404 Not Found",
            r#"{"success": false, "message": "Book not found"}"#.to_string(),
        ),
        Err(_) => (
            "HTTP/1.1 500 Internal Server Error",
            r#"{"success": false, "message": "Could not fetch book"}"#.to_string(),
        ),
    }
}

pub fn handle_edit_book(id: i64, request_body: &str, db: &Database) -> (&'static str, String) {
    let updated_fields: Value = match serde_json::from_str::<Value>(request_body) {
        Ok(val) if val.is_object() => val,
        _ => {
            let error = r#"{ "success": false, "message": "Invalid JSON body" }"#;
            return ("HTTP/1.1 400 Bad Request", error.to_string());
        }
    };

    // Ensure at least one editable field is present
    let allowed_keys = [
        "title",
        "author",
        "isbn",
        "publication_year",
        "genre",
        "number_of_copies",
        "available",
    ];
    let has_valid_keys = updated_fields
        .as_object()
        .unwrap()
        .keys()
        .any(|k| allowed_keys.contains(&k.as_str()));

    if !has_valid_keys {
        let error = r#"{ "success": false, "message": "No valid fields provided to update" }"#;
        return ("HTTP/1.1 400 Bad Request", error.to_string());
    }

    // Call DB update function
    match db.edit_book(id, &updated_fields) {
        Ok(true) => {
            let response = r#"{ "success": true, "message": "Book updated successfully" }"#;
            ("HTTP/1.1 200 OK", response.to_string())
        }
        Ok(false) => {
            let response = r#"{ "success": false, "message": "Book not found or not updated" }"#;
            ("HTTP/1.1 404 Not Found", response.to_string())
        }
        Err(_) => {
            let response = r#"{ "success": false, "message": "Database error during update" }"#;
            ("HTTP/1.1 500 Internal Server Error", response.to_string())
        }
    }
}

pub fn handle_delete_book(id: i64, db: &Database) -> (&'static str, String) {
    match db.delete_book(id) {
        Ok(true) => (
            "HTTP/1.1 200 OK",
            r#"{ "success": true, "message": "Book deleted successfully" }"#.to_string(),
        ),
        Ok(false) => (
            "HTTP/1.1 404 Not Found",
            r#"{ "success": false, "message": "Book not found" }"#.to_string(),
        ),
        Err(_) => (
            "HTTP/1.1 500 Internal Server Error",
            r#"{ "success": false, "message": "Database error" }"#.to_string(),
        ),
    }
}

pub fn handle_fetch_users(db: &Database) -> (&'static str, String) {
    match db.fetch_users() {
        Ok(users) => {
            let json = serde_json::to_string(&users).unwrap_or("[]".to_string());
            ("HTTP/1.1 200 OK", json)
        }
        Err(_) => {
            let error = r#"{"success": false, "message": "Could not fetch books"}"#;
            ("HTTP/1.1 500 Internal Server Error", error.to_string())
        }
    }
}

pub fn handle_borrow_book(request_body: &str, db: &Database) -> (&'static str, String) {
    let parsed: serde_json::Value = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(_) => {
            let response = r#"{"success": false, "message": "Invalid JSON"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    let user_id = match parsed.get("user_id").and_then(|v| v.as_i64()) {
        Some(id) => id,
        None => {
            let response = r#"{"success": false, "message": "Missing or invalid user_id"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };

    let book_id = match parsed.get("book_id").and_then(|v| v.as_i64()) {
        Some(id) => id,
        None => {
            let response = r#"{"success": false, "message": "Missing or invalid book_id"}"#;
            return ("HTTP/1.1 400 Bad Request", response.to_string());
        }
    };
    match db.borrow_book(user_id, book_id) {
        Ok(true) => {
            let response = r#"{"success": true, "message": "Book borrowed successfully"}"#;
            ("HTTP/1.1 201 Created", response.to_string())
        }
        Ok(false) => {
            let response = r#"{"success": false, "message": "Book or User doesn't exist"}"#;
            ("HTTP/1.1 409 Conflict", response.to_string())
        }
        Err(_) => {
            let response = r#"{"success": false, "message": "Database error occurred"}"#;
            ("HTTP/1.1 500 Internal Server Error", response.to_string())
        }
    }
}

pub fn handle_fetch_borrowed_books(user_id: i64, db: &Database) -> (&'static str, String) {
    match db.fetch_borrowed_books(user_id) {
        Ok(books) => {
            let json = serde_json::to_string(&books).unwrap_or("[]".to_string());
            return ("HTTP/1.1 200 OK", json);
        }
        Err(_) => {
            let error = r#"{"success": false, "message": "Could not fetch borrowed books"}"#;
            ("HTTP/1.1 500 Internal Server Error", error.to_string())
        }
    }
}

pub fn handle_return_book(borrowed_id: i64, book_id: i64, db: &Database) -> (&'static str, String) {
    match db.return_book(borrowed_id, book_id) {
        Ok(true) => (
            "HTTP/1.1 200 OK",
            r#"{"success": true, "message":"Book returned successfully"}"#.to_string(),
        ),
        Ok(false) => (
            "HTTP/1.1 404 NOT FOUND",
            r#"{"success": false, "message": "Borrow details not found"}"#.to_string(),
        ),
        Err(_) => (
            "HTTP/1.1 500 Internal Server Error",
            r#"{"success": false, "message": "Database error"}"#.to_string(),
        ),
    }
}

pub fn handle_fetch_all_borrowed_books(db: &Database) -> (&'static str, String) {
    match db.fetch_all_borrowed_books() {
        Ok(books) => {
            let json = serde_json::to_string(&books).unwrap_or("[]".to_string());
            return ("HTTP/1.1 200 OK", json);
        }
        Err(_) => {
            let error = r#"{"success": false, "message": "Could not fetch borrowed books"}"#;
            ("HTTP/1.1 500 Internal Server Error", error.to_string())
        }
    }
}
