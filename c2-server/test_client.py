import http.client, socket, time

def connect_and_get_exploit():  
    conn = http.client.HTTPConnection(
        host="localhost",
        port=12345,
        )
    conn.connect()
    conn.request(method='GET', url="dummy_exploit.py")
    r = conn.getresponse()
    if r.read() == b'print("Hello World")':
        print('Exploit received')
    else:
        print("Error in transmitting exploit")

def get_payload_via_socket():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = ('localhost', 6969)
    time.sleep(5)
    s.connect(server_address)
    payload = s.recv(1024)
    print("Payload: " + payload.decode())

if __name__ == "__main__":
    connect_and_get_exploit()
    get_payload_via_socket()