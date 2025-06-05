# Rust Webapp

[![version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/EthanAndreas/Rust-Webapp)
[![author](https://img.shields.io/badge/author-EthanAndreas-blue)](https://github.com/EthanAndreas)

## Table of Contents
- [Description](#description)
- [Architecture](#architecture)
- [Technologies Used](#technologies-used)
  - [Backend (Actix-web)](#backend-actix-web)
  - [Frontend (Yew)](#frontend-yew)
- [Features](#features)
  - [Backend API Endpoints](#backend-api-endpoints)
  - [Frontend Pages](#frontend-pages)
- [Setup](#setup)
- [Run](#run)
- [Testing](#testing)
- [API Documentation](#api-documentation)
  - [Authentication](#authentication)
  - [Protected Routes](#protected-routes)
  - [Database Schema](#database-schema)
- [Future Improvements](#future-improvements)

## Description

This project is a simple web application built using Rust.
It contains a backend server using Actix-web and a frontend using Yew.
The backend includes user authentication, JWT token generation, and a simple API for managing users with a PostgreSQL database.
The frontend is a single-page application (SPA) that communicates with the backend API.

## Architecture

```
RustWebapp/
├── backend/         # Rust API with Actix-web
│   └── src/
│       ├── handlers/   # Route handlers
│       ├── models/     # Data models
│       └── db/         # Database configuration
└── frontend/        # User interface with Yew
    └── src/
        ├── components/ # Reusable components
        └── pages/      # Application pages
```

## Technologies Used

### Backend (Actix-web)
- Actix-web: High-performance web framework
- SQLx: Async PostgreSQL ORM
- CORS: For frontend requests
- JWT (to be implemented): For authentication
### Frontend (Yew)
- Yew: Rust framework for building web applications
- Trunk: WASM web application bundler
- wasm-bindgen: WebAssembly binding generator
- Bulma: CSS framework for styling

## Features

### Backend API Endpoints
- POST /login: User authentication
- POST /register: User registration
- GET /profile: Get user profile (protected route)
### Frontend Pages
- Login page
- Registration page
- Profile page
- Home page

## Setup

- Install dependencies for both backend and frontend
```bash
./setup.sh
```

## Run

- Start both backend and frontend servers:
```bash
./start.sh
```

This will:
- Start the backend server on http://localhost:8080
- Start the frontend server on http://localhost:3000

## Testing

- Test the API functions:
```bash
./test/api.sh
```

This will run a series of tests including:
- Database connectivity
- User registration
- User authentication
- Protected routes

## API Documentation

### Authentication
```bash
# Login
curl -X POST -H "Content-Type: application/json" \
  -d '{"username":"test","password":"test"}' \
  http://localhost:8080/login

# Register
curl -X POST -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"test"}' \
  http://localhost:8080/register
ⓘ
For code that is intended to be used in Siemens products or services, the code generation features of our AI Services may only be used after prior approval of your responsible organizational unit.
```

### Protected Routes
```bash
# Get Profile
curl -H "Authorization: Bearer <token>" \
  http://localhost:8080/profile
```

### Database Schema
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Future Improvements
#### Backend:
- Implement JWT authentication
- Add input validation
- Add custom middleware
- Add integration tests
#### Frontend:
- Add global state management
- Add more components
- Add unit tests
- Add custom styling