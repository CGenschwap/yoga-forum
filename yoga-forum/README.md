# Yoga-Forum (Backend)

This crate holds the code for the YOGA API backend. This is the primary development-point for the system, though will likely be broken up into smaller sub-crates as the project grows.

# Architecture (YOGA-forum Backend)

The code-level architecture of the YOGA backend attempts to be as simple and straightforward as possible.
```
main.rs
`-- lib.rs
     |-- server.rs        // This should be your first stop. Shows how everything comes together.
     |-- api/*            // API routes and code that are not auth
     |-- auth/*           // All things auth (including API routes) 
     |                    // NOTE: Both above are just used by server.rs
     |
     |-- storage.rs       // Storage-related traits/types for swapping out SQL engines
     |    `-- /sqlite.rs  // SQLite implementation
     |
     `-- *.rs             // Misc. other files used by server.rs
```
