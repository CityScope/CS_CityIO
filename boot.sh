#!/bin/bash

tmux -S /tmp/cityio kill-server
tmux -S /tmp/cityio new -d -s cityio
tmux -S /tmp/cityio send-keys 'kill -9 $(lsof -t -i:8080)' C-m
tmux -S /tmp/cityio send-keys '/Users/yasushi/Documents/code/CS_CityIO_Backend/target/release/cs_cityio_backend' C-m

chmod 777 /tmp/cityio
