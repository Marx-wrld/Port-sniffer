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
