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

fn main() {
    println!("Hello, world!");
}

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