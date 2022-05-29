use std::process;
use xtask_wasm::{anyhow::Result, clap, clap::Parser};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long = "log", default_value = "Info")]
    log_level: log::LevelFilter,
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Dist(Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

#[derive(Debug, Parser)]
struct Dist {
    /// Optimize the generated package using `wasm-opt`.
    #[clap(long)]
    optimize: bool,

    #[clap(flatten)]
    base: xtask_wasm::Dist,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .filter(Some("xtask"), cli.log_level)
        .init();

    match cli.cmd {
        SubCommand::Dist(arg) => {
            log::info!("Generating package...");

            let dist_result = arg
                .base
                .static_dir_path("static")
                .run("dominator-example")?;

            if arg.optimize {
                xtask_wasm::WasmOpt::level(1)
                    .shrink(2)
                    .optimize(dist_result.wasm)?;
            }
        }
        SubCommand::Watch(arg) => {
            log::info!("Watching for changes and check...");

            let mut command = process::Command::new("cargo");
            command.arg("check");

            arg.run(command)?;
        }
        SubCommand::Start(arg) => {
            arg.arg("dist").start(xtask_wasm::default_dist_dir(false))?;
        }
    }

    Ok(())
}
