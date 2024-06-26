use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SkillCheckDialogProps {
    pub on_close: Callback<()>,
}

#[function_component(SkillCheckDialog)]
pub fn skill_check_dialog(props: &SkillCheckDialogProps) -> Html {
    let roll_result = use_state(|| None);

    let roll_d20 = {
        let roll_result = roll_result.clone();
        Callback::from(move |_| {
            let roll = rand::random::<u8>() % 20 + 1;
            roll_result.set(Some(roll));
        })
    };

    html! {
        <div>
            <h2>{ "Skill Check" }</h2>
            <p>{ format!("Roll result: {:?}", *roll_result) }</p>
            <button onclick={roll_d20}>{ "Roll D20" }</button>
            <button onclick={props.on_close.clone()}>{ "Close" }</button>
        </div>
    }
}