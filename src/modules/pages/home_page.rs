use yew::prelude::*;

use crate::modules::{
    components::{headers, main_page},
    helpers::system_functions,
};

#[function_component(Home)]
pub fn home() -> Html {
    let location_name = use_state(|| "chennai".to_owned());
    let cloned_location_name = location_name.clone();
    let on_search = Callback::from(move |local_string| {
        cloned_location_name.set(local_string);
    });

    system_functions::get_locations();

    html! {
        <>
            <headers::Header search_string={on_search} />
            <main_page::MainPage loader={location_name.to_string()}/>
        </>
    }
}
