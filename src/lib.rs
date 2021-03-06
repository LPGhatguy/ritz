//! Ritz is a simple templating library that has [JSX][jsx-intro]-like syntax
//! powered by [Snax](https://github.com/LPGhatguy/snax).
//!
//! ## Requirements
//! Ritz requires Rust 1.32 or newer.
//!
//! Some things are still a bit in flux, so I'm sorry in advance if I break
//! anything!
//!
//! ## Examples
//!
//! ### Simple Page
//! ```rust
//! use ritz::html;
//!
//! fn main() {
//!     let page_title = "Hello, world, from Snax!";
//!
//!     let page = html! {
//!         /* Snax supports regular multi-line Rust comments. */
//!         <html>
//!             <head>
//!                 /*
//!                     Literal strings need to be quoted, unlike JSX.
//!                     This makes whitespace much more explicit, which is
//!                     useful!
//!                 */
//!                 <title>"Hello, Snax!"</title>
//!             </head>
//!             <body>
//!                 /*
//!                     Snax supports embedding Rust expressions that return
//!                     `impl IntoIterator<HtmlContent>`. String and &str work
//!                     great here!
//!                 */
//!                 <h1>
//!                     { page_title }
//!                 </h1>
//!             </body>
//!         </html>
//!     };
//!
//!     // The result of the html! macro is ritz::HtmlContent.
//!     // It implements Display and gives you compact HTML without a doctype!
//!     println!("<!doctype html>");
//!     println!("{}", page);
//! }
//! ```
//!
//! ### Composition via functions
//! Snax is designed to work well when using functions to reuse pieces of HTML!
//!
//! ```rust
//! use ritz::{html, Fragment, HtmlContent};
//!
//! fn user_widget<'a>(name: &'a str, age: u32) -> HtmlContent<'a> {
//!     html! {
//!         <div class="user">
//!             { name } " is " { age.to_string() } " years old!"
//!         </div>
//!     }
//! }
//!
//! fn users() -> HtmlContent<'static> {
//!     let users = vec![
//!         ("Gandalf", 34),
//!         ("Arwen Undómie", 75),
//!         ("Primula Brandybuck", 133),
//!     ];
//!
//!     html! {
//!         <div class="users">
//!             { Fragment::new(users.iter().map(|(name, age)| user_widget(name, *age))) }
//!         </div>
//!     }
//! }
//! ```
//!
//! ## License
//! Ritz is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for
//! details.
//!
//! [jsx-intro]: https://reactjs.org/docs/introducing-jsx.html

mod types;

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use ritz_impl::html;
pub use types::*;