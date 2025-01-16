# Port Scanner (PS)

A lightweight port scanner tool for scanning open ports on a given IP or CIDR range.

## Installation

Run the following command to download and install the tool:

```bash
curl -fsSL https://raw.githubusercontent.com/rajiknows/portscanner/main/install.sh | bash


## Usage
```bash
sk -l -p 8000 8010
sk -ip 192.168.1.1 -p 20 80
sk -cidr 192.168.1.0/24 -p 80 100
