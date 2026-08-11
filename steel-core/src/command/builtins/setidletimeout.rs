//! The vanilla `/setidletimeout` command.

use text_components::TextComponent;
use steel_utils::Identifier;
use steel_utils::translations::{COMMANDS_SETIDLETIMEOUT_SUCCESS, COMMANDS_SETIDLETIMEOUT_SUCCESS_DISABLED};
use crate::command::brigadier::{ArgumentType, CommandNodeBuilder, CommandSyntaxError};
use crate::command::execution::{argument, literal, CommandSource, SteelCommandContext, SteelCommandRuntime};
use crate::command::registration::CommandRegistration;

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("setidletimeout"), |_| command())
}

fn command() -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    literal("setidletimeout").then(
        argument("minutes", ArgumentType::integer(0, i32::MAX)).executes(set_idle_timeout),
    )
}

fn set_idle_timeout(context: &SteelCommandContext<CommandSource>) -> Result<i32, CommandSyntaxError> {
    let Some(time) = context.integer("minutes") else {
        return Err(missing_argument("pos"));
    };
    context.source().server().player_idle_timeout = time;

    if time > 0 {
        context.source().send_success(&COMMANDS_SETIDLETIMEOUT_SUCCESS.message([TextComponent::plain(format!("{}", time))]).component(), true);
    } else {
        context.source().send_success(&TextComponent::from(&COMMANDS_SETIDLETIMEOUT_SUCCESS_DISABLED), true);
    }

    Ok(1)
}
fn missing_argument(name: &str) -> CommandSyntaxError {
    CommandSyntaxError::dynamic(format!(
        "Parsed value for {name} is missing from the command context"
    ))
}
