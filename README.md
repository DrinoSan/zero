# Unit Tests
- To see all logs:
```
TEST_LOG=true cargo test health_check_works | bunyan
```
- Otherwise just run 
```
cargo test
```

# Prod
To run on remote production server.
- Dont forget export APP_ENVIRONMENT=production