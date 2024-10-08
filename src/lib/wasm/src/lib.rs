use std::net::Ipv4Addr;

use ipcount::{cidr::Cidr, to_addrs, to_cidrs};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_cidrs(addrs: Vec<String>) -> Result<Vec<String>, String> {
    let addrs: Vec<Ipv4Addr> = addrs
        .iter()
        .map(|l| l.trim().parse().map_err(Into::into))
        .collect::<anyhow::Result<Vec<Ipv4Addr>>>()
        .map_err(|e| e.to_string())?;

    Ok(to_cidrs::addrs_to_cidrs(&addrs)
        .into_iter()
        .map(|cidr| cidr.to_string())
        .collect())
}

#[wasm_bindgen]
pub fn convert_to_addrs(cidrs: Vec<String>) -> Result<Vec<String>, String> {
    let cidrs = cidrs
        .iter()
        .map(|l| l.trim().parse().map_err(Into::into))
        .collect::<anyhow::Result<Vec<Cidr>>>()
        .map_err(|e| e.to_string())?;

    Ok(to_addrs::cidrs_to_addrs(&cidrs)
        .into_iter()
        .map(|addr| addr.to_string())
        .collect())
}
