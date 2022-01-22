fn main() {
    // Required to recompile if migrations have changed (but code hasn't)
    // otherwise Cargo won't recompile
    // SOURCE: https://docs.rs/sqlx/0.5.10/sqlx/macro.migrate.html
    println!("cargo:rerun-if-changed=src/storage/migrations-sqlite");
}
