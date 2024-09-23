use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub search_string: Callback<String>,
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    let on_change = props.search_string.clone();
    let submit = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .set_value("");
        on_change.emit(value);
    });

    //my style section
    // let style_sheet = style!(
    //     r#"
    //
    //     "#r
    //     ).unwrap();

    html! {
        <div class={""} >
            <nav class="navbar navbar-expand-sm d-flex justify-space-between navbar-dark bg-dark">
                <a class="navbar-brand mx-2 d-flex align-items-center" href="#">
                    <img src="https://mohamedalosaili.github.io/weather-app/img/logo.png" class="mx-2" width="30" height="30" alt="" />
                    {"Weather App"}
                </a>
                <div class="w-100 d-flex justify-content-end">
                    <div class="form-inline d-flex flex-row p-2 h-50 input input-group-sm">
                        <input class="form-control mr-sm-2 mx-2 h-50" type="text" placeholder="Search" aria-label="Search" onchange={submit} />
                    </div>
                </div>
            </nav>
        </div>
    }
}
