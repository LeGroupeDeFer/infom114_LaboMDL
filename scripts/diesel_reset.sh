#!/usr/bin/env bash

# place ourselve in the app root directory
cd /usr/src/app

bash scripts/diesel_revert_all.sh TEST
bash scripts/diesel_revert_all.sh DEV
bash scripts/diesel_migration_run.sh TEST
bash scripts/diesel_migration_run.sh DEV