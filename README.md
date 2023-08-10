# Unit Tests
- To see all logs:
```
TEST_LOG=true cargo test health_check_works | bunyan
```
- Run unit test and remove sqlx noise
```
export RUST_LOG="sqlx=error,info"
export TEST_LOG=enabled
cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan
```
- Otherwise just run
```
cargo test
```

# Prod
To run on remote production server.
- Dont forget export APP_ENVIRONMENT=production

# Database migration
- If docker is running;
-- SKIP_DOCKER=true ./scripts/init_db.sh
- Else:
-- ./scripts/init_db.sh
