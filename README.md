# Rust Shop API (Actix-web)

A simple **Rust + Actix-web** backend project that implements:
- Authentication with cookies
- Shopping cart (add/remove/view)
- Product catalog

This project demonstrates a clean backend structure with separated models, handlers, and routes.

---

## Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/username/rust-shop-api.git
cd rust-shop-api
```

### 2. Build the project
```bash
cargo build
```

### 3. Run the server
```bash
cargo run
```

The server will be available at:
`http://127.0.0.1:8080`

---

## API Endpoints

### Authentication
- POST `/login` – login (sets `auth_token` cookie)
- GET `/check` – check authentication
- POST `/logout` – logout (removes cookie)

## Cart

- POST `/cart/add` – add a product
```json
{ "id": 1, "name": "Apple", "price": 5.0 }
```
- POST `/cart/remove` – remove a product
```json
{ "id": 1 }
```
- GET `/cart` – view the cart

## Products

- GET `/products` – list available products
