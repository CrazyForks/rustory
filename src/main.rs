use clap::{Arg, Command};
use rustory::commands::*;
use std::path::PathBuf;

fn main() {
    let app = Command::new("rustory")
        .version("0.1.3")
        .about("A lightweight local version management tool written in Rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialize a new rustory repository")
                .arg(
                    Arg::new("path")
                        .help("Path to initialize (default: current directory)")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Create a new snapshot")
                .alias("commit")
                .alias("log")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("Commit message")
                        .value_name("MSG"),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("history")
                .about("Show snapshot history")
                .alias("list")
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("status")
                .about("Show working directory status")
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show verbose output")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("diff")
                .about("Show differences between snapshots or working directory")
                .arg(Arg::new("id1").help("First snapshot ID").value_name("ID1"))
                .arg(Arg::new("id2").help("Second snapshot ID").value_name("ID2")),
        )
        .subcommand(
            Command::new("back")
                .about("Rollback to a previous snapshot")
                .alias("rollback")
                .arg(
                    Arg::new("id")
                        .help("Snapshot ID to rollback to")
                        .required(true)
                        .value_name("ID"),
                )
                .arg(
                    Arg::new("restore")
                        .long("restore")
                        .help("Directly restore to working directory")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("keep-index")
                        .long("keep-index")
                        .help("Don't update index.json")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("tag")
                .about("Tag a snapshot")
                .arg(
                    Arg::new("name")
                        .help("Tag name")
                        .required(true)
                        .value_name("NAME"),
                )
                .arg(
                    Arg::new("id")
                        .help("Snapshot ID to tag")
                        .required(true)
                        .value_name("ID"),
                ),
        )
        .subcommand(
            Command::new("ignore").about("Manage ignore rules").arg(
                Arg::new("action")
                    .help("Action: show, edit")
                    .value_name("ACTION"),
            ),
        )
        .subcommand(
            Command::new("config")
                .about("Get or set configuration options")
                .arg(
                    Arg::new("action")
                        .help("Action: get, set")
                        .required(true)
                        .value_name("ACTION"),
                )
                .arg(
                    Arg::new("key")
                        .help("Configuration key")
                        .required(true)
                        .value_name("KEY"),
                )
                .arg(
                    Arg::new("value")
                        .help("Configuration value (for set action)")
                        .value_name("VALUE"),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove snapshots or cleanup unnecessary files")
                .alias("gc")
                .arg(
                    Arg::new("target")
                        .help("Snapshot number/ID to remove, or range (e.g., 1-3, abc123-def456)")
                        .value_name("TARGET"),
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .help("Show what would be removed without actually removing")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("aggressive")
                        .long("aggressive")
                        .help("Perform more aggressive cleanup and optimization")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("prune-expired")
                        .long("prune-expired")
                        .help("Remove snapshots older than configured retention period")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("stats")
                .about("Show repository statistics")
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help("Output in JSON format")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify repository integrity")
                .arg(
                    Arg::new("fix")
                        .long("fix")
                        .help("Attempt to fix integrity issues")
                        .action(clap::ArgAction::SetTrue),
                ),
        );

    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("path").cloned();
            InitCommand::execute(path)
        }
        Some(("add", sub_matches)) | Some(("commit", sub_matches)) => {
            let message = sub_matches.get_one::<String>("message").cloned();
            let json = sub_matches.get_flag("json");
            AddCommand::execute(message, json)
        }
        Some(("history", sub_matches)) => {
            let json = sub_matches.get_flag("json");
            HistoryCommand::execute(json)
        }
        Some(("status", sub_matches)) => {
            let verbose = sub_matches.get_flag("verbose");
            let json = sub_matches.get_flag("json");
            StatusCommand::execute(verbose, json)
        }
        Some(("diff", sub_matches)) => {
            let id1 = sub_matches.get_one::<String>("id1").cloned();
            let id2 = sub_matches.get_one::<String>("id2").cloned();
            DiffCommand::execute(id1, id2)
        }
        Some(("back", sub_matches)) | Some(("rollback", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap().clone();
            let restore = sub_matches.get_flag("restore");
            let keep_index = sub_matches.get_flag("keep-index");
            BackCommand::execute(id, restore, keep_index)
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
        Some(("rm", sub_matches)) | Some(("gc", sub_matches)) => {
            let target = sub_matches.get_one::<String>("target").cloned();
            let dry_run = sub_matches.get_flag("dry-run");
            let aggressive = sub_matches.get_flag("aggressive");
            let prune_expired = sub_matches.get_flag("prune-expired");

            if let Some(target) = target {
                UtilsCommand::remove_snapshots(target, dry_run)
            } else {
                UtilsCommand::gc(dry_run, aggressive, prune_expired)
            }
        }
        Some(("stats", sub_matches)) => {
            let json = sub_matches.get_flag("json");
            UtilsCommand::stats(json)
        }
        Some(("verify", sub_matches)) => {
            let fix = sub_matches.get_flag("fix");
            UtilsCommand::verify(fix)
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
