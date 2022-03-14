# bat_warner

🔋Bat Warner is a program written in *Rust* 🦀 which alerts if system is charged over limit and is still plugged in. 

## Setup

For setup I recommend to let systemd run the program every minute.

1. Build binary 
  ```bash
  $ cargo build --release
  ```
3. Place binary in a better folder
  ```bash
  sudo cp /target/release/bat3 /usr/bin/bat3
  ```
5. Setting up systemd
  
  Place in ```/etc/systemd/system/``` for system wide usage or in ```/home/username/.local/share/systemd/user/``` for user
  
  ##### Timer
  ```Unit
  [Unit]
  Description="Timer wich runs battery every minute"
  
  [Timer]
  OnUnitActiveSec=60
  Unit=battery.service
  
  [Install]
  WantedBy=default.target
  ```
  
  ##### Service
  ```Unit
  [Unit]
Description=Unit which alerts if device is charged over 80%

[Service]
Type=simple
Environment="BAT_LIMIT=80"
Environment="BAT_FILE=/sys/class/power_supply/cw2015-battery/"
ExecStart=/usr/bin/bat3

[Install]
WantedBy=default.target
  ```
  
  Now initialysing with 
  ```bash
  $ sudo systemctl enable battery.timer
  ```
  ```bash
  $ sudo systemctl enable battery.service
  ```
  for system-wide and
  ```bash
  $ systemctl --user enable battery.timer
  ```
  ```bash
  $ systemctl --user enable battery.service
  ```
  for user installation
  
7. See log with with journalctl

  sytem:
  ```bash
  $ journalctl -u battery.service
  Mär 12 12:00:00 personalcomputer Systemd[431]: Started Unit which alerts if device is charged over 80%.
  Mär 12 12:00:00 personalcomputer bat3[823]: 50
  ```
  
  user:
 
  ```bash
  $ journalctl --user -u battery.service
  Mär 12 12:00:00 personalcomputer Systemd[431]: Started Unit which alerts if device is charged over 80%.
  Mär 12 12:00:00 personalcomputer bat3[823]: 50
  ```
