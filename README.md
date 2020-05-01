# UNanimity

[![Build Status](https://drone.findot.me/api/badges/LeGroupeDeFer/infom114_LaboMDL/status.svg)](https://drone.findot.me/LeGroupeDeFer/infom114_LaboMDL)
![Codecov coverage](https://codecov.io/gh/LeGroupeDeFer/infom114_LaboMDL/branch/dev/graph/badge.svg)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/LeGroupeDeFer/infom114_LaboMDL/blob/master/LICENSE)
![GitHub tag](https://img.shields.io/github/v/tag/LeGroupeDeFer/infom114_LaboMDL)

## Getting started

1. Install [docker](https://hub.docker.com/search?q=&type=edition&offering=community&sort=updated_at&order=desc) and [docker-compose](https://docs.docker.com/compose/install/)
2. create a `.env` file with a `DATABASE_URL=mysql://testuser:testpassword@db/test_rocket`
3. `docker-compose build`
4. `docker-compose up [--build] [--detach]`
5. Go on <http://localhost:8000>

## Execute seeder

```shell script
cargo run --bin=seeder
```

## Tests

1. add a `TEST_DATABASE_URL` in your `.env` file
2. execute the `scripts/setup_test.sh` (This will ensure that the database schema is up to date in your test database)
3. `cargo test -- --test-threads=1` (Since the database is truncated at the begining of each test, running the tests in parallel is impossible)

## Documentation

Cargo natively propose a solution for the documentation.

You can use `cargo doc --open` to open the generated documentation.
