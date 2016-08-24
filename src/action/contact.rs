/*  Periodically crawl web pages and alert the user of changes 
 *  Copyright (C) 2016  Owen Stenson
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>. 
 *
 *  More information in the enclosed `LICENSE' file
 */

//Send messages 
// via Email using EMAIL_FROM (and EMAIL_ALIAS?) var(s)
// via pushjet using PUSHJET_SECRET var
//      (but temporarily using pushjet.json located in any parent dir)

extern crate json;
extern crate hyper;
extern crate url;
extern crate lettre;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
//web stuff
use self::hyper::Url;
use self::url::form_urlencoded::Serializer;
use self::hyper::header::{ContentType, Authorization, Basic};
//lettre stuff
use self::lettre::email::EmailBuilder;
use self::lettre::transport::EmailTransport;
use self::lettre::transport::smtp::SmtpTransportBuilder;

const CONFIG_FILE: &'static str = "pushjet.json";
const DEFAULT_PJ_URL: &'static str = "https://api.pushjet.io";
const PROGRAM_DESCIP: &'static str = "page-mon";

fn find_file(name: &str) -> Option<PathBuf> {
    //check current directory for file
    //if absent, check previous directory for file
    //start with current working directory
    let mut path = env::current_dir().unwrap();
    loop {
        //for each parent directory, check if file is present
        let guess = path.join(name);
        //println!("\tguess = {:?}", guess);
        if guess.is_file() {
            //sought file is present
            return Some(guess);
        } else if path.pop() == false {
            //parent directory doesn't exist, so file isn't found
            return None;
        }
    }
}

pub fn load_config() -> Result<(hyper::Url,String),String> {
    //returns url and `secret`
    //parse file first
    //locate file in some parent/grandparent folder;
    let path = match find_file(CONFIG_FILE) {
        Some(p) => p,
        None    => return Err("No such file exists".to_string())
    };
    //open it
    let mut file = match File::open(path) {
        Ok(f)   => f,
        //Err(e)=>return Err(e.description().to_string().as_ref()) //TODO
        //do this with Result<_,&str> and lifetimes?
        Err(e)  => 
            return Err(format!("File input err: {}", e.description()))
    };
    // and read its contents;
    let mut text = String::new();
    if let Err(e) = file.read_to_string(&mut text) {
        return Err(format!("File read err: {}", e.description()))
    }
    //then parse the data in it.
    let data = match json::parse(&text) {
        Ok(d)   => d,
        Err(e)  => 
            return Err(format!("JSON parse err: {}", e.description()))
    };
    //`secret` must be present
    let secret = match data["secret"].as_str() { 
        Some(s) => s.to_string(),
        None => 
            return Err("Data err: missing field `secret`".to_string())
    };
    //`url` has a reasonable default 
    let address = match data["url"].as_str() {
        Some(u) => u,
        None    => DEFAULT_PJ_URL,
    };
    //make sure url is valid
    let url = match hyper::Url::parse(address) {
        Ok(u)   => u,
        Err(e)  => 
            return Err(format!("Url parse err: {}", e.description()))
    };

    Ok((url, secret))
}

pub fn generate_email(from: &str, to: &str, subject: &str, 
                      text: &str) -> Result<String,String> {
    //send an email via lettre over port 25
    //requires postfix be set up with a domain and everything
    let email = EmailBuilder::new()
                    .to(to)
                    .from(from)
                    .subject(subject)
                    .body(text)
                    .build();

    let email = match email {
        Ok(e)  => e,
        Err(e) => return Err(format!("Email malformed: {}", e)),
    };

    //SmtpTransportBuilder::localhost() should never fail, right?
    //Maybe if /etc/hosts is screwy or something??
    //TODO should probably fix this; don't want it crashing the program
    let mut mailer = SmtpTransportBuilder::localhost().unwrap().build();

    return match mailer.send(email) {
        Err(e) => Err(format!("Email failed to send: {}", e.description())),
        Ok(r)  => Ok(format!("Email Response: {}", r.message().join("; "))),
    }
}

pub fn post_email(api_key: &str, domain: &str, to: &str, 
             subject: &str, text: &str) -> Result<String,String> {
    //send an email via a POST request and Mailgun
    //more consistent, but relies on an external service 
    //  auth: "api:api_key"
    //  url:  "api.mailgun.net/.../DOMAIN/..."
    //  from: "page-mon <mailgun@DOMAIN 
    
    let url = format!("https://api.mailgun.net/v3/{}/messages", domain);
    let url = match Url::parse(url.as_ref()) {
        Ok(u)  => u,
        Err(e) => return Err(format!("Url malformed: {}", e.description())),
    };

    let from = format!("{} <mailgun@{}>", PROGRAM_DESCIP, domain);

    let client = hyper::Client::new();

    //serialize data
    let payload: String = Serializer::new(String::new())
                    .append_pair("from",   from.as_ref())
                    .append_pair("to",     to)
                    .append_pair("subject",subject)
                    .append_pair("text",   text)
                    .finish();

    //set up authorization as header
    let auth = Authorization (
		   Basic {
			   username: "api".to_owned(),
			   password: Some(api_key.to_owned()),
		   });

    //set up and make request
    let res = client.post(url)
                    .header(auth)
                    .body(payload.as_bytes())
                    .send();

    match res {
        Err(e) => Err(e.description().to_string()),
        Ok(t)  => Ok(t
                     .status
                     .canonical_reason()
                     .unwrap_or("Success: Reason unknown")
                     .to_string()),
    }

}

pub fn pushjet(pushjet_url: hyper::Url, secret: &str, message: &str, 
               title: &str, level: u8, link: &str) -> Result<String,String> {
    //on failure: error description wrapped in Err()
    //on success: return response (for logging?) in Ok()
    //  could just return ()? does anyone care? Will there be logging?
    //`secret` (and pushjet_url?) should come from User-defined Vars in config
    let url = pushjet_url.join("message").unwrap();
    let client = hyper::Client::new();

    //serialize data
    let payload: String = Serializer::new(String::new())
                    .append_pair("secret",  secret)
                    .append_pair("message", message)
                    .append_pair("title",   title)
                    .append_pair("level",   level.to_string().as_str())
                    .append_pair("link",    link)
                    .finish();

    //set up and make request
    let res = client.post(url)
                    .header(ContentType::form_url_encoded())
                    .body(payload.as_bytes())
                    .send();

    //return status code or error message
    match res {
        Err(e) => Err(e.description().to_string()),
        Ok(t)  => Ok(t
                     .status
                     .canonical_reason()
                     .unwrap_or("Success: reason unknown")
                     .to_string()),
    }
}




