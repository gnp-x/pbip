# pbip

> **p**ork**b**un **ip**

This little program updates the IP for A records of a specified domain hosted on porkbun.com and monitors it indefinitely. This is perfect for servers on a dynamic IP that can potentially change, which is typical of a self-hosted setup.

**NOTE**: You will need to install curl first on your system before attempting to run the program. This is how we obtain the IP of the machine running the program.

## env
Recommend making an .env file with the following since the keys are really long...
```sh
secretapikey=SECRET_KEY_HERE
apikey=KEY_HERE
```
or you can just set them in the terminal or .rc file.
```sh
export secretapikey=SECRET_KEY_HERE
export apikey=KEY_HERE
```

## Example
```sh
./pbip liminal.cafe 5
```
This will check to update records every `5` minutes for domain `liminal.cafe`.

## Build
```sh
git clone ssh://git@codeberg.org/gnp/pbip.git
cd pbip
cargo build
```
