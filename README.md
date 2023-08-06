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

# Database migration
- If docker is running;
-- SKIP_DOCKER=true ./scripts/init_db.sh
- Else:
-- ./scripts/init_db.sh
