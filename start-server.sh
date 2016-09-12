#!/bin/sh

sudo cp config/ssl.conf /etc/httpd/conf.d/ssl.conf
sudo service httpd stop
sudo httpd -f `echo $PWD`/config/httpd.conf -k start
