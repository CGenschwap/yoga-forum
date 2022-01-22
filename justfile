# Build everything locally (debug mode)
build:
    cargo build
    just template
    
# Run the server (dev mode)
run-dev: template
    YOGA_SECRET=test cargo run --bin yoga-forum | bunyan

# Run the server (release mode)
run-release SECRET:
    YOGA_SECRET={{SECRET}} cargo run --bin yoga-forum --release

# Test everything
test:
    cargo test

# Test a single test
test-one TEST:
    cargo test {{TEST}}

# Actor-Testing
actor-test:
    cargo run --bin yoga-actor

# Templating
template:
    ./target/debug/templatize-me index.html -c templates/context.toml > public/index.html
    ./target/debug/templatize-me guide.html -c templates/context.toml > public/guide.html
    ./target/debug/templatize-me api_docs.html -c templates/context.toml -c public/openapi.json > public/api_docs.html
    ./target/debug/templatize-me yoga_api_v1.txt -c templates/context.toml -c public/openapi.json > public/yoga_api_v1.txt

template-watch:
    find templates/ | entr -r just template

# Deployment
deploy:
    flyctl deploy

deploy-test: template
    flyctl deploy -c fly.test.toml

