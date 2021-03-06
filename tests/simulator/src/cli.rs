use clap::{App, Arg, SubCommand};

pub fn cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("simulator")
        .version(crate_version!())
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Options for interacting with simulator")
        .subcommand(
            SubCommand::with_name("beacon-chain-sim")
            .about(
                "Lighthouse Beacon Chain Simulator creates `n` beacon node and validator clients, \
                    each with `v` validators. A deposit contract is deployed at the start of the \
                    simulation using a local `ganache-cli` instance (you must have `ganache-cli` \
                    installed and avaliable on your path). All beacon nodes independently listen \
                    for genesis from the deposit contract, then start operating. \

                    As the simulation runs, there are checks made to ensure that all components \
                    are running correctly. If any of these checks fail, the simulation will \
                    exit immediately.",
                    )
                    .arg(Arg::with_name("nodes")
                        .short("n")
                        .long("nodes")
                        .takes_value(true)
                        .help("Number of beacon nodes (default 4)"))
                    .arg(Arg::with_name("validators_per_node")
                        .short("v")
                        .long("validators_per_node")
                        .takes_value(true)
                        .help("Number of validators (default 20)"))
                    .arg(Arg::with_name("speed_up_factor")
                        .short("s")
                        .long("speed_up_factor")
                        .takes_value(true)
                        .help("Speed up factor (default 4)"))
                    .arg(Arg::with_name("end_after_checks")
                        .short("e")
                        .long("end_after_checks")
                        .takes_value(false)
                        .help("End after checks (default true)"))
        )
        .subcommand(
            SubCommand::with_name("syncing-sim")
                .about("Run the syncing simulation")
                .arg(
                    Arg::with_name("speedup")
                        .short("s")
                        .long("speedup")
                        .takes_value(true)
                        .help("Speed up factor for eth1 blocks and slot production (default 15)"),
                )
                .arg(
                    Arg::with_name("initial_delay")
                        .short("i")
                        .long("initial_delay")
                        .takes_value(true)
                        .help("Epoch delay for new beacon node to start syncing (default 50)"),
                )
                .arg(
                    Arg::with_name("sync_delay")
                        .long("sync_delay")
                        .takes_value(true)
                        .help("Epoch delay for newly added beacon nodes get synced (default 10)"),
                )
                .arg(
                    Arg::with_name("strategy")
                        .long("strategy")
                        .takes_value(true)
                        .possible_values(&["one-node", "two-nodes", "mixed", "all"])
                        .help("Sync strategy to run. (default all)"),
                ),
        )
}
