use pyo3::prelude::*;
use telepathic_core::session::DeviceInfo;

#[derive(Clone, PartialEq, Eq, FromPyObject, IntoPyObject)]
#[pyo3(from_item_all)]
pub struct BindingDevice {
  pub name: String,
  pub display_name: String,
  pub platform: String,
  pub distro: String,
  pub desktop_env: String,
  pub cpu_arch: String,
}

impl From<DeviceInfo> for BindingDevice {
  fn from(value: DeviceInfo) -> Self {
    Self {
      name: value.name,
      display_name: value.display_name,
      platform: format!("{:?}", value.platform),
      distro: value.distro,
      desktop_env: format!("{:?}", value.desktop_env),
      cpu_arch: format!("{:?}", value.cpu_arch),
    }
  }
}
