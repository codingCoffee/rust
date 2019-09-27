#![crate_name = "rustc_highlight"]
#![unstable(feature = "rustc_highlight", issue = "9999")]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/")]
#![deny(warnings)]

#![feature(rustc_private)]
#![feature(staged_api)]

#[macro_use] extern crate syntax;
extern crate syntax_pos;
#[macro_use] extern crate log;
extern crate term;

pub mod highlight;
pub mod escape;
