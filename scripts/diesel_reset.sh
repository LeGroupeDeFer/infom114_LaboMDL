#!/usr/bin/env bash

# place ourselves in the app root directory
cd /usr/src/app || exit 1

bash scripts/diesel_revert_all.sh TEST
bash scripts/diesel_revert_all.sh DEV
bash scripts/diesel_migration_run.sh TEST
bash scripts/diesel_migration_run.sh DEV

diesel print-schema > src/database/schema.rs
