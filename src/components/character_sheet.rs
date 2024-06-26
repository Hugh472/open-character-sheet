use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{AbilityCheckDialog, SkillCheckDialog, SaveCheckDialog};

#[function_component(CharacterSheet)]
pub fn character_sheet() -> Html {
    let show_ability_check_dialog = use_state(|| false);
    let show_skill_check_dialog = use_state(|| false);
    let show_save_check_dialog = use_state(|| false);

    let toggle_ability_check_dialog = {
        let show_ability_check_dialog = show_ability_check_dialog.clone();
        Callback::from(move |_| show_ability_check_dialog.set(!*show_ability_check_dialog))
    };

    let toggle_skill_check_dialog = {
        let show_skill_check_dialog = show_skill_check_dialog.clone();
        Callback::from(move |_| show_skill_check_dialog.set(!*show_skill_check_dialog))
    };

    let toggle_save_check_dialog = {
        let show_save_check_dialog = show_save_check_dialog.clone();
        Callback::from(move |_| show_save_check_dialog.set(!*show_save_check_dialog))
    };

    html! {
        <div>
            <h1>{ "Character Sheet" }</h1>
            <button onclick={toggle_ability_check_dialog.clone()}>{ "Ability Check" }</button>
            <button onclick={toggle_skill_check_dialog.clone()}>{ "Skill Check" }</button>
            <button onclick={toggle_save_check_dialog.clone()}>{ "Save Check" }</button>
            <button onclick={Callback::from(|_| log::info!("Save button clicked"))}>{ "Save" }</button>
            <Link<Route> to={Route::Spells}>{ "Spells" }</Link>
            
            if *show_ability_check_dialog {
                <AbilityCheckDialog on_close={toggle_ability_check_dialog} />
            }
            if *show_skill_check_dialog {
                <SkillCheckDialog on_close={toggle_skill_check_dialog} />
            }
            if *show_save_check_dialog {
                <SaveCheckDialog on_close={toggle_save_check_dialog} />
            }
            // Character sheet content
        </div>
    }
}