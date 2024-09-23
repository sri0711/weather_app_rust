use gloo::console::log;
use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use crate::modules::helpers::network_call::*;

const STYLE_FILE: &str = include_str!("mainPage.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub loader: String,
}

#[styled_component(MainPage)]
pub fn main_page(props: &Props) -> Html {
    //	style sheet
    let custom_style = Style::new(STYLE_FILE).unwrap();
    let place = props.loader.clone();

    /*
    main url parameters
    lat=13.020725741054365&lon=80.18229792040938&limit=1&appid=3f04ddd0162fe6a258a0cb36c44b305b
     */

    {
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let url = query_generator(QueryGeneratorOptions {
                    url: "https://api.openweathermap.org/geo/1.0/reverse".to_string(),
                    query: vec![QueryOptionValues {
                        key: Some("appid".to_string()),
                        value: Some("".to_string()),
                    }],
                });
                let response = Request::get(&url).send().await.unwrap();
                log!(response.status());
            });
        });
    }
    // const appid: &str = "3f04ddd0162fe6a258a0cb36c44b305b";
    // let mut url = Url::parse("https://api.openweathermap.org/geo/1.0/reverse")?;
    // log!(url.to_string());
    html! {
        <div class={custom_style} >
            <div class="main">
                <p>{"this is an welcome page"}</p>
                <p>{place}</p>
            </div>
        </div>
    }
}
