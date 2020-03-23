#!/usr/bin/env bash

# force quit when fault
set -eu

# place ourselve in the app root directory
cd /usr/src/app

# TODO : validate the existence of TEST_DATABASE_URL

# export variable for diesel cli
DATABASE_URL=$(export $(cat .env | grep TEST_DATABASE_URL) && echo $TEST_DATABASE_URL)

# setup & redo migrations to have a clean database
export DATABASE_URL=$DATABASE_URL && diesel setup
export DATABASE_URL=$DATABASE_URL && diesel migration redo
export DATABASE_URL=$DATABASE_URL && diesel print-schema
