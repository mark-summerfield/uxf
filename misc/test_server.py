#!/usr/bin/env python3

import http.server
import os
import socketserver

PORT = 5558

os.chdir(os.path.dirname(__file__) + '/t')


class Handler(http.server.SimpleHTTPRequestHandler):

    def log_message(self, format, *args):
        pass


with socketserver.TCPServer(('', PORT), Handler) as httpd:
    print(f'test_server.py on port {PORT}')
    httpd.serve_forever()
