# Port Scanner (PS)

A lightweight port scanner tool for scanning open ports on a given IP or CIDR range.

## Installation

Run the following command to download and install the tool:

```bash
curl -fsSL https://raw.githubusercontent.com/rajiknows/portscanner/main/install.sh | bash
```


## Usage

to check availablity of single port
```bash
sk -p <port>
```
to check availablity of range of ports 
```bash
sk -p <start> <end>
```
to free a port
```bash 
sk -n <port>
```
to free a range of ports
```bash
sk -n <start> <end>
```


