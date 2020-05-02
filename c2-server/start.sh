#!/bin/sh

python3 implant-server.py & 
sudo python3 -m http.server 80 &
sudo python3 serverpy
