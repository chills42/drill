use std::collections::HashMap;
use std::io::Read;

extern crate yaml_rust;
use self::yaml_rust::Yaml;

extern crate colored;
use self::colored::*;

extern crate serde_json;
use self::serde_json::Value;

extern crate hyper;
use self::hyper::client::{Client, Response};

extern crate time;

use interpolator;

#[derive(Clone)]
pub struct Request {
  name: String,
  url: String,
  time: f64,
  pub with_item: Option<Yaml>,
  pub assign: Option<String>,
}

impl Request {
  pub fn is_that_you(item: &Yaml) -> bool{
    item["request"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, with_item: Option<Yaml>) -> Request {
    let reference: Option<&str> = item["assign"].as_str();

    Request {
      name: item["name"].as_str().unwrap().to_string(),
      url: item["request"]["url"].as_str().unwrap().to_string(),
      time: 0.0,
      with_item: with_item,
      assign: reference.map(str::to_string)
    }
  }

  pub fn execute(&mut self, base_url: &String, context: &mut HashMap<&str, Yaml>, responses: &mut HashMap<String, Value>) {
    if self.with_item.is_some() {
      context.insert("item", self.with_item.clone().unwrap());
    }

    let interpolator = interpolator::Interpolator::new(&base_url, &context, &responses);

    let final_url = interpolator.resolve(&self.url);

    let mut response = self.send_request(&final_url);

    println!("{:width$} {} {} {}{}", self.name.green(), final_url.blue().bold(), response.status.to_string().yellow(), (self.time * 1000.0).round().to_string().cyan(), "ms".cyan(), width=25);

    // TODO: Solve multable borrow issue
    // if self.assign.is_some() {
    //   self.assign_response(&mut response, responses);
    // }
  }

  fn send_request(&mut self, url: &str) -> Response {
    let client = Client::new();
    let begin = time::precise_time_s();

    let response = client.get(url).send();

    if let Err(e) = response {
      panic!("Error connecting '{}': {:?}", url, e);
    }

    self.time = time::precise_time_s() - begin;

    response.unwrap()
  }

  // TODO: Solve multable borrow issue
  // fn assign_response(&self, response: &mut Response, responses: &mut HashMap<String, Value>) {
  //   let mut data = String::new();
  //   let ref option = self.assign;
  //   let key = option.unwrap();

  //   response.read_to_string(&mut data).unwrap();

  //   let value: Value = serde_json::from_str(&data).unwrap();

  //   responses.insert(key, value);
  // }
}