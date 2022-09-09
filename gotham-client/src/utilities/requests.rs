// Gotham-city
//
// Copyright 2018 by Kzen Networks (kzencorp.com)
// Gotham city is free software: you can redistribute
// it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
use floating_duration::TimeFormat;
use log::info;
use reqwest::header::ACCEPT;
extern crate base64;
use base64::{decode};

use std::time::Instant;

use crate::ClientShim;

pub fn post<V>(client_shim: &ClientShim, path: &str) -> Option<V>
where
    V: serde::de::DeserializeOwned,
{
    _postb(client_shim, path, "{}")
}

pub fn postb<T, V>(client_shim: &ClientShim, path: &str, body: T) -> Option<V>
where
    T: serde::ser::Serialize,
    V: serde::de::DeserializeOwned,
{
    _postb(client_shim, path, body)
}

fn _postb<T, V>(client_shim: &ClientShim, path: &str, body: T) -> Option<V>
where
    T: serde::ser::Serialize,
    V: serde::de::DeserializeOwned,
{
    let start = Instant::now();

    let mut b = client_shim
        .client
        .post(&format!("{}/{}", client_shim.endpoint, path))
        .header(ACCEPT, "application/json");

    if client_shim.auth_token.is_some() {
        b = b.bearer_auth(client_shim.auth_token.clone().unwrap());
    }

    let res = b.json(&body).send();
    println!("response");
    info!("(req {}, took: {:?})", path, TimeFormat(start.elapsed()));

    let value = match res {
        Ok(mut v) => v.text().unwrap(),
        Err(_) => return None,
    };
    let decoded = decode(value.as_str());
    match decoded {
        Ok(v) => {
            let json_str = std::str::from_utf8(&v).unwrap();
            let max = std::cmp::min(json_str.len(), 1000);
            println!("parsing {}", &json_str[..max]);
            return Some(serde_json::from_str(json_str).unwrap())
        },
        Err(_) => return Some(serde_json::from_str(value.as_str()).unwrap()),
    }
}
