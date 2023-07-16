use clap::App;
use clap::Arg;
use clap::SubCommand;
use structopt::clap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cmdstack", about = "This is a note-taking app.")]
pub enum Cmd {
    Add {
        cmd: String,
        #[structopt(short)]
        tag: Option<String>,
        #[structopt(short)]
        note: Option<String>,
    },
    Update {
        id: Option<String>,
        #[structopt(short)]
        cmd: Option<String>,
        #[structopt(short)]
        tag: Option<String>,
        #[structopt(short)]
        note: Option<String>,
    },
    Delete {
        id: Option<String>,
    },
    Search {
        cmd: Option<String>,
        tag: Option<String>,
        note: Option<String>,
        id: Option<String>,
    },
}

pub fn parse_commands() {
    let matches = App::new("CmdStack")
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("cmd").required(true))
                .arg(Arg::with_name("tag").short("t").takes_value(true))
                .arg(Arg::with_name("note").short("n").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("update")
                .arg(Arg::with_name("id").takes_value(true))
                .arg(Arg::with_name("cmd").short("c").takes_value(true))
                .arg(Arg::with_name("tag").short("t").takes_value(true))
                .arg(Arg::with_name("note").short("n").takes_value(true)), // .before_help("Options:\n    -c, --cmd\t\tUpdates the command\n    -t, --tag\t\tUpdates the tag\n    -n, --note\t\tUpdates the note\n    id\t\t\tUpdates the ID"),
        )
        .subcommand(SubCommand::with_name("delete").arg(Arg::with_name("id").takes_value(true)))
        .subcommand(
            SubCommand::with_name("search")
                .arg(Arg::with_name("cmd").short("c").takes_value(true))
                .arg(Arg::with_name("tag").short("t").takes_value(true))
                .arg(Arg::with_name("note").short("n").takes_value(true))
                .arg(Arg::with_name("id").takes_value(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let cmd = sub_matches.value_of("cmd").unwrap().to_string();
            let tag = sub_matches.value_of("tag").map(|t| t.to_string());
            let note = sub_matches.value_of("note").map(|n| n.to_string());
            let cmd_enum = Cmd::Add { cmd, tag, note };
            // Process the Add variant of Cmd enum
            println!("{:?}", cmd_enum);
        }
        ("update", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").map(|i| i.to_string());
            let cmd = sub_matches.value_of("cmd").map(|c| c.to_string());
            let tag = sub_matches.value_of("tag").map(|t| t.to_string());
            let note = sub_matches.value_of("note").map(|n| n.to_string());

            if cmd.is_none() && tag.is_none() && note.is_none() && id.is_none() {
                println!("No update fields provided");
                return;
            } else if !id.is_none() && cmd.is_none() && tag.is_none() && note.is_none() {
                println!("Need at least 1 argument to update command");
                return;
            }

            let cmd_enum = Cmd::Update { id, cmd, tag, note };
            // Process the Update variant of Cmd enum
            println!("{:?}", cmd_enum);
        }
        ("delete", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").map(|i| i.to_string());
            let cmd_enum = Cmd::Delete { id };
            // Process the Delete variant of Cmd enum
            println!("{:?}", cmd_enum);
        }
        ("search", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").map(|i| i.to_string());
            let cmd = sub_matches.value_of("cmd").map(|c| c.to_string());
            let tag = sub_matches.value_of("tag").map(|t| t.to_string());
            let note = sub_matches.value_of("note").map(|n| n.to_string());

            if cmd.is_none() && tag.is_none() && note.is_none() && id.is_none() {
                println!("At least one search option is required");
                return;
            }

            let cmd_enum = Cmd::Search { id, cmd, tag, note };
            // Process the Search variant of Cmd enum
            println!("{:?}", cmd_enum);
        }
        _ => {
            // Handle invalid commands
            println!("Invalid command");
        }
    }
}
