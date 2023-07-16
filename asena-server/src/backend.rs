use std::{
    hash::Hash,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use asena_ast_db::{
    db::AstDatabase,
    package::Package,
    vfs::{FileSystem, VfsFile, VfsFileData, VfsPath},
};
use asena_ast_lowering::db::AstLowerrer;
use asena_ast_resolver::db::AstResolverDatabase;
use asena_prec::PrecDatabase;
use asena_report::BoxInternalError;
use im::HashSet;
use itertools::Itertools;
use ropey::Rope;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, Diagnostic,
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, ExecuteCommandOptions,
        InitializeParams, InitializeResult, InitializedParams, MessageType, OneOf, Position, Range,
        ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url, WorkspaceFolder,
        WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
    },
    Client, LanguageServer,
};

#[derive(Debug, Clone)]
pub struct Backend {
    pub client: Client,
    pub db: Arc<crate::ide_db::IdeDatabase>,
    pub workspace_ready: Arc<AtomicBool>,
}

unsafe impl Send for Backend {}

unsafe impl Sync for Backend {}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(false)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: None,
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        let Ok(Some(workspaces)) = self.client.workspace_folders().await else {
            return;
        };

        for workspace in workspaces {
            self.load_workspace(workspace).await.ok();
        }
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".into(), "World".into()),
        ])))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let text = &mut params.content_changes[0].text;

        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(text),
            version: params.text_document.version,
        })
        .await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub struct TextDocumentItem {
    pub uri: Url,
    pub version: i32,
    pub text: String,
}

impl Backend {
    pub async fn on_change(&self, params: TextDocumentItem) {
        let backend = self.clone();

        backend.info("file opened").await;

        tokio::spawn(async move {
            while let false = backend.workspace_ready.load(Ordering::SeqCst) {}

            backend.info("  file opened within workspace").await;

            let module_ref = backend.db.path_module(PathBuf::from(params.uri.path()));
            let vfs_file = backend.db.vfs_file(module_ref);
            let file = backend.db.ast(vfs_file);
            let file = backend.db.infix_commands(file.into());
            let file = backend.db.ordered_prec(file.into());
            let file = backend.db.ast_resolved_file(file.into());
            let _hir = backend.db.hir_file(file.into());

            let source = backend.db.source(vfs_file);
            let source = ropey::Rope::from(source.to_string());

            let diagnostics = backend
                .db
                .diagnostics(vfs_file)
                .into_iter()
                .enumerate()
                .map(|(order, diagnostic)| OrdDiagnostic { order, diagnostic })
                .collect::<HashSet<OrdDiagnostic>>()
                .into_iter()
                .sorted_by(|d, n| d.order.cmp(&n.order))
                .filter_map(|d| backend.build_diagnostic(d.diagnostic, &source))
                .collect_vec();

            backend
                .client
                .publish_diagnostics(params.uri, diagnostics, Some(params.version))
                .await;
        });
    }

    pub async fn info(&self, message: impl Into<String>) {
        self.client
            .log_message(MessageType::INFO, message.into())
            .await;
    }

    pub async fn load_workspace(&self, workspace: WorkspaceFolder) -> tokio::io::Result<()> {
        let path = workspace.uri.path();
        let mut files = tokio::fs::read_dir(path).await?;
        let vfs = Arc::new(FileSystem {
            base_dir: Some(path.into()),
        });
        let pkg = Package::new(&*self.db, "Local", "0.0.0", vfs);
        self.info(format!("loading workspace: {path:?}")).await;

        while let Some(entry) = files.next_entry().await? {
            let path = entry.path();
            self.info(format!("  -> loading file: {path:?}")).await;

            let name = entry.file_name().to_string_lossy().into_owned();
            let vfs_path = VfsPath { path };
            let metadata = entry.metadata().await?;
            if metadata.is_file() && vfs_path.path.extension().unwrap_or_default() == "ase" {
                VfsFileData::new(&*self.db, &name, vfs_path, pkg);
            }
        }

        self.workspace_ready.store(true, Ordering::SeqCst);

        Ok(())
    }

    fn build_diagnostic(&self, diagnostic: AsenaDiagnostic, rope: &Rope) -> Option<Diagnostic> {
        let message = diagnostic.message.to_string();
        let range = diagnostic.message.span.range;

        let start = Self::offset_to_position(range.start(), rope)?;
        let end = Self::offset_to_position(range.end(), rope)?;

        Some(Diagnostic::new_simple(Range::new(start, end), message))
    }

    fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
        let line = rope.try_char_to_line(offset).ok()?;
        let first_char_of_line = rope.try_line_to_char(line).ok()?;
        let column = offset - first_char_of_line;
        Some(Position::new(line as u32, column as u32))
    }
}

type AsenaDiagnostic = asena_report::Diagnostic<BoxInternalError>;

#[derive(Debug, Clone)]
struct OrdDiagnostic {
    order: usize,
    diagnostic: AsenaDiagnostic,
}

impl Hash for OrdDiagnostic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.diagnostic.hash(state);
    }
}

impl PartialEq for OrdDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.diagnostic == other.diagnostic
    }
}

impl Eq for OrdDiagnostic {}

impl PartialOrd for OrdDiagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for OrdDiagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
