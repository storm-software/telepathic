use crate::{
  types::{
    binding_error::{BindingErrors, BindingResult},
    binding_input::{
      BindingExportOkfInput, BindingListProjectsInput, BindingQueryGraphInput,
      BindingReadGraphInput, BindingSearchGraphInput, BindingTraceGraphInput,
      BindingWriteGraphInput,
    },
    binding_options::BindingOptions,
    binding_output::{
      BindingExportOkfOutput, BindingGetSchemaOutput, BindingGetSessionOutput,
      BindingGetSettingsOutput, BindingIndexRepositoryOutput, BindingListProjectsOutput,
      BindingListRepositoriesOutput, BindingQueryGraphOutput, BindingReadGraphOutput,
      BindingSearchGraphOutput, BindingTraceGraphOutput, BindingWriteGraphOutput,
    },
  },
  utils::to_binding_error,
};
use napi::{Either, Env, bindgen_prelude::PromiseRaw};
use napi_derive::napi;
use telepathic_engine::Engine;

#[napi]
#[derive(Debug)]
pub struct BindingEngine {
  inner: Engine,
}

#[napi]
impl BindingEngine {
  #[napi(constructor)]
  pub fn new(options: BindingOptions) -> napi::Result<Self> {
    let inner = Engine::new(options.into());
    if inner.is_err() {
      return Err(napi::Error::from_reason(inner.err().unwrap().to_string()));
    }

    Ok(Self { inner: inner.expect("Unable to create Telepathic engine") })
  }

  #[napi]
  pub fn get_settings<'env>(
    &mut self,
    env: &'env Env,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingGetSettingsOutput>>> {
    let result = self.inner.get_settings();
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn get_session<'env>(
    &mut self,
    env: &'env Env,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingGetSessionOutput>>> {
    let result = self.inner.get_session();
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn get_schema<'env>(
    &mut self,
    env: &'env Env,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingGetSchemaOutput>>> {
    let result = self.inner.get_schema();
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn list_repositories<'env>(
    &mut self,
    env: &'env Env,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingListRepositoriesOutput>>> {
    let result = self.inner.list_repositories();
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn index_repository<'env>(
    &mut self,
    env: &'env Env,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingIndexRepositoryOutput>>> {
    let result = self.inner.index_repository();
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn list_projects<'env>(
    &mut self,
    env: &'env Env,
    input: BindingListProjectsInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingListProjectsOutput>>> {
    let result = self.inner.list_projects(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn write_graph<'env>(
    &mut self,
    env: &'env Env,
    input: BindingWriteGraphInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingWriteGraphOutput>>> {
    let result = self.inner.write_graph(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn read_graph<'env>(
    &mut self,
    env: &'env Env,
    input: BindingReadGraphInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingReadGraphOutput>>> {
    let result = self.inner.read_graph(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn query_graph<'env>(
    &mut self,
    env: &'env Env,
    input: BindingQueryGraphInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingQueryGraphOutput>>> {
    let result = self.inner.query_graph(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn search_graph<'env>(
    &mut self,
    env: &'env Env,
    input: BindingSearchGraphInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingSearchGraphOutput>>> {
    let result = self.inner.search_graph(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn trace_graph<'env>(
    &mut self,
    env: &'env Env,
    input: BindingTraceGraphInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingTraceGraphOutput>>> {
    let result = self.inner.trace_graph(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  pub fn export_okf<'env>(
    &mut self,
    env: &'env Env,
    input: BindingExportOkfInput,
  ) -> napi::Result<PromiseRaw<'env, BindingResult<BindingExportOkfOutput>>> {
    let result = self.inner.export_okf(input.into());
    let fut = async move {
      match result {
        Ok(output) => Ok(Either::B(output.into())),
        Err(err) => Ok(Either::A(BindingErrors::new(vec![to_binding_error(&err)]))),
      }
    };

    env.spawn_future(fut)
  }

  #[napi]
  // - `Engine::close()/inner.close()` requires acquiring `&mut self`
  // - Acquiring `&mut self` in async napi `fn` is unsafe, so we must use a sync `fn` here.
  // - But `Engine::close()/inner.close()` contains async cleanup operations, so we have await its returned future
  // in another async context instead of directly calling `close().await`.
  // - This also affects how the code is written in `Engine::close()/inner.close()`, see the implementation there for more details.
  pub fn close<'env>(&mut self, env: &'env Env) -> napi::Result<PromiseRaw<'env, ()>> {
    let cleanup_fut = self.inner.close();
    env.spawn_future(async move {
      let res = cleanup_fut.await;
      if res.is_err() {
        return Err(napi::Error::from_reason(res.err().unwrap().to_string()));
      }
      Ok(())
    })
  }

  #[napi(getter)]
  pub fn is_closed(&self) -> bool {
    self.inner.is_closed()
  }
}
