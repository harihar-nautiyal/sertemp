use clap::{Parser, Subcommand};

use crate::style::style;

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(about = "Sertemp workspace automation & deployment runner", long_about = None)]
#[command(styles = style())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
    DeleteContainer { container_name: String },
}

#[derive(Debug, Subcommand)]
pub enum ContainerCommands {
    /// List all containers managed by sertemp
    List,
    /// Delete a specific container
    Delete { container_name: String },
    /// Backup a specific container
    Backup { container_name: String },
}

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// List all registered services
    List,
}

#[derive(Debug, Subcommand)]
pub enum ListCommands {
    /// List all containers
    Container,
    /// List all services
    Service,
}
