# powoffie
Have you ever wanted to let your friends shut down your computer? It is now possible with the help of powoffie!

# Features
1. Powoffie can't do anything other than power off your computer. It is designed to _only_  ever run `poweroff` and `notify-send` with pre-determined options.

1. Powoffie has 2 parts: host and relay.
You can run the relay on a server (that can't be shut down) and the host on your computer. So you can have either 1 or 2 instances.
With just 1 instance, your friends are going to get "Connection refused" errors when your PC is off. And it has less securtity options (only the password). It is merely designed to be used by the relay, though you could expose it to the internet directly, if you wanted.

1. All your friends can have their own personal token - add and remove them as you wish to control who has access.

1. You can have a special "today's password" that has to be used in combination with your friends' tokens.

1. Customizable rate limits that can be disabled entirely

# Setup
You can make up your own weird way to use this but here's how it's intended to be used:

## Configuration
### On relay
```json
{
    "mode": "relay",
    "admin-token": "something",
    "relay-host-token": "something",
    "logLevel": "Debug",
    "listen": {
        "address": "0.0.0.0",
        "port": "6969"
    }
}
```

I am pretty sure all of this is self-explanatory.

- The `admin-token` is used to add/remove your friends' passwords and set "today's password".
- The `relay-host-token` (which should be the same as on the host) lets the host and relay communicate with each other

### On host
```json
{
    "mode": "host",
    "relay-host-token": "something",
    "logLevel": "Debug",
    "listen": {
        "address": "0.0.0.0",
        "port": "6969"
    }
}
```

I am pretty sure all of this is self-explanatory too.

- The `relay-host-token` (which should be the same as on the relay) lets the host and relay communicate with each other

## Installation
### On relay
- Download the relay from [releases](https://github.com/wait-what/powoffie/releases)
- Create a `config.json` based on [./config.example.json](./config.example.json)
    - `bind` - the address and port the relay should listen on
    - `relay-endpoint` - where the relay is accessible from the network
    - `host-endpoint` - where the host is accessible from the network
    - `rate-limit` - `n` seconds between each request, so `5` means there can only be 1 request per token, per 5 seconds. `0` to disable.
    - `tokens` - an object with the following format: `{ "token": "username" }`. The user name is used when showing who won.
- Run `./powoffie-relay-...` (whatever the full name of it is)

Now it's your job to figure out how to port forward/unfirewall/http-proxy powoffie so that your friends can actually access it.

### On host
- Download the host from [releases](https://github.com/wait-what/powoffie/releases)
- Run `./powoffie-host-...` (whatever the full name of it is) and give it the following arguments:
    - Port
    - Timeout
    - Password

So for example, `./powoffie-host 6968 5 amogus`. This way, if someone guesses the password `amogus`, your pc will turn off in 7 seconds (5 seconds + 2 hardcoded)

## By friends
**The best part!** Now that your relay is available on the internet, your friends can spam you with password guesses.

Your friends can navigate to the relay's endpoint and type in their token and today's password. If everything goes well, your PC should turn off.

# License
This amazing project is licensed under [MIT](./LICENSE)!
