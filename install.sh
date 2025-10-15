sudo mkdir -p /opt/rt
sudo cp rt /opt/rt/rt
sudo cp -r locales /opt/rt/locales
sudo ln -s /opt/rt/rt /usr/local/bin/rt
sudo chmod +x /usr/local/bin/rt