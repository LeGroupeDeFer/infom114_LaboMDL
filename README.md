# infom114_LaboMDL

[![Build Status](https://drone.findot.me/api/badges/LeGroupeDeFer/infom114_LaboMDL/status.svg)](https://drone.findot.me/LeGroupeDeFer/infom114_LaboMDL)
![Codecov coverage](https://codecov.io/gh/LeGroupeDeFer/infom114_LaboMDL/branch/dev/graph/badge.svg)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/LeGroupeDeFer/infom114_LaboMDL/blob/master/LICENSE)
![GitHub tag](https://img.shields.io/github/v/tag/LeGroupeDeFer/infom114_LaboMDL)

## Quickstart

1. Install docker and docker-compose
2. create a `.env` file with a `DATABASE_URL=mysql://testuser:testpassword@db/test_rocket`
3. `docker-compose up [--build] [--detach]`
4. go on <http://localhost:8000/hello/yourname>

## Features

- Utilisateurs: Profs, etudiants, AGEs, Cercles,
- Cana
- Categories des contenu
- Types de contenu:
  - Informatif (Event)
  - Decisionnel (Poll)
  - Questions (Comme StackOverflow? :D)
  - Boite a idee

## DB

![schema](out/uml/database_schema/db_mdl.png)

## Tests

1. add a `TEST_DATABASE_URL` in your `.env` file
2. execute the `scripts/setup_test.sh` (This will ensure that the database schema is up to date in your test database)
3. `cargo test -- --test-threads=1` (Since the database is truncated at the begining of each test, running the tests in parallel is impossible)

## Documentation

Cargo natively propose a solution for the documentation.

You can use `cargo doc --open` to open the generated documentation.
