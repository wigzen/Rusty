use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
    <Header />
        <h3>{ "Hello World  " }</h3>
    <p> {"Vikas Lodh"} </p>
    <Footer/>
    </>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
     <>
     <h1>{ "Header" }</h1>
    </>
     }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
    <>

    {"adadd"}
        <h4>{ "Footer" }</h4>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
