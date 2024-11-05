#![allow(non_snake_case)]
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "Demo")]
pub async fn Demo() -> Result<()> {
    Ok(())
}
