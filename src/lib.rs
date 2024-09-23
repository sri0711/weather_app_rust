use yew::prelude::*;
use yew_bootstrap::util::{include_cdn, include_cdn_js};

pub mod modules;

use modules::pages::home_page;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            {include_cdn()}
                <home_page::Home/>
            {include_cdn_js()}
        </div>
    }
}
