#![feature(try_blocks)]

use std::sync::{atomic::AtomicBool, Arc};

use tower_lsp::{LspService, Server};

use crate::backend::Backend;

pub mod backend;
pub mod ide_db;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db = ide_db::IdeDatabase::default();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        db: Arc::new(db),
        workspace_ready: Arc::new(AtomicBool::new(false)),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
