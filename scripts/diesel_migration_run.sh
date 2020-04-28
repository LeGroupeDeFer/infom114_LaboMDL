#!/usr/bin/env bash

# force quit when fault
set -eu

# place ourselves in the app root directory
cd /usr/src/app || exit 2

info()
{
    echo "Please give the database environment where you want to generate migration (DEV or TEST)"
    echo "    ex :"
    echo "    sh diesel_migration_run.sh TEST"
    exit 1
}

if [ "$#" -lt 1 ]
then
    info
fi

if [[ "$1" == "DEV" ]] 
then
    DATA=$(source .env && echo $DATABASE_URL)
    export DATABASE_URL=$DATA && diesel migration run
elif [[ "$1" == "TEST" ]]
then
    DATA=$(source .env && echo $TEST_DATABASE_URL)
    export DATABASE_URL=$DATA && diesel migration run
else    
    info
fi

exit 0

