#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;
