use yew::{html, function_component, Callback, InputEvent};

fn main() {
    yew::Renderer::<App>();
}

#[function_component(App)]
fn app() -> Html {
let handle_input: Callback<InputEvent> = Callback::from(|_| {});

    html! {
    <main>
    <div>
    {"Ingresa una elemento"}
    </div>
    <div>
    <input type="text" oninput="{handle_input}"/>
    </div>
    <div><button>{"Agregar"}</button></div>
    <div><button>{"Editar"}</button></div>
    <div><button>{"Eliminar"}</button></div>
    </main>
    
    }
}