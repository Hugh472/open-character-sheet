use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectDirectoryDialogProps {
    pub on_close: Callback<()>,
}

#[function_component(SelectDirectoryDialog)]
pub fn select_directory_dialog(props: &SelectDirectoryDialogProps) -> Html {
    // Directory selection logic

    html! {
        <div>
            <h2>{ "Select Directory" }</h2>
            <button onclick={props.on_close.clone()}>{ "Close" }</button>
        </div>
    }
}