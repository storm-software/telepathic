use telepathic_core::{Context, Options};

use crate::PipelineResult;

#[derive(Debug, Clone)]
pub struct Pipeline {
  pub(super) context: Context,
  pub(super) is_closed: bool,
}

impl Pipeline {
  #[tracing::instrument(skip(options), level = "trace")]
  pub fn new(options: Options) -> PipelineResult<Self> {
    let context = Context::new(options);

    Ok(Self { context, is_closed: false })
  }
}
