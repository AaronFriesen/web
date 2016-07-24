#!/bin/sh

sudo cp config/ssl.conf /etc/httpd/conf.d/ssl.conf
sudo httpd -f /home/ec2-user/git/web/config/httpd.conf -k start
