# infom114_LaboMDL

[![Build Status](https://drone.findot.me/api/badges/LeGroupeDeFer/infom114_LaboMDL/status.svg)](https://drone.findot.me/LeGroupeDeFer/infom114_LaboMDL)
[![GPLv3 license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/LeGroupeDeFer/infom114_LaboMDL/blob/master/LICENSE)
![GitHub tag](https://img.shields.io/github/v/tag/LeGroupeDeFer/infom114_LaboMDL)

## Quickstart

1. install rust(cargo)
2. go nighty
3. setup mariadb on your machine (look at logins in .env & rocket.toml)
4. Install `diesel-cli`
5. `diesel migration run`
6. `cargo build` && `cargo run`
7. go on localhost:8000

## Dockerization

1. Install docker and docker-compose
2. docker-compose up [--build]
3. go on <http://localhost:8000/hello/yourname>

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
