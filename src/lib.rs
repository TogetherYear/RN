#![allow(non_snake_case)]
use napi_derive::napi;

#[napi(js_name="Sum")]
pub fn Sum(a: i32, b: i32) -> i32 {
  a + b
}