# mksws

A simple command line utility that creates a static web server in the current directory.

## Installation

This will install the binary into `/usr/bin`

    wget https://github.com/nanashi-1/mksws/releases/download/v1.0-alpha/mksws-1.0-alpha.tar.gz && mkdir mksws && tar -zxf mksws-1.0-alpha.tar.gz -C mksws && cd mksws && bash install.sh

## Usage

Simple usage:

    mksws

This will run a static web server in the current directory using the IP address `0.0.0.0` and port `8181`.

|Option|Description|
|---|---|
|`-i` or `--ip_address`|The server will listen to this IP address. It will be `0.0.0.0` by default.|
|`p` or `--port`|The server will listen to this port. It will be `8181` by default.
|`-h` or `--help`|Shows the help message|
|`-V` or `--version`|Print the version|

## License

This project is licensed under the [GNU General Public License v3.0 (GPLv3)](https://www.gnu.org/licenses/gpl-3.0.html).
