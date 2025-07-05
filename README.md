# Rust Actix-Web Application

A modern, secure REST API built with Rust using Actix-web framework, SeaORM for database operations, and JWT authentication.

## 🚀 Features

- **User Authentication**: JWT-based authentication with registration and login endpoints
- **User Management**: CRUD operations for user data with advanced filtering
- **Post Management**: Full CRUD operations for posts with user association
- **Advanced Search & Filtering**: Search by name/email (users) and title/text (posts)
- **Flexible Sorting**: Sort by creation date, name (users), or title (posts)
- **Date Range Filtering**: Filter records by creation date ranges
- **Pagination**: Efficient pagination for large datasets
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

### User Management Endpoints

All user endpoints require authentication via Bearer token.

#### Get User by ID
```http
GET /user/{id}
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "User found",
  "data": {
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### List All Users with Advanced Features
```http
GET /user/users/list
Authorization: Bearer your-jwt-token
```

**Query Parameters:**
- `page` (optional): Page number for pagination (default: 1)
- `limit` (optional): Items per page (default: 10, max: 100)
- `search` (optional): Search term for name and email fields
- `sort_by` (optional): Sort field - `created_at` or `name` (default: `created_at`)
- `sort_order` (optional): Sort order - `asc` or `desc` (default: `desc`)
- `start_date` (optional): Filter by creation date from (format: YYYY-MM-DD)
- `end_date` (optional): Filter by creation date to (format: YYYY-MM-DD)

**Examples:**

Basic listing:
```http
GET /user/users/list?page=1&limit=10
```

Search users:
```http
GET /user/users/list?search=john
```

Sort by name ascending:
```http
GET /user/users/list?sort_by=name&sort_order=asc
```

Filter by date range:
```http
GET /user/users/list?start_date=2024-01-01&end_date=2024-12-31
```

Combined filters:
```http
GET /user/users/list?search=john&sort_by=created_at&sort_order=desc&start_date=2024-01-01&page=1&limit=20
```

**Response:**
```json
{
  "status": 200,
  "message": "Users found: 5 (page 1 of 1)",
  "data": {
    "users": [
      {
        "id": 1,
        "name": "John Doe",
        "email": "john@example.com",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "avatar": null
      }
    ],
    "pagination": {
      "current_page": 1,
      "per_page": 10,
      "total_items": 5,
      "total_pages": 1
    }
  }
}
```

#### Update User
```http
PUT /user/update/{id}
Authorization: Bearer your-jwt-token
Content-Type: application/json

{
  "name": "John Updated"
}
```

**Response:**
```json
{
  "status": 200,
  "message": "User updated",
  "data": {
    "id": 1,
    "name": "John Updated",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### Post Management Endpoints

All post endpoints require authentication via Bearer token.

#### Get Post by ID
```http
GET /post/{id}
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "Post found",
  "data": {
    "id": 1,
    "user_id": 1,
    "title": "Sample Post",
    "text": "This is a sample post content.",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### List All Posts with Advanced Features
```http
GET /post/posts/list
Authorization: Bearer your-jwt-token
```

**Query Parameters:**
- `page` (optional): Page number for pagination (default: 1)
- `limit` (optional): Items per page (default: 10, max: 100)
- `search` (optional): Search term for title and text fields
- `sort_by` (optional): Sort field - `created_at` or `title` (default: `created_at`)
- `sort_order` (optional): Sort order - `asc` or `desc` (default: `desc`)
- `start_date` (optional): Filter by creation date from (format: YYYY-MM-DD)
- `end_date` (optional): Filter by creation date to (format: YYYY-MM-DD)

**Examples:**

Basic listing:
```http
GET /post/posts/list?page=1&limit=10
```

Search posts:
```http
GET /post/posts/list?search=tutorial
```

Sort by title ascending:
```http
GET /post/posts/list?sort_by=title&sort_order=asc
```

Filter by date range:
```http
GET /post/posts/list?start_date=2024-01-01&end_date=2024-12-31
```

Combined filters:
```http
GET /post/posts/list?search=tutorial&sort_by=created_at&sort_order=desc&start_date=2024-01-01&page=1&limit=20
```

**Response:**
```json
{
  "status": 200,
  "message": "Posts found: 3 (page 1 of 1)",
  "data": {
    "posts": [
      {
        "id": 1,
        "user_id": 1,
        "title": "Sample Post",
        "text": "This is a sample post content.",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "banner": null
      }
    ],
    "pagination": {
      "current_page": 1,
      "per_page": 10,
      "total_items": 3,
      "total_pages": 1
    }
  }
}
```

#### Create Post
```http
POST /post/create
Authorization: Bearer your-jwt-token
Content-Type: application/json

{
  "user_id": "1",
  "title": "New Post",
  "text": "This is my new post content."
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Post created",
  "data": {
    "id": 2,
    "user_id": 1,
    "title": "New Post",
    "text": "This is my new post content.",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Update Post
```http
PUT /post/update/{id}
Authorization: Bearer your-jwt-token
Content-Type: application/json

{
  "title": "Updated Post Title",
  "text": "Updated post content."
}
```

**Response:**
```json
{
  "status": 200,
  "message": "Post updated",
  "data": {
    "id": 1,
    "user_id": 1,
    "title": "Updated Post Title",
    "text": "Updated post content.",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Delete Post
```http
DELETE /post/delete/{id}
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "Post deleted",
  "data": {
    "id": 1,
    "user_id": 1,
    "title": "Deleted Post",
    "text": "This post was deleted.",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### Get My Posts
```http
GET /post/posts/my-posts
Authorization: Bearer your-jwt-token
```

**Response:**
```json
{
  "status": 200,
  "message": "Posts found: 2",
  "data": [
    {
      "id": 1,
      "user_id": 1,
      "title": "My First Post",
      "text": "This is my first post.",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

## 🔍 Advanced Search & Filtering

Both user and post endpoints support advanced search, sorting, and filtering capabilities:

### Query Parameters

| Parameter | Type | Description | Default | Example |
|-----------|------|-------------|---------|---------|
| `page` | number | Page number for pagination | 1 | `?page=2` |
| `limit` | number | Items per page (max: 100) | 10 | `?limit=20` |
| `search` | string | Search term for relevant fields | - | `?search=john` |
| `sort_by` | string | Field to sort by | `created_at` | `?sort_by=name` |
| `sort_order` | string | Sort direction (`asc` or `desc`) | `desc` | `?sort_order=asc` |
| `start_date` | string | Filter from date (YYYY-MM-DD) | - | `?start_date=2024-01-01` |
| `end_date` | string | Filter to date (YYYY-MM-DD) | - | `?end_date=2024-12-31` |

### Search Fields

| Endpoint | Search Fields |
|----------|---------------|
| `/user/users/list` | `name`, `email` |
| `/post/posts/list` | `title`, `text` |

### Sort Fields

| Endpoint | Available Sort Fields |
|----------|----------------------|
| `/user/users/list` | `created_at`, `name` |
| `/post/posts/list` | `created_at`, `title` |

### Examples

```bash
# Search users by name or email
curl -H "Authorization: Bearer <token>" \
  "http://localhost:8080/user/users/list?search=john"

# Get posts sorted by title in ascending order
curl -H "Authorization: Bearer <token>" \
  "http://localhost:8080/post/posts/list?sort_by=title&sort_order=asc"

# Filter posts created in January 2024
curl -H "Authorization: Bearer <token>" \
  "http://localhost:8080/post/posts/list?start_date=2024-01-01&end_date=2024-01-31"

# Complex query: search, sort, filter, and paginate
curl -H "Authorization: Bearer <token>" \
  "http://localhost:8080/user/users/list?search=admin&sort_by=created_at&sort_order=desc&start_date=2024-01-01&page=1&limit=5"
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
│   │   ├── user_routes.rs      # User route definitions
│   │   ├── post_routes.rs      # Post route definitions
│   │   ├── handlers/
│   │   │   ├── auth_handler.rs # Authentication logic
│   │   │   ├── user_handler.rs # User management logic
│   │   │   └── post_handler.rs # Post management logic
│   │   └── middlewares/
│   │       └── auth_middlewares.rs # JWT authentication middleware
│   └── utils/
│       ├── api_response.rs     # Standardized API responses
│       ├── app_state.rs        # Application state management
│       ├── constants.rs        # Environment configuration
│       └── jwt.rs              # JWT token utilities
├── entity/
│   └── src/
│       ├── user.rs             # User entity model
│       └── post.rs             # Post entity model
├── migration/
│   └── src/
│       ├── m20250703_135737_create_user_table.rs # User table migration
│       └── m20220101_000001_create_post_table.rs # Post table migration
├── Cargo.toml                  # Project dependencies
└── README.md
```

## 🛡️ Security Features

- **Password Hashing**: Passwords are hashed using SHA-256 before storage
- **JWT Authentication**: Secure token-based authentication
- **Input Validation**: Request body validation using Serde
- **SQL Injection Prevention**: SeaORM provides protection against SQL injection
- **Environment Variables**: Sensitive data stored in environment variables
- **Route Protection**: Authentication middleware for protected routes

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
