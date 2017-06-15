#!/bin/sh

cp config/nginx.conf /etc/nginx/nginx.conf
service nginx reload
screen -X -S srvr quit
cargo build --release
screen -dmS srvr cargo run --release
