# Todo API - Production-Ready Rust REST API

A modern, secure, and scalable Todo REST API built with Rust using Axum framework, featuring JWT authentication, PostgreSQL integration, and comprehensive error handling.

## 🚀 Features

- **RESTful API** with full CRUD operations for todos
- **JWT Authentication** with secure token-based authorization
- **PostgreSQL Integration** with SQLx for type-safe database operations
- **Input Validation** using the validator crate
- **Comprehensive Error Handling** with custom error types
- **CORS Support** for cross-origin requests
- **Structured Logging** with tracing for observability
- **Database Migrations** with automatic schema management
- **Production-Ready** configuration management

## 🛠 Technology Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- **Authentication**: JWT tokens with [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- **Password Hashing**: [bcrypt](https://github.com/Stebalien/bcrypt) for secure password storage
- **Validation**: [validator](https://github.com/Keats/validator) for request validation
- **Logging**: [tracing](https://github.com/tokio-rs/tracing) for structured logging
- **Environment**: [dotenvy](https://github.com/allan2/dotenvy) for configuration management
- **CORS**: [tower-http](https://github.com/tower-rs/tower-http) for HTTP middleware

## 📋 Prerequisites

Before running this application, ensure you have:

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **PostgreSQL** (12+ recommended) - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Git** - [Install Git](https://git-scm.com/downloads)

## ⚙️ Setup Instructions

### 1. Clone the Repository

```bash
git clone <repository-url>
cd todo-api
```

### 2. Database Setup

Create a PostgreSQL database:

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE todo_db;

# Create user (optional)
CREATE USER todo_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE todo_db TO todo_user;
```

### 3. Environment Configuration

Create a `.env` file in the project root:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
# Database Configuration
DATABASE_URL=postgresql://username:password@localhost:5432/todo_db

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION_HOURS=24

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Logging Level (optional)
RUST_LOG=todo_api=debug,tower_http=debug
```

### 4. Build and Run

```bash
# Install dependencies and build
cargo build --release

# Run database migrations (automatic on startup)
# The application will create tables automatically

# Start the server
cargo run --release
```

The server will start at `http://127.0.0.1:3000`

## 📚 API Documentation

### Base URL
```
http://localhost:3000
```

### Authentication
Most endpoints require JWT authentication. Include the token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

### Endpoints Overview

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/health` | Health check | No |
| POST | `/auth/register` | User registration | No |
| POST | `/auth/login` | User login | No |
| GET | `/todos` | Get user's todos | Yes |
| POST | `/todos` | Create new todo | Yes |
| PUT | `/todos/:id` | Update todo | Yes |
| DELETE | `/todos/:id` | Delete todo | Yes |

### Detailed Endpoint Documentation

#### Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### User Registration
```http
POST /auth/register
Content-Type: application/json

{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Response (201 Created):**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  }
}
```

#### User Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  }
}
```

#### Get Todos
```http
GET /todos
Authorization: Bearer <token>
```

**Response (200 OK):**
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Buy groceries",
    "description": "Milk, eggs, bread",
    "completed": false,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  }
]
```

#### Create Todo
```http
POST /todos
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Learn Rust",
  "description": "Complete the Rust programming tutorial"
}
```

**Response (201 Created):**
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "title": "Learn Rust",
  "description": "Complete the Rust programming tutorial",
  "completed": false,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

#### Update Todo
```http
PUT /todos/456e7890-e89b-12d3-a456-426614174001
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Learn Advanced Rust",
  "completed": true
}
```

**Response (200 OK):**
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "title": "Learn Advanced Rust",
  "description": "Complete the Rust programming tutorial",
  "completed": true,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T11:30:00Z"
}
```

#### Delete Todo
```http
DELETE /todos/456e7890-e89b-12d3-a456-426614174001
Authorization: Bearer <token>
```

**Response (204 No Content)**

### Error Responses

All errors follow a consistent format:

```json
{
  "error": "Error description",
  "status": 400
}
```

**Common HTTP Status Codes:**
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (authentication required/failed)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found (resource doesn't exist)
- `409` - Conflict (user already exists)
- `500` - Internal Server Error

### cURL Examples

#### Register a new user:
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

#### Login:
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

#### Create a todo (replace TOKEN with actual token):
```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer TOKEN" \
  -d '{
    "title": "My first todo",
    "description": "This is a test todo"
  }'
```

#### Get todos:
```bash
curl -X GET http://localhost:3000/todos \
  -H "Authorization: Bearer TOKEN"
```

## 🗄️ Database Schema

### Entity Relationship Diagram

```
┌─────────────────┐       ┌─────────────────┐
│      users      │       │      todos      │
├─────────────────┤       ├─────────────────┤
│ id (UUID) PK    │◄─────┐│ id (UUID) PK    │
│ username        │      ││ user_id (UUID)  │
│ email           │      │└─ FK to users.id │
│ password_hash   │      │  title           │
│ created_at      │      │  description     │
│ updated_at      │      │  completed       │
└─────────────────┘      │  created_at      │
                         │  updated_at      │
                         └─────────────────┘
```

### Table Definitions

#### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

#### Todos Table
```sql
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Indexes
- `idx_users_email` - Fast user lookup by email
- `idx_users_username` - Fast user lookup by username
- `idx_todos_user_id` - Fast todo queries by user
- `idx_todos_completed` - Fast filtering by completion status
- `idx_todos_created_at` - Fast sorting by creation date

## 🔐 Security Implementation

### Authentication Flow
1. **User Registration**: Password hashed with bcrypt (cost: 12)
2. **User Login**: Password verified, JWT token issued
3. **Protected Requests**: JWT token validated on each request
4. **Token Expiration**: Configurable expiration (default: 24 hours)

### JWT Token Structure
```json
{
  "sub": "user-uuid",    // Subject (User ID)
  "exp": 1642248000,     // Expiration timestamp
  "iat": 1642161600      // Issued at timestamp
}
```

### Password Security
- **Hashing**: bcrypt with salt rounds (cost factor: 12)
- **Storage**: Only hashed passwords stored, never plaintext
- **Validation**: Minimum 6 character requirement

### Security Best Practices Implemented
- ✅ **No hardcoded secrets** - All sensitive data in environment variables
- ✅ **Input validation** - All user inputs validated
- ✅ **CORS configured** - Cross-origin requests handled securely
- ✅ **Database relationships** - Foreign key constraints with cascading deletes
- ✅ **User isolation** - Users can only access their own todos
- ✅ **Error handling** - No sensitive information leaked in error responses

## 🏗️ Code Architecture

### Project Structure
```
src/
├── main.rs                 # Application entry point and server setup
├── lib.rs                  # Shared types, errors, and app state
├── models/
│   ├── mod.rs             # Module exports
│   ├── user.rs            # User model and request/response types
│   └── todo.rs            # Todo model and request/response types
├── handlers/
│   ├── mod.rs             # Module exports
│   ├── auth.rs            # Authentication handlers (register/login)
│   └── todos.rs           # Todo CRUD handlers
├── middleware/
│   ├── mod.rs             # Module exports
│   └── auth.rs            # JWT authentication middleware
├── utils/
│   ├── mod.rs             # Module exports
│   ├── jwt.rs             # JWT token utilities
│   └── password.rs        # Password hashing utilities
└── database/
    ├── mod.rs             # Database connection and migration utilities
    └── migrations/        # SQL migration files
        ├── 001_create_users_table.sql
        └── 002_create_todos_table.sql
```

### Key Design Decisions

#### 1. **Modular Architecture**
- Separation of concerns with dedicated modules
- Clean boundaries between authentication, business logic, and data access
- Reusable utility functions

#### 2. **Type Safety**
- SQLx for compile-time SQL verification
- Strong typing throughout the application
- Validation at API boundaries

#### 3. **Error Handling Strategy**
- Custom `AppError` enum for all error types
- Consistent error responses across all endpoints
- Proper logging without exposing sensitive information

#### 4. **State Management**
- Single `AppState` struct containing database pool and configuration
- Shared state across all handlers through Axum's dependency injection

#### 5. **Middleware Implementation**
- Authentication middleware using Axum's `FromRequestParts` trait
- Automatic token validation for protected routes

### Configuration Management
- Environment-based configuration with sensible defaults
- Validation of critical configuration values at startup
- Support for development and production environments

## 🧪 Testing & Development

### Running in Development Mode
```bash
# Set development environment
export RUST_LOG=todo_api=debug,tower_http=debug

# Run with cargo watch for auto-reload (install with: cargo install cargo-watch)
cargo watch -x run
```

### Testing the API
```bash
# Health check
curl http://localhost:3000/health

# Test registration
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "email": "test@example.com", "password": "password123"}'
```

### Database Migrations
Migrations run automatically on application startup. To run manually:

```bash
# The application handles migrations automatically
# Migration files are located in src/database/migrations/
```

### Common Troubleshooting

#### Database Connection Issues
- Verify PostgreSQL is running: `pg_ctl status`
- Check connection string in `.env` file
- Ensure database exists and user has proper permissions

#### JWT Token Issues
- Verify `JWT_SECRET` is set in `.env`
- Check token expiration time
- Ensure proper Authorization header format: `Bearer <token>`

#### Build Errors
- Update Rust to latest version: `rustup update`
- Clean build cache: `cargo clean`
- Check all dependencies: `cargo check`

## 🚀 Deployment Considerations

### Production Configuration
```env
# Use strong, unique JWT secret
JWT_SECRET=your-very-long-and-random-secret-key-here

# Use production database URL
DATABASE_URL=postgresql://username:password@prod-db:5432/todo_db

# Bind to all interfaces in production
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Reduce log verbosity
RUST_LOG=todo_api=info,error
```

### Environment Variables Reference
| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `JWT_SECRET` | Secret key for JWT signing | - | Yes |
| `JWT_EXPIRATION_HOURS` | Token expiration in hours | 24 | No |
| `SERVER_HOST` | Server bind address | 127.0.0.1 | No |
| `SERVER_PORT` | Server port | 3000 | No |
| `RUST_LOG` | Logging configuration | - | No |

### Docker Considerations (Future Enhancement)
```dockerfile
# Example Dockerfile structure
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/todo-api /usr/local/bin/
EXPOSE 3000
CMD ["todo-api"]
```

### Production Checklist
- [ ] Use strong JWT secret (64+ characters)
- [ ] Enable HTTPS/TLS termination
- [ ] Set up database connection pooling limits
- [ ] Configure log aggregation
- [ ] Set up health monitoring
- [ ] Enable database backups
- [ ] Configure rate limiting (consider using nginx/cloudflare)
- [ ] Set up CI/CD pipeline

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📞 Support

For support and questions:
- Create an issue in the repository
- Check the troubleshooting section above
- Review the API documentation for proper usage

---

**Built with ❤️ in Rust**