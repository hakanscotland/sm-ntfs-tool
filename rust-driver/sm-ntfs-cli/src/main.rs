//! SM-NTFS CLI Tool
//!
//! Command-line interface for mounting and managing NTFS volumes.

use clap::{Parser, Subcommand};
use sm_ntfs_core::utils::logging;

#[derive(Parser)]
#[command(name = "sm-ntfs")]
#[command(version = "0.1.0")]
#[command(about = "SM-NTFS Tool for macOS - NTFS Read/Write Driver", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mount an NTFS volume
    Mount {
        /// Device path (e.g., /dev/disk2s1)
        #[arg(short, long)]
        device: String,

        /// Mount point (e.g., /tmp/ntfs)
        #[arg(short, long)]
        mount_point: String,

        /// Enable read-write mode
        #[arg(short, long, default_value_t = false)]
        read_write: bool,
    },

    /// Unmount an NTFS volume
    Unmount {
        /// Mount point
        mount_point: String,
    },

    /// List NTFS volumes
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    logging::init_logging();

    let cli = Cli::parse();

    match cli.command {
        Commands::Mount {
            device,
            mount_point,
            read_write,
        } => {
            tracing::info!(
                "Mounting {} at {} (read_write: {})",
                device,
                mount_point,
                read_write
            );
            println!("TODO: Implement mount functionality");
            // TODO: Implement in Week 1-2
        }
        Commands::Unmount { mount_point } => {
            tracing::info!("Unmounting {}", mount_point);
            println!("TODO: Implement unmount functionality");
            // TODO: Implement in Week 1-2
        }
        Commands::List => {
            tracing::info!("Listing NTFS volumes");
            println!("TODO: Implement list functionality");
            // TODO: Implement in Week 1-2
        }
    }

    Ok(())
}
