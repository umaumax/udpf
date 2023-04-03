#!/usr/bin/env python3

import argparse
import socketserver


class MyUDPHandler(socketserver.BaseRequestHandler):

    def handle(self):
        data = self.request[0].strip()
        socket = self.request[1]
        print(
            "{}:{} wrote:".format(
                self.client_address[0],
                self.client_address[1]))
        print(data)
        socket.sendto(data.upper(), self.client_address)


def main():
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('-t', '--target', default="0.0.0.0")
    parser.add_argument('-p', '--port', type=int, default=9999)
    parser.add_argument('args', nargs='*')

    args, extra_args = parser.parse_known_args()

    HOST, PORT = args.target, args.port
    with socketserver.UDPServer((HOST, PORT), MyUDPHandler) as server:
        server.serve_forever()


if __name__ == '__main__':
    main()
