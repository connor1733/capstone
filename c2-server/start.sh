#!/bin/sh

python3 implant-server.py & 
echo "Implant server started on port 8080"
sleep 10
sudo python3 -m http.server 80 &
echo "Started HTTP server on port 80"
sleep 10
sudo python3 serverpy
