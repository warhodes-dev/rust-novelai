use std::io::{self, Write};
use colored::Colorize;
use novelai::{
    api,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let token_path = std::path::Path::new("./access.token");
    let access_token = if token_path.exists() {
        std::fs::read_to_string(token_path)?
    } else {
        let email = prompt_line("Enter email: ")?;
        let password = rpassword::prompt_password("Enter password: ")?;
        let access_key = novelai::utils::get_access_key(&email, &password)?;
        let access_token = api::user::login(access_key).await?;
        let mut token_file = std::fs::File::create(token_path)?;
        token_file.write_all(access_token.as_bytes())?;
        access_token
    };
    
    let params = api::ai::AiGenerateParameters{
        min_length: 1.0,
        max_length: 40.0,
        use_string: Some(true),

        early_stopping: Some(true),

        temperature: Some(1.348),
        top_k: Some(64.0),
        top_p: Some(0.909),
        top_a: Some(1.0),
        typical_p: Some(1.0),
        tail_free_sampling: Some(0.688),
        repetition_penalty: Some(4.967),
        repetition_penalty_range: Some(2048.0),
        repetition_penalty_slope: Some(0.09),

        ..Default::default()
    };

    let model = api::ai::AiModel::Euterpe;

    let memory = "";
    let prompt = "You are walking through a calm forest when you hear a soft rustling coming from behind a nearby tree.";

    let mut text_buf = String::from(prompt);

    println!("{prompt}");

    println!("{}", "\nTip: Try to phrase your response in the second person, and be specific.\n\
                      For example: \"> You crane your neck to get a better look at the nearby brush\"".cyan());

    loop {
        //let action = prompt_line("\n> ")?;
        let action = prompt_line("\n> ")?;
        text_buf.push_str(&action);

        let input = format!("{memory}{text_buf}");
        let output = api::ai::generate(&input, &model, &params).await?.output.unwrap();
        text_buf.push_str(&output);

        println!("{action}{output}");
    }
}


fn read_line() -> Result<String, io::Error> {
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn prompt_line(s: &str) -> Result<String, io::Error> {
    print!("{}", s);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

/*
   fn prompt_action() -> Result<String, Box<dyn std::error::Error>> {
   use termion::{color, style};
   enum Action {
   Do,
   Say,
   Story,
   }
   impl std::fmt::Display for Action {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
   let prompt = match self {
   Action::Do => format!("{} DO  {}", color::Fg(color::Red), color::Fg(color::Reset)),
   Action::Say => format!("{} SAY {}", color::Fg(color::Magenta), color::Fg(color::Reset)),
   Action::Story => format!("{}STORY{}", color::Fg(color::Blue), color::Fg(color::Reset)),
   };
   write!(f, "[{prompt}]: ")
   }
   }

   let mut current_action = Action::Story;

   loop {
   let input = prompt_line(&current_action.to_string())?;
   if input.is_empty() {
   continue;
   } else if input.starts_with('/') {
   current_action = match input.as_str() {
   "/say" => Action::Say,
   "/do" => Action::Do,
   "/story" => Action::Story,
   _ => current_action,
   };
   continue;
   }

   let response = match current_action {
   Action::Do => format!("You {input}"),
   Action::Say => format!("You say \"{input}\""),
   Action::Story => format!("{input}"),
   };
   return Ok(response);
   }
   }
   */
