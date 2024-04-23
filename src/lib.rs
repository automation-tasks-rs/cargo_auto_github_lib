// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_github_lib
//!
//! **Library for cargo-auto `automation tasks written in rust language` with functions for GitHub.**  
//! ***version: 1.1.5 date: 2024-04-23 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/automation-tasks-rs/cargo_auto_github_lib)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rust](https://img.shields.io/badge/rust-orange)
//!  ![cargo-auto](https://img.shields.io/badge/cargo_auto-orange)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-412-green.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-158-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-30-purple.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-60-orange.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/)
//!
//! [![crates.io](https://img.shields.io/crates/v/cargo_auto_github_lib.svg)](https://crates.io/crates/cargo_auto_github_lib) [![Documentation](https://docs.rs/cargo_auto_github_lib/badge.svg)](https://docs.rs/cargo_auto_github_lib/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_github_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_github_lib/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_github_lib/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/blob/master/LICENSE) [![Rust](https://github.com/automation-tasks-rs/cargo_auto_github_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/automation-tasks-rs/cargo_auto_github_lib/) ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/714373530.svg)
//!
//! Hashtags: #rustlang #buildtool #developmenttool #github  
//! My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Try it
//!
//! In your rust project root directory (where the Cargo.toml is)  
//! first install [cargo-auto](https://crates.io/crates/cargo-auto) and generate a new helper project:
//!
//! ```bash
//! cargo install cargo-auto
//! cargo auto new
//! ```
//!
//! In a new editor open the generated directory `automation_tasks_rs` as an independent rust project. There is already this dependency in `Cargo.toml`:  
//!
//! ```toml
//! cargo_auto_github_lib="0.1.*"
//! ```
//!
//! Preview the code and observe all the `auto_github_*` functions from `cargo_auto_github_lib`.  
//! Example:  
//!
//! ```rust ignore
//! fn task_github_new_release() {
//!     // ...
//!
//!     let github_client = crate::github_mod::GitHubClient::new();
//!     let json_value = github_client.send_to_github_api(cgl::github_api_create_new_release(
//!         &owner,
//!         &repo_name,
//!         &tag_name_version,
//!         &release_name,
//!         branch,
//!         &body_md_text,
//!     ));
//!
//!     //...
//!
//!     // upload asset
//!     cgl::github_api_upload_asset_to_release(
//!         &github_client,
//!         &owner,
//!         &repo_name,
//!         &release_id,
//!         &tar_name,
//!     );
//! }
//!
//! ```
//!
//! You need to have a [GitHub PAT (personal access secret_token)](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token).
//!
//! Run (in your main rust project):
//!
//! ```bash
//! cargo auto release
//! cargo auto github_new_release
//! ```
//!
//! With a little luck, it will create a new release in github.  
//!
//! ## Functions
//!
//! All the functions have extensive hep/docs to describe how they work.  
//! It is nice when you use a code editor with IntelliSense like VSCode.  
//! Here is a list of some of them:  
//!
//! - `auto_github_create_new_release()` - creates new release on Github
//! - `auto_github_upload_asset_to_release()` - add asset to the github release
//!
//! ## GitHub API secret_token
//!
//! The GitHub API secret_token is a secret just like a password. Maybe even greater.  
//! With this API secret_token, a maleficent actor can change basically anything in your GitHub account. You don't want that.
//!
//! How to protect this secret?  
//! Ok, there are some basic recommendations:
//!
//! - HTTPS is a no-brainer. Never use HTTP ever again. It is plain text over the wire.
//! - Expire the secret_token frequently, so old secret_tokens are of no use
//! - Never store the secret_token in a file as plain text
//! - Plain text inside env vars can also be accessed from malware
//! - give the least permission/authorization to the API secret_token
//!
//! But the true problem arises at the moment when you want to use the secret_token. How to trust the code you are giving the secret_token to?  
//! Probably the best is that this code is written by you or that you have complete control over it. This makes very cumbersome the use of libraries/crates. You cannot trust them by default. However, it is impossible to avoid trust in low-level crates/libraries.
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
mod auto_github_api_mod;
mod utils_mod;

// reexport functions for callers of the library

pub use auto_github_api_mod::description_and_topics_to_github;
pub use auto_github_api_mod::git_has_remote;
pub use auto_github_api_mod::git_has_upstream;
pub use auto_github_api_mod::github_api_create_a_github_pages_site;
pub use auto_github_api_mod::github_api_create_new_release;
pub use auto_github_api_mod::github_api_get_repository;
pub use auto_github_api_mod::github_api_replace_all_topics;
pub use auto_github_api_mod::github_api_repository_new;
pub use auto_github_api_mod::github_api_update_description;
pub use auto_github_api_mod::github_api_upload_asset_to_release;
pub use auto_github_api_mod::new_remote_github_repository;
pub use auto_github_api_mod::SendToGitHubApi;
