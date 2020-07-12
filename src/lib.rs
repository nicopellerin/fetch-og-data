#[allow(unused_variables)]
extern crate wasm_bindgen;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct OgData {
    title: String,
    desc: String,
    image: String,
}

#[wasm_bindgen]
pub fn get_og_data(html: String) -> JsValue {
    let fragment = Html::parse_fragment(html.as_str());
    let og_image = Selector::parse("meta[property='og:image']").unwrap();
    let og_title = Selector::parse("meta[property='og:title']").unwrap();
    let og_description = Selector::parse("meta[property='og:description']").unwrap();

    let image = fragment.select(&og_image).next().unwrap();
    let title = fragment.select(&og_title).next().unwrap();
    let desc = fragment.select(&og_description).next().unwrap();

    let og_data = OgData {
        title: String::from(title.value().attr("content").unwrap()),
        desc: String::from(desc.value().attr("content").unwrap()),
        image: String::from(image.value().attr("content").unwrap()),
    };

    let og_data_json = serde_json::to_string(&og_data).unwrap();

    JsValue::from(og_data_json)
}
