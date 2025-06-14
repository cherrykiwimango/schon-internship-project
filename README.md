# Library Management System

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![HTML5](https://img.shields.io/badge/html5-%23E34F26.svg?style=for-the-badge&logo=html5&logoColor=white)
![CSS3](https://img.shields.io/badge/css3-%231572B6.svg?style=for-the-badge&logo=css3&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)
![JWT](https://img.shields.io/badge/JWT-black?style=for-the-badge&logo=JSON%20web%20tokens)

A full-stack library management system built with Rust backend and vanilla HTML/CSS/JavaScript frontend. Features user authentication, role-based access control, and complete book management functionality.


## 🚀 Features

### Authentication & Security
- User registration and login system
- Password hashing for secure storage
- JWT-based authentication for protected routes
- Role-based access control (Admin/User)

### Admin Features
- **Book Management**: Complete CRUD operations for books
- **User Management**: View all registered users
- **Borrow Tracking**: Monitor all borrowed books and due dates

### User Features
- **Book Browsing**: View available books in the library
- **Book Borrowing**: Borrow available books (7-day loan period)
- **Book Returning**: Return borrowed books
- **Personal Library**: View personal borrowing history

### Technical Features
- **Custom Multithreaded Server**: Built from scratch using Rust following The Rust Book
- **Thread Pool**: 4-worker thread pool for concurrent request handling
- **SQLite Database**: Lightweight database with three main tables (users, books, borrowed)
- **RESTful API**: Clean API design with proper HTTP methods
- **No Framework Dependencies**: Pure implementation without external web frameworks

## 🛠️ Technology Stack

- **Backend**: Rust (Custom TCP Server)
- **Frontend**: HTML, CSS, JavaScript (Vanilla)
- **Database**: SQLite
- **Authentication**: JWT tokens
- **Architecture**: Multithreaded server with thread pool

## 📋 Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone <your-repository-url>
cd library-management-system
```
### 2. Run the Application
```bash
cargo run
```
The server will start on http://localhost:7878
### 3. Access the Application

- **Login Page**: http://localhost:7878/login.html
- **Dashboard**: http://localhost:7878/dashboard.html (after login)
- **Admin Dashboard**: http://localhost:7878/admin_dashboard.html (after login)


- Note: The application automatically creates the SQLite database (project.db) on first run.

## 📚 API Documentation
### Authentication Endpoints
#### User Registration
```bash
POST /api/signup
Content-Type: application/json

{
  "username": "john_doe",
  "password": "secure_password",
}
```
#### Response:
```bash
{
  "success": true,
  "message": "User username_here created successfully",
}
```
#### User Login
```bash
POST /api/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "secure_password"
}
```
#### Response:
```bash
{
  "success": true,
  "message": "Login successful",
  "userId": 5,
  "username":"Johnny Depp",
  "role":"user",
  "jwt": "jwt_token_here",
}
```
### User Management Endpoints
#### Get All Users (Admin Only)
```bash
GET /api/users
```
#### Response:
```bash
{
  "success": true,
  "users": [
    {
      "username": "john_doe",
    }
  ]
}
```
### Book Management Endpoints
#### Get All Books
```bash
GET /api/books
```
#### Response:
```bash
{
  "success": true,
  "books": [
    {
      "id": 1,
      "title": "The Rust Programming Language",
      "author": "Steve Klabnik",
      "isbn": "9781593278281",
      "publication_year":"2003"
      "genre":"Horror"
      "number_of_copies": 3,
    }
  ]
}
```
#### Get Single Book
```bash
GET /api/books/{book_id}
```
#### Add New Book (Admin Only)
```bash
POST /api/books
Content-Type: application/json

{
  "title": "Clean Code",
  "author": "Robert C. Martin",
  "isbn": "9780132350884",
  "publication_year":"2005",
  "genre":"Self-Help",
  "number_of_copies":3,
}
```
#### Update Book (Admin Only)
```bash
PATCH /api/books/{book_id}
Content-Type: application/json

{
  "title": "Clean Code - Updated",
  "number_of_copies": 1
}
```
#### Delete Book (Admin Only)
```bash
DELETE /api/books/{book_id}
```
### Borrowing Endpoints
#### Borrow Book
```bash
POST /api/borrow
Content-Type: application/json

{
  "book_id": 1,
  "user_id": 1
}
```
#### Response:
```bash
{
  "success": true,
  "message": "Book borrowed successfully",
}
```
#### Get User's Borrowed Books
```bash
GET /api/borrow/{user_id}
```
#### Get All Borrowed Books (Admin Only)
```bash
GET /api/borrow
```
#### Return Book
```bash
DELETE /api/borrow/{borrow_id}/{book_id}
```
## 🗄️ Database Schema
### Users Table
```bash
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
### Books Table
```bash
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    isbn TEXT UNIQUE,
    total_copies INTEGER NOT NULL DEFAULT 1,
    available_copies INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
### Borrowed Table
```bash
CREATE TABLE borrowed (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    borrowed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    due_date DATETIME NOT NULL,
    returned_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (book_id) REFERENCES books (id)
);
```
## 🏗️ Project Structure
```bash
├── src/
│   ├── main.rs              # Main server implementation with ThreadPool
│   ├── handlers.rs          # API request handlers
│   ├── db.rs               # Database operations
│   └── lib.rs              # ThreadPool implementation
├── frontend/                 # Frontend files
│   ├── login.html
│   ├── dashboard.html 
│   ├── admin_dashboard.html ..and so on
│   ├── js/                 # Javascript files
│       ├── auth.js ..and so on
│   └── styles.css
├── Cargo.toml             # Rust dependencies
└── README.md              # This file
```
## 🔧 Architecture Details
### Custom Multithreaded Server
-The application implements a custom TCP server following **The Rust Book's final project**:

- **ThreadPool**: Manages 4 worker threads for handling concurrent requests
- **Connection Handling**: Each HTTP request is processed in a separate thread
- **Database Sharing**: Thread-safe database operations using Arc and Mutex
- **Job Queue**: Uses mpsc channels for distributing work among threads

### Security Features

- **Password Hashing**: User passwords are securely hashed before storage
- **JWT Authentication**: Stateless authentication using JSON Web Tokens
- **Role-based Access**: Different permissions for admin and regular users
- **Input Validation**: Server-side validation for all API endpoints

## 🚦 HTTP Status Codes

- **200 OK** - Successful request
- **201 Created** - Resource created successfully
- **400 Bad Request** - Invalid request format or parameters
- **401 Unauthorized** - Missing or invalid authentication token
- **403 Forbidden** - Insufficient permissions
- **404 Not Found** - Resource not found
- **500 Internal Server Error** - Server-side error

## 📝 Development Notes

- The server binds to 127.0.0.1:7878 and automatically creates the SQLite database on first run
- Create admin users by setting "role": "admin" in by modifying the users table
- Book loan period is fixed at 7 days from the borrow date
- Books become unavailable when all copies are borrowed (number_of_copies = 0)

## 🎯 Key Implementation Highlights

- No External Web Frameworks: Built entirely with Rust standard library
- Custom HTTP Parser: Manually parsing HTTP requests and constructing responses
- Thread Safety: Proper use of Arc<Mutex<>> for sharing database connections
- Error Handling: Comprehensive error handling throughout the application
- Clean Architecture: Separation of concerns with dedicated modules for database, handlers, and server logic

## 📄 License
- This project is created for educational and assessment purposes.

Note: This implementation demonstrates core concepts of web development, database management, concurrent programming, and system architecture using Rust without external web frameworks.
