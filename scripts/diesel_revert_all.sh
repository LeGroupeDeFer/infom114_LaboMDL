#!/usr/bin/env bash

# force quit when fault
set -eu

# place ourselves in the app root directory
cd /usr/src/app || exit 2

info()
{
    echo "Please give the database environment you want to revert (DEV or TEST)"
    echo "    ex :"
    echo "    sh diesel_revert_all.sh TEST"
    exit 1
}

if [ "$#" -lt 1 ]
then
    info
fi

if [[ "$1" == "DEV" ]] 
then
    while true
    do
        DATA=$(source .env && echo $DATABASE_URL)
        export DATABASE_URL=$DATA && diesel migration revert
    done
elif [[ "$1" == "TEST" ]]
then
    while true
    do
        DATA=$(source .env && echo $TEST_DATABASE_URL)
        export DATABASE_URL=$DATA && diesel migration revert
    done
else
    info
fi


