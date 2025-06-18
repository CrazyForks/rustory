use clap::{Arg, Command};
use rustory::commands::*;
use std::path::PathBuf;

fn main() {
    let app = Command::new("rustory")
        .version("0.1.0")
        .about("A lightweight local version management tool written in Rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new rustory repository")
                .arg(
                    Arg::new("path")
                        .help("Path to initialize (default: current directory)")
                        .value_parser(clap::value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("commit")
                .about("Create a new snapshot")
                .alias("log")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("Commit message")
                        .value_name("MSG")
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("history")
                .about("Show snapshot history")
                .alias("list")
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("status")
                .about("Show working directory status")
        )
        .subcommand(
            Command::new("diff")
                .about("Show differences between snapshots or working directory")
                .arg(
                    Arg::new("id1")
                        .help("First snapshot ID")
                        .value_name("ID1")
                )
                .arg(
                    Arg::new("id2")
                        .help("Second snapshot ID")
                        .value_name("ID2")
                )
        )
        .subcommand(
            Command::new("rollback")
                .about("Rollback to a previous snapshot")
                .arg(
                    Arg::new("id")
                        .help("Snapshot ID to rollback to")
                        .required(true)
                        .value_name("ID")
                )
                .arg(
                    Arg::new("restore")
                        .long("restore")
                        .help("Directly restore to working directory")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("keep-index")
                        .long("keep-index")
                        .help("Don't update index.json")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("tag")
                .about("Tag a snapshot")
                .arg(
                    Arg::new("name")
                        .help("Tag name")
                        .required(true)
                        .value_name("NAME")
                )
                .arg(
                    Arg::new("id")
                        .help("Snapshot ID to tag")
                        .required(true)
                        .value_name("ID")
                )
        )
        .subcommand(
            Command::new("ignore")
                .about("Manage ignore rules")
                .arg(
                    Arg::new("action")
                        .help("Action: show, edit")
                        .value_name("ACTION")
                )
        )
        .subcommand(
            Command::new("config")
                .about("Get or set configuration options")
                .arg(
                    Arg::new("action")
                        .help("Action: get, set")
                        .required(true)
                        .value_name("ACTION")
                )
                .arg(
                    Arg::new("key")
                        .help("Configuration key")
                        .required(true)
                        .value_name("KEY")
                )
                .arg(
                    Arg::new("value")
                        .help("Configuration value (for set action)")
                        .value_name("VALUE")
                )
        )
        .subcommand(
            Command::new("gc")
                .about("Cleanup unnecessary files and optimize the repository")
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Show what would be removed without actually removing")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("aggressive")
                        .long("aggressive")
                        .help("Perform more aggressive cleanup and optimization")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("prune-expired")
                        .long("prune-expired")
                        .help("Remove snapshots older than configured retention period")
                        .action(clap::ArgAction::SetTrue)
                )
        );

    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("path").cloned();
            InitCommand::execute(path)
        }
        Some(("commit", sub_matches)) => {
            let message = sub_matches.get_one::<String>("message").cloned();
            let json = sub_matches.get_flag("json");
            CommitCommand::execute(message, json)
        }
        Some(("history", sub_matches)) => {
            let json = sub_matches.get_flag("json");
            HistoryCommand::execute(json)
        }
        Some(("status", _)) => {
            StatusCommand::execute()
        }
        Some(("diff", sub_matches)) => {
            let id1 = sub_matches.get_one::<String>("id1").cloned();
            let id2 = sub_matches.get_one::<String>("id2").cloned();
            DiffCommand::execute(id1, id2)
        }
        Some(("rollback", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap().clone();
            let restore = sub_matches.get_flag("restore");
            let keep_index = sub_matches.get_flag("keep-index");
            RollbackCommand::execute(id, restore, keep_index)
        }
        Some(("tag", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().clone();
            let id = sub_matches.get_one::<String>("id").unwrap().clone();
            TagCommand::execute(name, id)
        }
        Some(("ignore", sub_matches)) => {
            let action = sub_matches.get_one::<String>("action").cloned();
            IgnoreCommand::execute(action)
        }
        Some(("config", sub_matches)) => {
            let action = sub_matches.get_one::<String>("action").unwrap().clone();
            let key = sub_matches.get_one::<String>("key").unwrap().clone();
            let value = sub_matches.get_one::<String>("value").cloned();
            ConfigCommand::execute(action, key, value)
        }
        Some(("gc", sub_matches)) => {
            let dry_run = sub_matches.get_flag("dry-run");
            let aggressive = sub_matches.get_flag("aggressive");
            let prune_expired = sub_matches.get_flag("prune-expired");
            GcCommand::execute(dry_run, aggressive, prune_expired)
        }
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            std::process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
