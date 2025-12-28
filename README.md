# pbip

> **p**ork**b**un **ip** (updater)

This little program updates the IP for A records of a specified domain hosted on porkbun.com and monitors it indefinitely. This is perfect for servers on a dynamic IP that can potentially change, which is typical of a self-hosted setup.

**NOTE**: You will need to install curl first on your system before attempting to run the program. This is how we obtain the IP of the machine running the program. You will also need to enable API access for your porkbun domain.

## env
Make a config.toml file in the root directory with the following.
```toml
[env]
secretapikey=SECRET_KEY_HERE
apikey=KEY_HERE
```

## Example
```sh
./pbip example.com 5
```
This will attempt to update records every `5` minutes for domain `example.com`.

## Build
```sh
git clone ssh://git@codeberg.org/gnp/pbip.git
cd pbip
cargo build --release --config config.toml
```
