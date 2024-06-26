use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{CharacterSheet, SpellSheet, SelectDirectoryDialog};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/character_sheet")]
    CharacterSheet,
    #[at("/spells")]
    Spells,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <SelectDirectoryDialog on_close={Callback::from(|_| log::info!("Directory selected"))} /> },
        Route::CharacterSheet => html! { <CharacterSheet /> },
        Route::Spells => html! { <SpellSheet character_name="John Doe" /> },
    }
}