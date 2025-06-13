use rusqlite::params;
use rusqlite::{Connection, Result as SqliteResult};
use serde::Serialize;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

#[derive(Debug, Serialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub publication_year: String,
    pub genre: String,
    pub number_of_copies: i32,
    pub available: bool,
}

impl Database {
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;

        // Create users table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                role TEXT DEFAULT 'user',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        //create books table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            isbn TEXT UNIQUE NOT NULL,
            publication_year TEXT NOT NULL,
            genre TEXT NOT NULL,
            number_of_copies INTEGER NOT NULL DEFAULT 1,
            available BOOLEAN NOT NULL DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
            [],
        )?;

        Ok(Database {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn create_user(&self, username: &str, password: &str) -> SqliteResult<bool> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1")?;
        let count: i32 = stmt.query_row([username], |row| row.get(0))?;

        if count > 0 {
            return Ok(false);
        }

        //hash the password later
        conn.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            [username, password],
        )?;

        Ok(true)
    }

    pub fn verify_user(
        &self,
        username: &str,
        password: &str,
    ) -> SqliteResult<Option<(String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT username, password, role FROM users WHERE username = ?1")?;

        match stmt.query_row([username], |row| {
            let stored_username: String = row.get(0)?;
            let stored_password: String = row.get(1)?;
            let role: String = row.get(2)?;
            if stored_password == password {
                Ok(Some((stored_username, role)))
            } else {
                Ok(None)
            }
        }) {
            Ok(result) => Ok(result),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn add_book(
        &self,
        title: &str,
        author: &str,
        isbn: &str,
        publication_year: &str,
        genre: &str,
        number_of_copies: i32,
    ) -> SqliteResult<bool> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM books WHERE isbn = ?1")?;
        let count: i32 = stmt.query_row([isbn], |row| row.get(0))?;

        if count > 0 {
            return Ok(false);
        }

        conn.execute(
            "INSERT INTO books (title, author, isbn, publication_year, genre, number_of_copies) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (title, author, isbn, publication_year, genre, number_of_copies),
        )?;

        Ok(true)
    }

    pub fn fetch_books(&self) -> Result<Vec<Book>, String> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = match conn.prepare(
            "SELECT id, title, author, isbn, publication_year, genre, number_of_copies, available FROM books",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Err("Failed to prepare statement".to_string()),
        };

        let rows = match stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                isbn: row.get(3)?,
                publication_year: row.get(4)?,
                genre: row.get(5)?,
                number_of_copies: row.get(6)?,
                available: row.get(7)?,
            })
        }) {
            Ok(rows) => rows,
            Err(_) => return Err("Failed to query books".to_string()),
        };

        let mut books = Vec::new();
        for book_result in rows {
            match book_result {
                Ok(book) => books.push(book),
                Err(_) => return Err("Error reading book row".to_string()),
            }
        }

        Ok(books)
    }

    pub fn fetch_book(&self, book_id: i64) -> Result<Option<Book>, String> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = match conn.prepare(
            "SELECT id, title, author, isbn, publication_year, genre, number_of_copies, available FROM books WHERE id = ?1",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Err("Failed to prepare statement".to_string()),
        };

        let rows = match stmt.query_map([book_id], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                isbn: row.get(3)?,
                publication_year: row.get(4)?,
                genre: row.get(5)?,
                number_of_copies: row.get(6)?,
                available: row.get(7)?,
            })
        }) {
            Ok(rows) => rows,
            Err(_) => return Err("Failed to query books".to_string()),
        };

        for book_result in rows {
            match book_result {
                Ok(book) => return Ok(Some(book)),
                Err(_) => return Err("Error reading book row".to_string()),
            }
        }

        Ok(None)
    }

    pub fn edit_book(&self, book_id: i64, updated_fields: &Value) -> SqliteResult<bool> {
        let mut query = String::from("UPDATE books SET ");
        let mut sets = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(title) = updated_fields.get("title").and_then(|v| v.as_str()) {
            sets.push("title = ?");
            values.push(Box::new(title.to_string()));
        }
        if let Some(author) = updated_fields.get("author").and_then(|v| v.as_str()) {
            sets.push("author = ?");
            values.push(Box::new(author.to_string()));
        }
        if let Some(isbn) = updated_fields.get("isbn").and_then(|v| v.as_str()) {
            sets.push("isbn = ?");
            values.push(Box::new(isbn.to_string()));
        }
        if let Some(year) = updated_fields
            .get("publication_year")
            .and_then(|v| v.as_str())
        {
            sets.push("publication_year = ?");
            values.push(Box::new(year.to_string()));
        }
        if let Some(genre) = updated_fields.get("genre").and_then(|v| v.as_str()) {
            sets.push("genre = ?");
            values.push(Box::new(genre.to_string()));
        }
        if let Some(copies) = updated_fields
            .get("number_of_copies")
            .and_then(|v| v.as_i64())
        {
            sets.push("number_of_copies = ?");
            values.push(Box::new(copies));
        }
        if let Some(available) = updated_fields.get("available").and_then(|v| v.as_bool()) {
            sets.push("available = ?");
            values.push(Box::new(available));
        }

        if sets.is_empty() {
            return Ok(false); // Nothing to update
        }

        query.push_str(&sets.join(", "));
        query.push_str(" WHERE id = ?");
        values.push(Box::new(book_id));

        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(&query)?;
        stmt.execute(rusqlite::params_from_iter(values))?;

        Ok(true)
    }

    pub fn delete_book(&self, book_id: i64) -> SqliteResult<bool> {
        let conn = self.connection.lock().unwrap();

        let affected_row = conn.execute("DELETE FROM books WHERE id=?", params![book_id])?;

        Ok(affected_row > 0)
    }

    /*understand this bit of code */
    pub fn fetch_users(&self) -> Result<Vec<Value>, String> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = match conn.prepare("SELECT username FROM users WHERE role='user'") {
            Ok(stmt) => stmt,
            Err(_) => return Err("Failed to prepare statement".to_string()),
        };

        let users_iter = match stmt.query_map([], |row| {
            let username: String = row.get(0)?;
            Ok(json!({ "username": username }))
        }) {
            Ok(rows) => rows,
            Err(_) => return Err("Failed to query users".to_string()),
        };

        let mut users = Vec::new();
        for result in users_iter {
            match result {
                Ok(user) => users.push(user),
                Err(_) => return Err("Error reading user row".to_string()),
            }
        }

        Ok(users)
    }
}
