[//]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_github_lib

[//]: # (auto_cargo_toml_to_md start)

**Library for cargo-auto `automation tasks written in rust language` with functions for github.**  
***version: 0.1.24 date: 2023-05-31 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_auto_github_lib)***  

![status](https://img.shields.io/badge/obsolete-red) 
![status](https://img.shields.io/badge/archived-red) 

***The functions of this crate are moved to the crate cargo_auto_lib.***

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-138-green.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-152-blue.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-11-purple.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-60-orange.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/)

[//]: # (auto_lines_of_code end)

[![crates.io](https://img.shields.io/crates/v/cargo_auto_github_lib.svg)](https://crates.io/crates/cargo_auto_github_lib) [![Documentation](https://docs.rs/cargo_auto_github_lib/badge.svg)](https://docs.rs/cargo_auto_github_lib/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_github_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_github_lib/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_github_lib/) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/blob/master/LICENSE) [![Rust](https://github.com/bestia-dev/cargo_auto_github_lib/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_auto_github_lib/) ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/714373530.svg)

Hashtags: #rustlang #buildtool #developmenttool #github  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

In your rust project root directory (where the Cargo.toml is)  
first install [cargo-auto](https://crates.io/crates/cargo-auto) and generate a new helper project:

```bash
cargo install cargo-auto
cargo auto new
```

In a new editor open the generated directory `automation_tasks_rs` as an independent rust project. There is already this dependency in `Cargo.toml`:  

```toml
cargo_auto_github_lib="0.1.*"
```

Preview the code and observe all the `auto_github_*` functions from `cargo_auto_github_lib`.  
Example:  

```rust ignore
fn task_github_new_release() {
    // async block inside sync code with tokio
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        // ...

        let release_id =  auto_github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text).await;
        println!("New release created, now uploading release asset. This can take some time if the files are big. Wait...");

        // upload asset
        let path_to_file = format!(
            "target/release/{package_name}",
            package_name = package_name()
        );

        auto_github_upload_asset_to_release(&owner, &repo, &release_id, &path_to_file).await;
        println!("Asset uploaded.");
    });
}

```

You need to have a [github PAT (personal access token)](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token) and save it in a environment variable:  

```bash
 export GITHUB_TOKEN=ghp_111111111111111111111
```

Run (in your main rust project):

```bash
cargo auto release
cargo auto github_new_release
```

With a little luck, it will create a new release in github.  

## Functions

All the functions have extensive hep/docs to describe how they work.  
It is nice when you use a code editor with IntelliSense like VSCode.  
Here is a list of some of them:  

- `auto_github_create_new_release()` - creates new release on Github
- `auto_github_upload_asset_to_release()` - add asset to the github release


## cargo crev reviews and advisory

We leave in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
