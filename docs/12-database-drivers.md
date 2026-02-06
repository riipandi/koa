# Database Drivers

Koa has **built-in database drivers** for SQLite and PostgreSQL (Phase 1).

## Philosophy

- **Built-in drivers** - No need for external dependencies
- **Simple API** - Unified interface for all databases
- **Type-safe** - Query result mapping to types
- **Async** - Non-blocking database operations

---

## Available Drivers

Phase 1:
- ✅ **SQLite** - Embedded database
- ✅ **PostgreSQL** - Client-server database

Phase 2+ (Future):
- MySQL
- MongoDB
- Redis

---

## Importing Drivers

```typescript
// SQLite driver
import { SqliteConnection, SqliteResult } from "driver/sqlite"

// PostgreSQL driver
import { PostgresConnection, PostgresResult } from "driver/postgres"
```

---

## SQLite

### Connection

```typescript
import { SqliteConnection } from "driver/sqlite"

async fn main(): !void {
    // In-memory database
    let db: SqliteConnection = try SqliteConnection::in_memory()

    // File-based database
    let db: SqliteConnection = try SqliteConnection::open("database.db")

    // ...
}
```

### Queries

```typescript
import { SqliteConnection } from "driver/sqlite"

async fn example(): !void {
    let db: SqliteConnection = try SqliteConnection::open("database.db")

    // Execute SQL
    try db.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)"
    )

    // Insert
    try db.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        ["Alice", "alice@example.com"]
    )

    // Query
    let rows: SqliteResult = try db.query("SELECT * FROM users")

    for row in rows {
        let id: i32 = row.get("id")
        let name: string = row.get("name")
        let email: string = row.get("email")

        println!("User: {} - {}", name, email)
    }
}
```

### Transactions

```typescript
async fn transfer(db: SqliteConnection, from: i32, to: i32, amount: i32): !void {
    // Start transaction
    try db.begin_transaction()

    errdefer {
        // Rollback on error
        try db.rollback()
    }

    // Debit from
    try db.execute(
        "UPDATE accounts SET balance = balance - ? WHERE id = ?",
        [amount, from]
    )

    // Credit to
    try db.execute(
        "UPDATE accounts SET balance = balance + ? WHERE id = ?",
        [amount, to]
    )

    // Commit on success
    try db.commit()
}
```

---

## PostgreSQL

### Connection

```typescript
import { PostgresConnection } from "driver/postgres"

async fn main(): !void {
    // Connection string
    let conn: PostgresConnection = try PostgresConnection::connect(
        "postgresql://user:password@localhost:5432/mydb"
    )

    // ...
}
```

### Queries

```typescript
import { PostgresConnection } from "driver/postgres"

async fn example(): !void {
    let conn: PostgresConnection = try PostgresConnection::connect(
        "postgresql://user:password@localhost:5432/mydb"
    )

    // Create table
    try conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT, email TEXT)"
    )

    // Insert with parameters
    try conn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        ["Alice", "alice@example.com"]
    )

    // Query
    let rows: PostgresResult = try conn.query("SELECT * FROM users")

    for row in rows {
        let id: i32 = row.get("id")
        let name: string = row.get("name")
        let email: string = row.get("email")

        println!("User: {} - {}", name, email)
    }
}
```

### Prepared Statements

```typescript
async fn prepared_statements(conn: PostgresConnection): !void {
    // Prepare statement
    let stmt: PreparedStatement = try conn.prepare(
        "SELECT * FROM users WHERE id = $1"
    )

    // Execute multiple times
    for id in 1..100 {
        let rows: PostgresResult = try stmt.query([id])
        // process rows...
    }

    // Close statement
    try stmt.close()
}
```

### Transactions

```typescript
async fn transfer(conn: PostgresConnection, from: i32, to: i32, amount: i32): !void {
    // Start transaction
    try conn.begin()

    errdefer {
        // Rollback on error
        try conn.rollback()
    }

    // Debit
    try conn.execute(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        [amount, from]
    )

    // Credit
    try conn.execute(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        [amount, to]
    )

    // Commit
    try conn.commit()
}
```

---

## Unified Database Interface

### Generic Database Trait

```typescript
pub interface Database {
    fn execute(sql: string, params: []any): !void
    fn query(sql: string, params: []any): !Result
    fn begin(): !void
    fn commit(): !void
    fn rollback(): !void
}
```

### Usage

```typescript
async fn process_users<D: Database>(db: D): !void {
    // Works with SQLite or Postgres
    try db.execute("CREATE TABLE users ...")
    let rows: !Result = db.query("SELECT * FROM users")
    // ...
}
```

---

## Connection Pooling

### SQLite Pool

```typescript
import { SqlitePool } from "driver/sqlite"

async fn example(): !void {
    // Create pool
    let pool: SqlitePool = try SqlitePool::new("database.db", 5)  // 5 connections

    // Get connection
    let conn: SqliteConnection = try pool.acquire()
    defer pool.release(conn)

    // Use connection
    try conn.execute("INSERT INTO users ...")
}
```

### Postgres Pool

```typescript
import { PostgresPool } from "driver/postgres"

async fn example(): !void {
    // Create pool
    let pool: PostgresPool = try PostgresPool::new(
        "postgresql://user:pass@localhost/mydb",
        10  // 10 connections
    )

    // Get connection
    let conn: PostgresConnection = try pool.acquire()
    defer pool.release(conn)

    // Use connection
    try conn.execute("INSERT INTO users ...")
}
```

---

## ORM Integration (Phase 2)

Future: Built-in ORM for type-safe queries

```typescript
// Future feature
#[derive(Table)]
struct User {
    #[primary_key]
    id: i32,
    name: string,
    email: string,
}

async fn get_user(id: i32): !User {
    // ORM query
    let user: User = try db.find_by_id::<User>(id)
    user
}
```

---

## Migration System (Phase 2)

Future: Built-in migration system

```bash
# Create migration
koa migration create create_users_table

# Run migrations
koa migration up

# Rollback
koa migration down
```

---

## Error Handling

### Database Errors

```typescript
const DbError = error {
    ConnectionFailed,
    QueryFailed,
    TransactionFailed,
    ConstraintViolation,
}
```

### Example

```typescript
async fn safe_query(db: SqliteConnection): !void {
    match db.query("SELECT * FROM users") {
        Ok(rows) => {
            for row in rows {
                println!("{}", row)
            }
        },
        Err(error.ConnectionFailed) => {
            println!("Cannot connect to database")
            return error.ConnectionFailed
        },
        Err(err) => {
            println!("Error: {}", err)
            return err
        },
    }
}
```

---

## Best Practices

### 1. Use Connection Pools

```typescript
// GOOD: Use pool
let pool: SqlitePool = try SqlitePool::new("db.db", 5)
let conn: SqliteConnection = try pool.acquire()

// BAD: Single connection for concurrent access
let conn: SqliteConnection = try SqliteConnection::open("db.db")
```

### 2. Always Handle Transactions

```typescript
// GOOD: Proper transaction handling
try db.begin()
errdefer { try db.rollback() }
try db.commit()

// BAD: No error handling
db.begin()  // Error ignored
```

### 3. Use Prepared Statements

```typescript
// GOOD: Prepared statement
let stmt: PreparedStatement = try conn.prepare("SELECT * FROM users WHERE id = $1")

// BAD: Execute every time (slower)
for id in ids {
    conn.query("SELECT * FROM users WHERE id = $1", [id])
}
```

---

## Comparison with Other Languages

| Feature        | Rust (Diesel)  | Go (sqlx)      | Koa               |
|----------------|----------------|----------------|-------------------|
| **Built-in**   | External crate | External crate | ✅ Built-in        |
| **Type-safe**  | ✅ Compile-time | Runtime checks | ✅ Type-safe       |
| **Async**      | External       | ✅ Async        | ✅ Async           |
| **Migrations** | Diesel CLI     | External       | Planned (Phase 2) |
| **ORM**        | ✅ Diesel       | External       | Planned (Phase 2) |

---

## Next Steps

- [Implementation Plan](10-implementation-plan.md) - Database drivers in Phase 6
- [Standard Library](09-standard-library.md) - Stdlib modules

---

## Implementation Notes

### Driver Location

```
src/driver/
├── mod.koa             # Driver exports
├── sqlite/
│   ├── connection.koa  # SQLite connection
│   ├── result.koa      # Query results
│   └── pool.koa        # Connection pool
└── postgres/
    ├── connection.koa  # Postgres connection
    ├── result.koa      # Query results
    └── pool.koa        # Connection pool
```

### FFI Integration

- SQLite: Link to `libsqlite3`
- PostgreSQL: Link to `libpq`
- Async I/O: Non-blocking calls

---

## Examples

### Web Service with Database

```typescript
import { PostgresPool } from "driver/postgres"
import { http_get, http_post } from "std/net/http"

async fn main(): !void {
    let pool: PostgresPool = try PostgresPool::new(
        "postgresql://user:pass@localhost/mydb",
        10
    )

    async fn handle_get_user(req: HttpRequest): !HttpResponse {
        let conn: PostgresConnection = try pool.acquire()
        defer pool.release(conn)

        let id: i32 = try parse_i32(req.param("id"))
        let rows: PostgresResult = try conn.query(
            "SELECT * FROM users WHERE id = $1",
            [id]
        )

        match rows.next() {
            Option::Some(row) => {
                let user: User = parse_user(row)
                HttpResponse::json(user)
            },
            Option::None => HttpResponse::not_found(),
        }
    }

    let server: Server = Server::bind("0.0.0.0:8080")
    server.run(handle_get_user).await
}
```
