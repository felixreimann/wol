# Wake On LAN
Wake up remote computers in the local network.
## Installation
'''
cargo install
'''
## Execution
With the MAC address of the remote computer, call
'''
./wol <MAC>
'''
For example:
'''
./wol 00:22:44:66:88:AA
'''
## Command line options
* **-h** print help message
* **-4** use a UDP/IPv4 packet
* **-6** use a UDP/IPv6 packet

