# Justfile

project-name := `basename $(git rev-parse --show-toplevel)`
zip-path := "/tmp/" + project-name + ".zip"

# List all recipes
default:
    @just --list

# Create a zip of the repo excluding .git, target, docs, etc.
zip:
    @echo "Creating archive: {{ zip-path }}"
    git archive --format=zip -o {{ zip-path }} HEAD
    @echo "Archive created at {{ zip-path }}"
    @open /tmp


# Run the axum package
axum:
    cargo run -p rust-axum

# Run the sqlx package
sqlx:
    cargo run -p rust-sqlx

# Run the jsonwebtoken package
jsonwebtoken:
    cargo run -p rust-jsonwebtoken

# Run the tracing package
tracing:
    cargo run -p rust-tracing
