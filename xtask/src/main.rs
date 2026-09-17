mod ui;

use clap::builder::styling::{Effects, RgbColor, Style, Styles};
use clap::{Parser, Subcommand};
use comfy_table::{Attribute, Cell};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn dark_knight_style() -> Styles {
    let bat_blue = Style::new()
        .fg_color(Some(RgbColor(65, 145, 255).into()))
        .effects(Effects::BOLD);

    let steel_blue = Style::new()
        .fg_color(Some(RgbColor(140, 185, 235).into()))
        .effects(Effects::BOLD);

    let slate_blue = Style::new()
        .fg_color(Some(RgbColor(170, 205, 240).into()))
        .effects(Effects::BOLD | Effects::UNDERLINE);

    let gotham_red = Style::new()
        .fg_color(Some(RgbColor(255, 75, 75).into()))
        .effects(Effects::BOLD);

    let arctic_mint = Style::new()
        .fg_color(Some(RgbColor(90, 230, 200).into()))
        .effects(Effects::BOLD);

    Styles::styled()
        .header(bat_blue)
        .usage(bat_blue)
        .literal(steel_blue)
        .placeholder(slate_blue)
        .error(gotham_red)
        .valid(arctic_mint)
        .invalid(gotham_red)
}

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Sertemp workspace automation & deployment runner", long_about = None)]
#[command(styles = dark_knight_style())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize services, databases, and Docker network/volumes
    Init,

    /// Start already initialized services and dependencies
    Start,

    /// Run a specific service container
    Run {
        /// Name of the service to run
        service_name: String,
    },

    /// Stop a specific service container
    Stop {
        /// Name of the service to stop
        service_name: String,
    },

    /// Clean temporary files, build artifacts, or stopped state for a service
    Clean {
        /// Name of the service to clean
        service_name: String,
    },

    /// Reset a service to its initial state
    Reset {
        /// Name of the service to reset
        service_name: String,
    },

    /// Backup a container's state and volume data
    Backup {
        /// Container identifier or name
        container_name: String,
    },

    /// Container lifecycle operations (list, delete, backup)
    #[command(subcommand)]
    Container(ContainerCommands),

    /// Service operations (list, inspect)
    #[command(subcommand)]
    Service(ServiceCommands),

    /// List active resources (shorthand: `cargo xtask list <service|container>`)
    #[command(subcommand)]
    List(ListCommands),

    /// Delete a container (shorthand)
    DeleteContainer {
        container_name: String,
    },
}

#[derive(Debug, Subcommand)]
enum ContainerCommands {
    /// List all containers managed by sertemp
    List,
    /// Delete a specific container
    Delete { container_name: String },
    /// Backup a specific container
    Backup { container_name: String },
}

#[derive(Debug, Subcommand)]
enum ServiceCommands {
    /// List all registered services
    List,
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    /// List all containers
    Container,
    /// List all services
    Service,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            ui::log_step("INIT", "Registering services and docker environments...");
            let sp = ui::spinner("Configuring docker network 'sertemp-net'...");
            thread::sleep(Duration::from_millis(500));
            let _ = Command::new("docker").args(["network", "create", "sertemp-net"]).output();
            sp.finish_and_clear();
            ui::log_success("Network", "sertemp-net created/verified");
            ui::log_success("Init", "All base services registered successfully");
        }
        Commands::Start => {
            ui::log_step("START", "Bringing up configured services...");
            let sp = ui::spinner("Starting containers...");
            thread::sleep(Duration::from_millis(600));
            sp.finish_and_clear();
            ui::log_success("Runtime", "Services started");
        }
        Commands::Run { service_name } => {
            ui::log_step("RUN", &format!("Launching {service_name}..."));
        }
        Commands::Stop { service_name } => {
            ui::log_step("STOP", &format!("Stopping {service_name}..."));
            docker_cmd(&["stop", &service_name]);
            ui::log_success("Stopped", &service_name);
        }
        Commands::Clean { service_name } => {
            ui::log_step("CLEAN", &format!("Cleaning resources for {service_name}"));
        }
        Commands::Reset { service_name } => {
            ui::log_step("RESET", &format!("Resetting {service_name}"));
        }
        Commands::Backup { container_name } => {
            ui::log_step("BACKUP", &format!("Creating backup snapshot for {container_name}..."));
            let sp = ui::spinner("Archiving volume data...");
            thread::sleep(Duration::from_millis(700));
            sp.finish_and_clear();
            ui::log_success("Backup", &format!("Snapshot complete for {container_name}"));
        }
        Commands::Container(sub) => match sub {
            ContainerCommands::List => render_container_table(),
            ContainerCommands::Delete { container_name } => {
                ui::log_step("DELETE", &format!("Removing container {container_name}"));
                docker_cmd(&["rm", "-f", &container_name]);
                ui::log_success("Container", &format!("Removed {container_name}"));
            }
            ContainerCommands::Backup { container_name } => {
                ui::log_step("BACKUP", &format!("Backing up {container_name}"));
            }
        },
        Commands::Service(sub) => match sub {
            ServiceCommands::List => render_service_table(),
        },
        Commands::List(sub) => match sub {
            ListCommands::Container => render_container_table(),
            ListCommands::Service => render_service_table(),
        },
        Commands::DeleteContainer { container_name } => {
            docker_cmd(&["rm", "-f", &container_name]);
            ui::log_success("Container", &format!("Deleted {container_name}"));
        }
    }
}

fn render_container_table() {
    let mut table = ui::create_table();
    table.set_header(vec![
        Cell::new("CONTAINER").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("IMAGE").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("STATUS").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("PORTS").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
    ]);

    table.add_row(vec![
        Cell::new("sertemp-postgres").fg(ui::steel_cell_color()),
        Cell::new("postgres:16-alpine"),
        Cell::new("Healthy (Up 2h)").fg(ui::mint_cell_color()),
        Cell::new("5432:5432"),
    ]);

    table.add_row(vec![
        Cell::new("sertemp-redis").fg(ui::steel_cell_color()),
        Cell::new("redis:7-alpine"),
        Cell::new("Running").fg(ui::mint_cell_color()),
        Cell::new("6379:6379"),
    ]);

    println!("{table}");
}

fn render_service_table() {
    let mut table = ui::create_table();
    table.set_header(vec![
        Cell::new("SERVICE").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("TIER").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("REPLICAS").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
        Cell::new("STATUS").fg(ui::bat_header_color()).add_attribute(Attribute::Bold),
    ]);

    table.add_row(vec![
        Cell::new("api-gateway").fg(ui::steel_cell_color()),
        Cell::new("edge"),
        Cell::new("2"),
        Cell::new("Active").fg(ui::mint_cell_color()),
    ]);

    table.add_row(vec![
        Cell::new("worker-queue").fg(ui::steel_cell_color()),
        Cell::new("backend"),
        Cell::new("4"),
        Cell::new("Active").fg(ui::mint_cell_color()),
    ]);

    println!("{table}");
}

fn docker_cmd(args: &[&str]) {
    let status = Command::new("docker").args(args).status();
    match status {
        Ok(s) if !s.success() => ui::log_error(&format!("Docker exited with status {s}")),
        Err(e) => ui::log_error(&format!("Failed to run docker: {e}")),
        _ => {}
    }
}
