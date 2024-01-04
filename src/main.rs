extern crate reqwest;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::Cursive;
use cursive::view::ScrollStrategy;
use cursive::views::{BoxView, Dialog, DummyView, EditView, LinearLayout, ScrollView, TextView};
use custom_error::custom_error;

// Creating a custom error

custom_error! {ChatError
    JSONError{source: serde_json::error::Error} = @{
        source.to_string()
    },
    ReqwestError{source: reqwest::Error } = @{
        source.to_string().split(": ").collect::<Vec<&str>>()[1]
    },
    Unknown = "unknown error"
}

//Defining structs for Rust Json Response

#[derive(Deserialize)]
struct Response {
    t: Time,
    m: Vec<MessageResp>,
}

#[derive(Deserialize)]
struct MessageResp {
    d: Message,
}

#[derive(Deserialize)]
struct Time {
    t: String,
}

//Message is a sub object of MessageResp
#[derive(Serialize, Deserialize)]
struct Message {
    uuid: String,
    text: String,
}

//Asynchronous Rust(multithreading) Creating two threads: a thread that searches for new messages arriving and a thread for our UI

fn main() {

    //We create two channels, one to pass the channel name to the subscribe function 
    //Another to send new messages from the subscribe function to the UI 
    let (channel_sender, channel_receiver) = channel();
    let (mut msg_sender, msg_receiver) = channel();
    
    //create a thread using spawn, which is the simplest way to create new threads in Rust
    //Create a seperate thread, this allows us to have a subscribe loop that wont stop the UI from updating
    let _handle1 = thread::spawn(move || {
        let mut time_token = "".to_string();
        //We wait for the UI to send us the channel name
        let test_channel = channel_receiver.recv();

        // When we receive a variable from another thread, we don’t know if it is an error or not. If the value is “Ok,” unwrap it into a string
        if test_channel.is_ok() {

            let channel_name: String = test_channel.unwrap();
            loop {
                //We call the subscribe function, which returns a Result type
                let result: Result<String, ChatError> = subscribe(&time_token, &mut msg_sender, &channel_name);
            }
        }
        if result.is_ok() {
            //We update the time_token var to get all messages that happened after that specific time.
            time_token = result.ok().unwrap();
        } else if result.is_err() {
            let err = result.unwrap_err();
            //If the request times out, thats okay, we just restart it with that same time token, looking for new messages.
            if err.to_string() != "timed out" {
                println!(
                    "Error: {:?} 
                    Please restart application to try again.",
                    err.to_string()
                );
                break;
            }
        }

    }
