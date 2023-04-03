#!/usr/bin/env python3

import argparse
import socket


def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('-t', '--target', default='localhost')
    parser.add_argument('-p', '--port', type=int, default=9999)
    parser.add_argument('args', nargs='*')

    args, extra_args = parser.parse_known_args()

    UDP_IP = args.target
    UDP_PORT = args.port
    MESSAGE = b"Hello, World!"

    print("UDP target IP: %s" % UDP_IP)
    print("UDP target port: %s" % UDP_PORT)
    print("message: %s" % MESSAGE)

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.sendto(MESSAGE, (UDP_IP, UDP_PORT))
    rx_data, addr = sock.recvfrom(1024)
    print("got: {} from {}".format(rx_data, addr))


if __name__ == '__main__':
    main()
