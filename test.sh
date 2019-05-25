#!/usr/bin/env bash


echo "** cityio test script!  **"

echo " listing: "

curl http://localhost:8080/api/tables/list

echo ""

sleep 1

echo " posting data: "

curl -X POST -d '{"data": "test"}' http://localhost:8080/api/table/update/test

echo ""

sleep 1

echo " checking if it was posted: "

curl http://localhost:8080/api/tables/list

echo ""

sleep 1

echo " get that data: "

curl http://localhost:8080/api/table/test

echo ""

sleep 1

echo " deep get: "

curl http://localhost:8080/api/table/test/meta

echo ""

sleep 1

echo " add module: "

curl -X POST -d '{"lots": "data"}' http://localhost:8080/api/table/update/test/module

echo ""

sleep 1

echo " deep get module: "

curl http://localhost:8080/api/table/test/module/lots

echo ""

sleep 1

echo " clear: "

curl http://localhost:8080/api/table/clear/test

echo ""

sleep 1

echo " list: "

curl http://localhost:8080/api/tables/list
echo ""

