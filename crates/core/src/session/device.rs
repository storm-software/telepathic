use whoami::{
  CpuArchitecture, DesktopEnvironment, Platform, cpu_arch, desktop_env, devicename, distro,
  hostname, platform,
};

use serde::{Deserialize, Serialize};

mod serde_platform {
  use super::*;
  use serde::{Deserialize, Deserializer, Serializer};

  pub(super) fn serialize<S: Serializer>(value: &Platform, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
  }

  pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Platform, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(parse(&value))
  }

  fn parse(value: &str) -> Platform {
    if let Some(rest) = value.strip_prefix("Unknown: ") {
      return Platform::Unknown(rest.to_string());
    }

    match value {
      "Linux" => Platform::Linux,
      "BSD" => Platform::Bsd,
      "Windows" => Platform::Windows,
      "macOS" => Platform::Mac,
      "illumos" => Platform::Illumos,
      "iOS" => Platform::Ios,
      "Android" => Platform::Android,
      "Nintendo 3DS" => Platform::Nintendo3ds,
      "PlayStation" => Platform::PlayStation,
      "Fuchsia" => Platform::Fuchsia,
      "Redox" => Platform::Redox,
      "GNU Hurd" => Platform::Hurd,
      other => Platform::Unknown(other.to_string()),
    }
  }
}

mod serde_desktop_env {
  use super::*;
  use serde::{Deserialize, Deserializer, Serializer};

  pub(super) fn serialize<S: Serializer>(
    value: &DesktopEnvironment,
    serializer: S,
  ) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
  }

  pub(super) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<DesktopEnvironment, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(parse(&value))
  }

  fn parse(value: &str) -> DesktopEnvironment {
    if let Some(rest) = value.strip_prefix("Unknown: ") {
      return DesktopEnvironment::Unknown(rest.to_string());
    }

    if let Some(rest) = value.strip_prefix("WebBrowser (").and_then(|s| s.strip_suffix(')')) {
      return DesktopEnvironment::WebBrowser(rest.to_string());
    }

    match value {
      "Gnome" => DesktopEnvironment::Gnome,
      "Windows" => DesktopEnvironment::Windows,
      "LXDE" => DesktopEnvironment::Lxde,
      "Openbox" => DesktopEnvironment::Openbox,
      "Mate" => DesktopEnvironment::Mate,
      "XFCE" => DesktopEnvironment::Xfce,
      "KDE Plasma" => DesktopEnvironment::Plasma,
      "Cinnamon" => DesktopEnvironment::Cinnamon,
      "I3" => DesktopEnvironment::I3,
      "Aqua" => DesktopEnvironment::Aqua,
      "IOS" => DesktopEnvironment::Ios,
      "Android" => DesktopEnvironment::Android,
      "Console" => DesktopEnvironment::Console,
      "Ubuntu" => DesktopEnvironment::Ubuntu,
      "Ermine" => DesktopEnvironment::Ermine,
      "Orbital" => DesktopEnvironment::Orbital,
      "Niri" => DesktopEnvironment::Niri,
      "Hyprland" => DesktopEnvironment::Hyprland,
      "Cosmic" => DesktopEnvironment::Cosmic,
      other => DesktopEnvironment::Unknown(other.to_string()),
    }
  }
}

mod serde_cpu_arch {
  use super::*;
  use serde::{Deserialize, Deserializer, Serializer};

  pub(super) fn serialize<S: Serializer>(
    value: &CpuArchitecture,
    serializer: S,
  ) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
  }

  pub(super) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
  ) -> Result<CpuArchitecture, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(parse(&value))
  }

  fn parse(value: &str) -> CpuArchitecture {
    if let Some(rest) = value.strip_prefix("Unknown: ") {
      return CpuArchitecture::Unknown(rest.to_string());
    }

    match value {
      "armv5" => CpuArchitecture::ArmV5,
      "armv6" => CpuArchitecture::ArmV6,
      "armv7" => CpuArchitecture::ArmV7,
      "arm64" => CpuArchitecture::Arm64,
      "i386" => CpuArchitecture::I386,
      "i586" => CpuArchitecture::I586,
      "i686" => CpuArchitecture::I686,
      "mips" => CpuArchitecture::Mips,
      "mipsel" => CpuArchitecture::MipsEl,
      "mips64" => CpuArchitecture::Mips64,
      "mips64el" => CpuArchitecture::Mips64El,
      "powerpc" => CpuArchitecture::PowerPc,
      "powerpc64" => CpuArchitecture::PowerPc64,
      "powerpc64le" => CpuArchitecture::PowerPc64Le,
      "riscv32" => CpuArchitecture::Riscv32,
      "riscv64" => CpuArchitecture::Riscv64,
      "s390x" => CpuArchitecture::S390x,
      "sparc" => CpuArchitecture::Sparc,
      "sparc64" => CpuArchitecture::Sparc64,
      "wasm32" => CpuArchitecture::Wasm32,
      "wasm64" => CpuArchitecture::Wasm64,
      "x86_64" => CpuArchitecture::X64,
      other => CpuArchitecture::Unknown(other.to_string()),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the device.
pub struct DeviceInfo {
  /// The name of the device.
  pub name: String,
  /// The display name of the device.
  pub display_name: String,
  /// The platform of the device.
  #[serde(with = "serde_platform")]
  pub platform: Platform,
  /// The distribution of the device.
  pub distro: String,
  /// The desktop environment of the device.
  #[serde(with = "serde_desktop_env")]
  pub desktop_env: DesktopEnvironment,
  /// The CPU architecture of the device.
  #[serde(with = "serde_cpu_arch")]
  pub cpu_arch: CpuArchitecture,
}

impl DeviceInfo {
  pub fn new(
    name: String,
    display_name: String,
    platform: Platform,
    distro: String,
    desktop_env: DesktopEnvironment,
    cpu_arch: CpuArchitecture,
  ) -> Self {
    Self { name, display_name, platform, distro, desktop_env, cpu_arch }
  }
}

impl Default for DeviceInfo {
  fn default() -> Self {
    let name =
      devicename().unwrap_or_else(|_| hostname().unwrap_or_else(|_| "Unknown Device".to_string()));
    let display_name = hostname().unwrap_or_else(|_| name.clone());

    Self {
      name,
      display_name,
      platform: platform(),
      distro: distro().unwrap_or_else(|_| "Unknown OS Distribution".to_string()),
      desktop_env: desktop_env()
        .unwrap_or_else(|| DesktopEnvironment::Unknown("Unknown".to_string())),
      cpu_arch: cpu_arch(),
    }
  }
}
