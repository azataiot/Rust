# SQLx 

## .env 

```ini
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
```

## SQLx-cli 

### Installation

Install SQLx-cli with support for PostgreSQL, SQLite, and native TLS:

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres,sqlite
```

### Usage

Create the database at DATABASE_URL:

```bash
sqlx database create
```

Create migration files:

```bash
sqlx migrate add -r book_table_init
```

```bash
sqlx migrate add -r book_table_init
Creating migrations/20250831082237_book_table_init.up.sql
Creating migrations/20250831082237_book_table_init.down.sql
```


Run migrations:

```bash
sqlx migrate run
```