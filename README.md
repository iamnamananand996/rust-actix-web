# Rust Actix-Web CRUD Application

A modern, secure REST API built with Rust using Actix-web framework, SeaORM for database operations, and JWT authentication.

## 🚀 Features

- **User Authentication**: JWT-based authentication with registration and login endpoints
- **User Management**: CRUD operations for user data
- **Database Integration**: PostgreSQL integration using SeaORM
- **Secure Password Hashing**: SHA-256 password hashing
- **API Response Standardization**: Consistent JSON response structure
- **Middleware Support**: Authentication middleware for protected routes
- **Database Migrations**: Automated database schema management
- **Environment Configuration**: Configurable settings via environment variables
- **Request Logging**: Built-in request logging middleware

## 🛠️ Tech Stack

- **Framework**: [Actix-web](https://actix.rs/) - High-performance web framework
- **ORM**: [SeaORM](https://www.sea-ql.org/SeaORM/) - Async ORM for Rust
- **Database**: PostgreSQL
- **Authentication**: JWT (JSON Web Tokens)
- **Password Hashing**: SHA-256
- **Serialization**: Serde
- **Environment**: dotenv

## 📋 Prerequisites

- Rust 1.70+ (2024 edition)
- PostgreSQL 12+
- Cargo

## 🔧 Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-actix-web
   ```

2. **Create environment file**
   ```bash
   cp .env.example .env
   ```

3. **Configure environment variables**
   ```env
   ADDRESS=127.0.0.1
   PORT=8080
   DATABASE_URL=postgresql://username:password@localhost/database_name
   JWT_SECRET=your-secret-key-here
   ```

4. **Install dependencies**
   ```bash
   cargo build
   ```

5. **Run database migrations**
   ```bash
   cargo run --bin migration
   ```

6. **Start the server**
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8080`

## 📚 API Documentation

### Authentication Endpoints

#### Register User
```http
POST /auth/register
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "User created successfully: 1",
  "data": {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Login User
```http
POST /auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Login successful",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

### Protected Endpoints

All endpoints under `/home` require authentication via Bearer token.

#### Get Personalized Greeting
```http
GET /home/{name}
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "Hello, John!",
  "data": "John"
}
```

#### List All Users
```http
GET /home/users/list
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "Found users 5",
  "data": [
    {
      "id": 1,
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

## 🔐 Authentication

The API uses JWT (JSON Web Tokens) for authentication. After successful login, include the token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

Tokens expire after 24 hours and contain:
- User ID
- User email
- Issued at timestamp
- Expiration timestamp

## 📁 Project Structure

```
rust-actix-web/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── routes/
│   │   ├── auth_routes.rs      # Authentication route definitions
│   │   ├── home_routes.rs      # Home route definitions
│   │   ├── handlers/
│   │   │   ├── auth_handler.rs # Authentication logic
│   │   │   └── home_handler.rs # Home endpoint logic
│   │   └── middlewares/
│   │       └── auth_middlewares.rs # JWT authentication middleware
│   └── utils/
│       ├── api_response.rs     # Standardized API responses
│       ├── app_state.rs        # Application state management
│       ├── constants.rs        # Environment configuration
│       └── jwt.rs              # JWT token utilities
├── entity/
│   └── src/
│       └── user.rs             # User entity model
├── migration/
│   └── src/
│       └── m20250703_135737_create_user_table.rs # Database migration
├── Cargo.toml                  # Project dependencies
└── README.md
```

## 🛡️ Security Features

- **Password Hashing**: Passwords are hashed using SHA-256 before storage
- **JWT Authentication**: Secure token-based authentication
- **Input Validation**: Request body validation using Serde
- **SQL Injection Prevention**: SeaORM provides protection against SQL injection
- **Environment Variables**: Sensitive data stored in environment variables

## 🧪 Testing

Run tests with:
```bash
cargo test
```

## 📈 Development

### Adding New Routes

1. Create handler functions in `src/routes/handlers/`
2. Define route configuration in `src/routes/`
3. Register routes in `src/main.rs`

### Database Changes

1. Create new migration in `migration/src/`
2. Update entity models in `entity/src/`
3. Run migrations with `cargo run --bin migration`

### Environment Variables

Required environment variables:
- `ADDRESS`: Server bind address
- `PORT`: Server port
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT signing

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Actix-web](https://actix.rs/) for the excellent web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) for the powerful ORM
- [Serde](https://serde.rs/) for serialization support 