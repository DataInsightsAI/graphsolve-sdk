//! `graphsolve` — a command-line client for the GraphSolve engine API.
//!
//! A credentialled client, not the engine. It contains no physics. It reads
//! `GRAPHSOLVE_API_KEY` and makes the same authenticated HTTPS calls as the
//! library, and every call is metered and billed identically. Without a key it
//! prints help and does nothing.
//!
//! Results go to stdout as JSON so the output pipes into `jq`. What a call cost
//! goes to stderr.

use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use graphsolve::{GraphSolve, ToolResponse, TOOL_NAMES};

#[derive(Parser)]
#[command(
    name = "graphsolve",
    version,
    about = "Client for the GraphSolve engine API.",
    long_about = "Client for the GraphSolve engine API.\n\n\
                  Reads the API key from GRAPHSOLVE_API_KEY. Arguments are a JSON \
                  object, read from a file or from stdin.\n\n\
                  This is a client, not the engine: every call is an authenticated \
                  request to the hosted service, and every call is metered.",
    // Lets `graphsolve solve_network model.json` work as well as the explicit
    // `graphsolve call solve_network model.json`.
    allow_external_subcommands = true
)]
struct Cli {
    /// API key. Prefer the environment variable: a key passed here lands in
    /// your shell history and in `ps` output.
    #[arg(
        long,
        global = true,
        env = "GRAPHSOLVE_API_KEY",
        hide_env_values = true
    )]
    api_key: Option<String>,

    /// Override the engine API, for a non-production deployment.
    #[arg(long, global = true, env = "GRAPHSOLVE_BASE_URL")]
    base_url: Option<String>,

    /// Override the token endpoint. Set it alongside --base-url: it is never
    /// derived from it.
    #[arg(long, global = true, env = "GRAPHSOLVE_TOKEN_URL")]
    token_url: Option<String>,

    /// Print the whole response envelope, not just `result`.
    #[arg(long, global = true)]
    envelope: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List every tool with its domain, summary and price.
    Tools {
        /// Only tools in this domain.
        #[arg(long)]
        domain: Option<String>,
    },
    /// Print one tool's full definition: schema, examples, price.
    Describe { tool: String },
    /// Run a tool and print its result.
    Call {
        tool: String,
        /// A JSON file of arguments. Reads stdin when omitted.
        file: Option<PathBuf>,
    },
    /// Price a long-running call without running it. Spends nothing.
    Quote { tool: String, file: Option<PathBuf> },
    /// Submit a long-running call as a job and wait for it.
    Run {
        tool: String,
        file: Option<PathBuf>,
        /// How often to poll, in seconds.
        #[arg(long, default_value = "5")]
        poll: u64,
        /// Give up and cancel after this many seconds.
        #[arg(long)]
        max_wait: Option<u64>,
    },
    /// Show the identity and scopes this API key's token carries.
    #[command(visible_alias = "whoami")]
    Me,

    /// `graphsolve <tool> [file]` — sugar for `graphsolve call <tool> [file]`.
    #[command(external_subcommand)]
    Tool(Vec<String>),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut builder = GraphSolve::builder();
    if let Some(key) = &cli.api_key {
        builder = builder.api_key(key);
    }
    if let Some(url) = &cli.base_url {
        builder = builder.base_url(url);
    }
    if let Some(url) = &cli.token_url {
        builder = builder.token_url(url);
    }
    let gs = builder
        .build()
        .context("no API key: set GRAPHSOLVE_API_KEY or pass --api-key")?;

    match cli.command {
        Command::Tools { domain } => {
            let mut tools = gs.tools().await?;
            if let Some(domain) = domain {
                tools.retain(|t| t.domain == domain);
            }
            for tool in &tools {
                let price = if tool.cost.base_credits == 0 {
                    "free".to_owned()
                } else {
                    format!("{} cr", tool.cost.base_credits)
                };
                println!(
                    "{:<38} {:<14} {:>7}  {}",
                    tool.name, tool.domain, price, tool.summary
                );
            }
            eprintln!("\n{} tools", tools.len());
        }

        Command::Describe { tool } => {
            print_json(&gs.describe(&tool).await?)?;
        }

        Command::Me => {
            print_json(&gs.me().await?)?;
        }

        Command::Call { tool, file } => {
            check_tool(&tool)?;
            let response = gs.call(&tool, &read_args(file.as_deref())?).await?;
            report(&response, cli.envelope.into())?;
        }

        Command::Tool(argv) => {
            let (tool, rest) = argv.split_first().expect("clap guarantees a name");
            check_tool(tool)?;
            if rest.len() > 1 {
                bail!("expected at most one arguments file, got {}", rest.len());
            }
            let file = rest.first().map(PathBuf::from);
            let response = gs.call(tool, &read_args(file.as_deref())?).await?;
            report(&response, cli.envelope.into())?;
        }

        Command::Quote { tool, file } => {
            check_tool(&tool)?;
            let quote = gs.quote(&tool, &read_args(file.as_deref())?).await?;
            print_json(&quote)?;
        }

        Command::Run {
            tool,
            file,
            poll,
            max_wait,
        } => {
            check_tool(&tool)?;
            let response = gs
                .run(
                    &tool,
                    &read_args(file.as_deref())?,
                    Duration::from_secs(poll),
                    max_wait.map(Duration::from_secs),
                )
                .await?;
            report(&response, cli.envelope.into())?;
        }
    }

    Ok(())
}

/// Check the tool name locally rather than spending a round trip on a typo.
fn check_tool(tool: &str) -> Result<()> {
    if TOOL_NAMES.contains(&tool) {
        return Ok(());
    }
    let near: Vec<&str> = TOOL_NAMES
        .iter()
        .copied()
        .filter(|name| name.contains(tool) || tool.contains(name))
        .take(5)
        .collect();
    if near.is_empty() {
        bail!("no tool called `{tool}`. Run `graphsolve tools` for the list.");
    }
    bail!(
        "no tool called `{tool}`. Did you mean: {}?",
        near.join(", ")
    );
}

fn read_args(file: Option<&std::path::Path>) -> Result<serde_json::Value> {
    let text = match file {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display()))?,
        None => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .context("could not read arguments from stdin")?;
            buffer
        }
    };
    if text.trim().is_empty() {
        return Ok(serde_json::json!({}));
    }
    serde_json::from_str(&text).context("arguments are not valid JSON")
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

/// How much of the response to print.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Output {
    /// Just the tool's own payload.
    Result,
    /// The whole envelope: warnings, failure kind, billing.
    Envelope,
}

impl From<bool> for Output {
    fn from(envelope: bool) -> Self {
        if envelope {
            Output::Envelope
        } else {
            Output::Result
        }
    }
}

/// Result to stdout, cost to stderr, so a pipe into `jq` gets clean JSON.
fn report(response: &ToolResponse, output: Output) -> Result<()> {
    match output {
        Output::Envelope => print_json(response)?,
        Output::Result => print_json(&response.result)?,
    }
    for warning in &response.warnings {
        eprintln!("warning: {warning}");
    }
    if let Some(charged) = response.credits_charged() {
        eprintln!("charged {charged} credits");
    }
    Ok(())
}
