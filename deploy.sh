#!/bin/bash

cd /var/www/astrology-saas || exit

echo "Pulling latest changes from GitHub..."
git reset --hard
git pull origin main

echo "Updating file permissions..."
sudo chown -R www-data:www-data /var/www/astrology-saas
sudo chmod -R 755 /var/www/astrology-saas

echo "Restarting Nginx..."
sudo systemctl restart nginx

echo "Deployment completed!"
