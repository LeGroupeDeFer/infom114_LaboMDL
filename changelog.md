# Changelog

Summary of implemented features classified by release (github tag)

## [0.2.4] - 2020-04-20

- add models structure for tags
- refactor (add mod `prelude` & `tables` in database/models)
- improve tag tests

## [0.2.3] - 2020-04-10

- add tests related to role creation
- add documentation on routes and models related to role management
- basic implementation of capability management
- add tests related to role update
- add tests related to role deletion
- implement route for assigning and unassagning role to/from a user
- add tests related to user-role assignation

## [0.2.1] - 2020-04-01

- update database helper scripts
- refactor routes to counter issue <https://github.com/SergioBenitez/Rocket/issues/1262>
- implement test helper for jwt token protected routes
- begin tests for role management

## [0.2.0] - 2020-03-25

- add database migrations for role management
- create structure for seeder
- add GET /api/capabilities route
- add GET /api/roles route
- add structure for /api/role routes
- update DB schemas (user, roles)
- add DB schemas (capabilities, roles_capabilities)
- move schema.rs to /database/
- implement business logic for capability database model
- begin to implement business logic for role database model

## [0.1.3] - 2020-03-24

- add tags openapi specs
- add roles openapi specs
- add posts openapi specs
- improve post UI (share button, search bar, tags, first draw for posts preview)
- users now need to be authenticated to upvote

## [0.1.2] - 2020-03-19

- transform crate into lib + bin
- add test structure
- remove .env from tracked files
- add check_email documentation test
- improve documentation
- model & db refactoring
- write all auth related tests

## [0.1.0] - 2020-03-02

- Fix issue #56 : manage 404 differently depending on uri (/api/)
- Login page UI (/login)
- Payload optimisation (600 kb -> 337 kb, concurrent loading)
- Fix issue #48 : use custom rocket igniter to bypass rocket.toml
- Fix issue #50 : send http 403 when not logged in
- Fix issue #60 : prevent register of authenticated user

## [0.0.3] - 2020-02-26

- Clean useless template
- Register process (issue #5)
- Login process (issue #6)
- Update openapi

## [0.0.2] - 2020-02-22

- Base App layout (sidebar & content, responsive)
- SPA routing (URI matching & lazy component loading)
- Frontend unit tests (js/utils & js/components)

## [0.0.1] - 2020-02-15

- User stories
- DB schema
- CI/CD (Drone)
- Rocket auth guard & cookies
- OpenAPI first draw
- Docker compilation cache (volume)
- CI/CD pipeline

## [0.0.0] - 2020-02-10

- POC rocket
- Diesel & MariaDB
- Docker image
