!/bin/bash

clear

echo "updating cityio"

pm2 stop cityio

cd /home/code/cityio/

git pull

npm production

pm2 restart cityio

echo "** cityio is now updated **"

