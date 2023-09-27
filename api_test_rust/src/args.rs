use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct PartfulCli {
    #[clap(subcommand)]
    pub entity_type: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List the parts in the part tree
    List(ListArgs),

    /// Create a new part
    Create(CreateArgs),

    /// Delete a part
    Delete(DeleteArgs),

    /// Get a new part
    Get(GetArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Pass an optional search string for looking up parts
    pub search_string: String,

    /// Number of records to skip for pagination
    pub skip: usize,

    /// Maximum number of records to return
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Product name to list parts to delete
    pub product_number: String,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Product name to list parts to delete
    pub product_number: String,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Product name to get
    pub product_number: String,
}
