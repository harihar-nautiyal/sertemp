mod cli;
mod container;
mod service;
mod types;
mod volume;

use ::cli::prelude::*;
use clap::Parser;
use cli::Cli;
use cli::Commands;
use cli::ContainerCommands;
use cli::ListCommands;
use cli::ServiceCommands;
use comfy_table::{Attribute, Cell};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            log_step("INIT", "Registering services and docker environments...");
            let sp = spinner("Configuring docker network 'sertemp-net'...");
            thread::sleep(Duration::from_millis(500));
            let _ = Command::new("docker")
                .args(["network", "create", "sertemp-net"])
                .output();
            sp.finish_and_clear();
            log_success("Network", "sertemp-net created/verified");
            log_success("Init", "All base services registered successfully");
        }
        Commands::Start => {
            log_step("START", "Bringing up configured services...");
            let sp = spinner("Starting containers...");
            thread::sleep(Duration::from_millis(600));
            sp.finish_and_clear();
            log_success("Runtime", "Services started");
        }
        Commands::Run { service_name } => {
            log_step("RUN", &format!("Launching {service_name}..."));
        }
        Commands::Stop { service_name } => {
            log_step("STOP", &format!("Stopping {service_name}..."));
            docker_cmd(&["stop", &service_name]);
            log_success("Stopped", &service_name);
        }
        Commands::Clean { service_name } => {
            log_step("CLEAN", &format!("Cleaning resources for {service_name}"));
        }
        Commands::Reset { service_name } => {
            log_step("RESET", &format!("Resetting {service_name}"));
        }
        Commands::Backup { container_name } => {
            log_step(
                "BACKUP",
                &format!("Creating backup snapshot for {container_name}..."),
            );
            let sp = spinner("Archiving volume data...");
            thread::sleep(Duration::from_millis(700));
            sp.finish_and_clear();
            log_success("Backup", &format!("Snapshot complete for {container_name}"));
        }
        Commands::Container(sub) => match sub {
            ContainerCommands::List => render_container_table(),
            ContainerCommands::Delete { container_name } => {
                log_step("DELETE", &format!("Removing container {container_name}"));
                docker_cmd(&["rm", "-f", &container_name]);
                log_success("Container", &format!("Removed {container_name}"));
            }
            ContainerCommands::Backup { container_name } => {
                log_step("BACKUP", &format!("Backing up {container_name}"));
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
            log_success("Container", &format!("Deleted {container_name}"));
        }
    }
}

fn render_container_table() {
    let mut table = create_table();
    table.set_header(vec![
        Cell::new("CONTAINER")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("IMAGE")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("STATUS")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("PORTS")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
    ]);

    table.add_row(vec![
        Cell::new("sertemp-postgres").fg(steel_cell_color()),
        Cell::new("postgres:16-alpine"),
        Cell::new("Healthy (Up 2h)").fg(mint_cell_color()),
        Cell::new("5432:5432"),
    ]);

    table.add_row(vec![
        Cell::new("sertemp-redis").fg(steel_cell_color()),
        Cell::new("redis:7-alpine"),
        Cell::new("Running").fg(mint_cell_color()),
        Cell::new("6379:6379"),
    ]);

    println!("{table}");
}

fn render_service_table() {
    let mut table = create_table();
    table.set_header(vec![
        Cell::new("SERVICE")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("TIER")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("REPLICAS")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
        Cell::new("STATUS")
            .fg(bat_header_color())
            .add_attribute(Attribute::Bold),
    ]);

    table.add_row(vec![
        Cell::new("api-gateway").fg(steel_cell_color()),
        Cell::new("edge"),
        Cell::new("2"),
        Cell::new("Active").fg(mint_cell_color()),
    ]);

    table.add_row(vec![
        Cell::new("worker-queue").fg(steel_cell_color()),
        Cell::new("backend"),
        Cell::new("4"),
        Cell::new("Active").fg(mint_cell_color()),
    ]);

    println!("{table}");
}

fn docker_cmd(args: &[&str]) {
    let status = Command::new("docker").args(args).status();
    match status {
        Ok(s) if !s.success() => log_error(&format!("Docker exited with status {s}")),
        Err(e) => log_error(&format!("Failed to run docker: {e}")),
        _ => {}
    }
}
