use serenity::all::ButtonStyle;
use serenity::all::CreateActionRow;
use serenity::all::CreateButton;
use serenity::all::CreateInteractionResponseMessage;
use serenity::builder::CreateCommand;
use serenity::builder::CreateEmbed;
use serenity::model::application::ResolvedOption;


pub async fn run(_options: &[ResolvedOption<'_>]) -> CreateInteractionResponseMessage {
    // let url = "http://localhost:8000/api/v1/accounts/123";
    // let res = fetch_data(GET, url, None).await;

    // println!("{:?}", res);

    // match res {
    //     Ok(_) => {
    //         return linked();
    //     }
    //     Err(NoData) => {
    //         return not_linked();
    //     }
    //     Err(_) => {
    //         return CreateInteractionResponseMessage::new()
    //             .content("An error occurred while fetching data.")
    //             .ephemeral(true);
    //     }
    // }
    CreateInteractionResponseMessage::new()
        .content("Panel")
        .ephemeral(true)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("panel").description("Open the panel")
}

fn not_linked() -> CreateInteractionResponseMessage {
    let url = "http://localhost:8080/";

    let button = CreateButton::new_link(url)
        .label("Link Account")
        .style(ButtonStyle::Primary);

    let button_row = CreateActionRow::Buttons(vec![button]);

    let embed = CreateEmbed::new()
        .title("Link Your Account")
        .description("Your Discord account is not linked to any account. Please link your account to continue. Paste this code: $gen into your account link settings.")
        .color(0x00FF00);
    
    let message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .components(vec![button_row])
        .ephemeral(true);

    message
}

fn linked() -> CreateInteractionResponseMessage {
    let url = "http://localhost:8000/api/v1/auth/sign_in";

    let button = CreateButton::new_link(url)
        .label("Sign In")
        .style(ButtonStyle::Primary);

    let button_row = CreateActionRow::Buttons(vec![button]);

    let embed = CreateEmbed::new()
        .title("Sign In")
        .description("Please fill in the required information to sign in.")
        .color(0x00FF00);
    
    let message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .components(vec![button_row]);
    
    message
}