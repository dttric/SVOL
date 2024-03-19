extern crate discord_rpc_client;
use discord_rpc_client::{ Client, Event };
use std::io;
use std::thread;
use std::time;

pub fn init(state: &str, large_image: &str) -> ()
{
  let mut drpc = Client::new(1219615194695602238);
  drpc.on_event(Event::Ready, |ctx| {
    println!("READY!");
  });

  drpc.start();
  drpc.set_activity( |act| act
    .state(state)
    .assets(|ac| ac
      .large_image(large_image)
    )).expect("Failed to set activity");
}