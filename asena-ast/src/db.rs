use std::sync::Arc;

use crate::reporter::Reporter;

#[salsa::query_group(ReporterStorage)]
pub trait ReporterDatabase {
    #[salsa::input]
    fn reporter(&self) -> Arc<Reporter>;
}
