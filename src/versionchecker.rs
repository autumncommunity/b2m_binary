use crate::print;
use gmod::lua::State;
use reqwest::{blocking::*, header::USER_AGENT};
use serde_json::{from_str, Value};

pub unsafe fn is_newest(lua: State, client: &Client) -> bool {
    let res = client
        .get("https://api.github.com/repos/autumncommunity/b2m/releases/latest")
        .header(USER_AGENT, "b2m_binary")
        .send()
        .expect("couldn't send http request");

    if !res.status().is_success() {
        println!("Checking version failed. Error: {}", res.text().unwrap());

        return false;
    }

    let text: String = res.text().unwrap();
    let json: Value = from_str(&text).unwrap();
    let version: String = json.get("name").unwrap().to_string().replace("\"", "");
    let current_version: String = env!("CARGO_PKG_VERSION").to_string();

    let is_newest: bool = version == current_version;

    if !is_newest {
        print(
            lua,
            "You using outdated version of B2M.\n\n\t> Download update in https://github.com/autumncommunity/b2m\n",
        );
    }

    return is_newest;
}
