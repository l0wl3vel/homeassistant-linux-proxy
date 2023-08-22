# Home Assistant Linux Proxy

**Goal: ** Forward useful data to Home Assistant via a Webhook

Why? I use a TV as a PC monitor, but instead of going into standby on signal loss it displays a *no input* warning message. This app allows me to write an automation to turn the TV on and off depending on my lock screen.

## Features

- DBus -> Webhook
- Notifies Home Assistant about Lock Screen state

## Supported Desktop Environments

Currently only Gnome/Mutter is supported. Unfortunately Mutter does not write to the `org.freedesktop.ScreenSaver` bus. But implementing other DEs should not be hard.

## Setup

1. Set up an Automation for handling messages. An example is:
```yaml
alias: PC Webhook
description: ""
trigger:
  - platform: webhook
    allowed_methods:
      - POST
    local_only: false
    webhook_id: "plz_randomize"
condition: []
action:
  - choose:
      - conditions:
          - condition: template
            value_template: "{{ trigger.json.lock_screen_status == true }}"
        sequence:
          - service: remote.turn_off
            data: {}
            target:
              device_id: remote_id
      - conditions:
          - condition: template
            value_template: >-
              {{ trigger.json.lock_screen_status == false or screen_woken_up ==
              true }}
        sequence:
          - service: remote.turn_on
            data: {}
            target:
              device_id: remote_id
mode: single

```


2. Retrieve your Webhook URL from {Instance URL}/config/cloud/account

3. Run `HOMEASSISTANT_WEBHOOK=https://hooks.nabu.casa/{hookid} homeassistant-linux-proxy`

## Known issues

Mutter does does not fire the WakeUpScreen Signal when waking up the screen. It only fires when the lock screen is unlocked. So you might not see the login screen and have to enter your password blind.