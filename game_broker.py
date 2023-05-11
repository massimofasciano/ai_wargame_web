from http.server import HTTPServer, BaseHTTPRequestHandler
from http import HTTPStatus
import json
import time


message = {
    'from': [3,4],
    'to': [4,4],
    'move': 0,
    'player': 0
}

class _RequestHandler(BaseHTTPRequestHandler):

    def _set_headers(self):
        self.send_response(HTTPStatus.OK.value)
        self.send_header('Content-type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

    def do_GET(self):
        global message;
        self._set_headers()
        self.wfile.write(json.dumps(message).encode('utf-8'))

    def do_POST(self):
        global message;
        length = int(self.headers.get('content-length'))
        new_message = json.loads(self.rfile.read(length))
        message = new_message;
        self._set_headers()
        self.wfile.write(json.dumps({'success': True}).encode('utf-8'))
        print(message)

    def do_OPTIONS(self):
        self.send_response(HTTPStatus.NO_CONTENT.value)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST')
        self.send_header('Access-Control-Allow-Headers', 'content-type')
        self.end_headers()

def run_server():
    server_address = ('', 8001)
    httpd = HTTPServer(server_address, _RequestHandler)
    print('serving at %s:%d' % server_address)
    httpd.serve_forever()


if __name__ == '__main__':
    run_server()
