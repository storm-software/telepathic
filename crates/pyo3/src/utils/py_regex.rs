use std::borrow::Cow;

/// According to the doc of `regress`, https://docs.rs/regress/0.10.0/regress/#comparison-to-regex-crate
/// **regress supports features that regex does not, in particular backreferences and zero-width lookaround assertions.**
/// these features are not commonly used, so in most cases the slow path will not be reached.
#[derive(Debug, Clone)]
pub enum HybridRegex {
  Optimize(regex::Regex),
  Ecma(regress::Regex),
}

// Please only used for testing
impl From<&str> for HybridRegex {
  fn from(pattern: &str) -> Self {
    HybridRegex::new(pattern).unwrap_or_else(|err| {
      panic!("failed to create HybridRegex from {pattern}, error details: {err}",)
    })
  }
}

impl HybridRegex {
  pub fn new(pattern: &str) -> anyhow::Result<Self> {
    let regex_pattern = Self::get_regex_pattern(pattern, "");
    match regex::Regex::new(&regex_pattern).map(HybridRegex::Optimize) {
      Ok(reg) => Ok(reg),
      Err(_) => regress::Regex::new(pattern).map(HybridRegex::Ecma).map_err(anyhow::Error::from),
    }
  }

  pub fn with_flags(pattern: &str, flags: &str) -> anyhow::Result<Self> {
    let regex_pattern = Self::get_regex_pattern(pattern, flags);
    match regex::Regex::new(&regex_pattern).map(HybridRegex::Optimize) {
      Ok(reg) => Ok(reg),
      Err(_) => regress::Regex::with_flags(pattern, flags)
        .map(HybridRegex::Ecma)
        .map_err(anyhow::Error::from),
    }
  }

  pub fn regex_pattern(&self) -> Option<&str> {
    match self {
      HybridRegex::Optimize(r) => Some(r.as_str()),
      HybridRegex::Ecma(_) => None,
    }
  }

  fn get_regex_pattern(pattern: &str, flags: &str) -> String {
    crate::concat_string!("(?R", flags, ")", pattern)
  }

  pub fn matches(&self, text: &str) -> bool {
    match self {
      HybridRegex::Optimize(reg) => reg.is_match(text),
      HybridRegex::Ecma(reg) => reg.find(text).is_some(),
    }
  }

  pub fn replace<'a>(&self, haystack: &'a str, replacement: &str) -> Cow<'a, str> {
    match self {
      HybridRegex::Optimize(r) => r.replace(haystack, replacement),
      HybridRegex::Ecma(reg) => {
        let next = reg.find_iter(haystack).next();
        let Some(m) = next else { return Cow::Borrowed(haystack) };
        Cow::Owned(crate::concat_string!(&haystack[..m.start()], replacement, &haystack[m.end()..]))
      }
    }
  }

  pub fn replace_all<'a>(&self, haystack: &'a str, replacement: &str) -> Cow<'a, str> {
    match self {
      HybridRegex::Optimize(r) => r.replace_all(haystack, replacement),
      HybridRegex::Ecma(reg) => regress_regexp_replace_all(reg, haystack, replacement),
    }
  }
}

fn regress_regexp_replace_all<'h>(
  reg: &regress::Regex,
  haystack: &'h str,
  replacement: &str,
) -> Cow<'h, str> {
  let iter = reg.find_iter(haystack);
  let mut iter = iter.peekable();
  if iter.peek().is_none() {
    return Cow::Borrowed(haystack);
  }

  let mut ret = String::with_capacity(haystack.len());
  let mut last = 0;
  for m in iter {
    ret.push_str(&haystack[last..m.start()]);
    ret.push_str(replacement);
    last = m.end();
  }
  ret.push_str(&haystack[last..]);
  Cow::Owned(ret)
}
