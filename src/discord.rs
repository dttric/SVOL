extern crate discord_rpc_client;
use discord_rpc_client::{ Client, Event };
use std::io;
use std::thread;
use std::time;
use colorized::*;

fn do_something() {}

pub fn init(state: &str, large_image: &str, large_text: String) -> ()
{
  let mut drpc = Client::new(1219615194695602238);
  drpc.on_event(Event::Ready, |ctx| { do_something() });

  drpc.start();
  drpc.set_activity( |act| act
    .state(state)
    .assets(|ac| ac
      .large_image(large_image)
      .large_text(large_text)
    )).expect("Failed to set activity");
}