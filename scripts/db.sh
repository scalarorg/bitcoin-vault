#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Missing required parameter: database name."
    exit 1
fi

default_port="8012"
port=${2:-$default_port}

db=$(realpath $1)
db_dir=$(dirname $db)
db_file=$(basename $db)
echo "Starting on: http://127.0.0.1:$port"
docker run -it -d -p "$port:8080" -v $db_dir:/data -e SQLITE_DATABASE="$db_file" coleifer/sqlite-web
