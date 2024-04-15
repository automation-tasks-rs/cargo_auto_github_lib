// auto_github_api_mod

//! functions to work with github api
//! WARNING: Never pass the secret API token to this crate library.
//! Pass the function send_to_github_api() as a parameter. It encapsulates the secret token.

use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cl::BLUE;
use cl::RED;
use cl::RESET;
use cl::YELLOW;

/// trait for GitHubClient in the calling crate
pub trait SendToGitHubApi {
    /// Send github api request
    ///
    /// This function encapsulates the secret API token.
    /// The RequestBuilder is created somewhere in the library crate.
    /// The client can be passed to the library. It will not reveal the secret token.
    fn send_to_github_api(&self, req: reqwest::blocking::RequestBuilder) -> serde_json::Value;

    /// Upload to github
    ///
    /// This function encapsulates the secret API token.
    /// The RequestBuilder is created somewhere in the library crate.
    /// The client can be passed to the library. It will not reveal the secret token.
    /// This is basically an async fn, but use of `async fn` in public traits is discouraged...
    fn upload_to_github(
        &self,
        req: reqwest::RequestBuilder,
    ) -> impl std::future::Future<Output = serde_json::Value> + Send;
}

/// Interactive ask to create a new remote GitHub repository
///
/// Use a function pointer to send_to_github_api() to avoid passing the secret token.
pub fn new_remote_github_repository(github_client: &impl SendToGitHubApi) -> Option<String> {
    // early error if Repository contains the placeholder "github_owner" or does not contain the true github_owner
    let cargo_toml = cl::CargoToml::read();
    let github_owner = cargo_toml
        .github_owner()
        .unwrap_or_else(|| panic!("{RED}ERROR: Element Repository in Cargo.toml does not contain the github_owner!{RESET}"));
    if github_owner == "github_owner" {
        panic!("{RED}ERROR: Element Repository in Cargo.toml contain the placeholder phrase '/github_owner/'! Modify it with your github owner name.{RESET}");
    }
    let name = cargo_toml.package_name();
    let description = cargo_toml.package_description().unwrap_or_else(|| {
        panic!("{RED}ERROR: Element Description in Cargo.toml does not exist!{RESET}")
    });

    // if push is not possible, then this function will not execute completely.
    // TODO: check if the github is in the ssh agent and panic if not

    // ask interactive
    println!("    {BLUE}This project does not have a remote GitHub repository.{RESET}");
    let answer = inquire::Text::new(&format!(
        "{BLUE}Do you want to create a new remote GitHub repository? (y/n){RESET}"
    ))
    .prompt()
    .unwrap();
    if answer.to_lowercase() != "y" {
        // early exit
        return None;
    }
    // continue if answer is "y"

    let json = github_client.send_to_github_api(github_api_repository_new(
        &github_owner,
        &name,
        &description,
    ));

    // get just the name, description and html_url from json
    println!(
        "{YELLOW}name: {}{RESET}",
        json.get("name").unwrap().as_str().unwrap()
    );
    println!(
        "{YELLOW}description: {}{RESET}",
        json.get("description").unwrap().as_str().unwrap()
    );
    let repo_html_url = json.get("html_url").unwrap().as_str().unwrap().to_string();
    println!("{YELLOW}url: {}{RESET}", &repo_html_url);

    // add this GitHub repository to origin remote over SSH (use sshadd for passphrase)
    cl::run_shell_command(&format!(
        "git remote add origin git@github.com:{github_owner}/{name}.git"
    ));
    cl::run_shell_command("git push -u origin main");

    // the docs pages are created with a GitHub action
    let _json = github_client
        .send_to_github_api(github_api_create_a_github_pages_site(&github_owner, &name));

    Some(repo_html_url)
}

/// Check and modify the description and topics on Github
///
/// The words topics, keywords and tags all mean the same concept.
/// In cargo.toml we have keywords.
/// In README.md I want to have badges for tags
/// In GitHub they are topics.
/// Topic must be only one word: lowercase letters, hyphens(-) or numbers, less then 35 characters.
pub fn description_and_topics_to_github(github_client: &impl SendToGitHubApi) {
    let cargo_toml = cl::CargoToml::read();
    let repo_name = cargo_toml.package_name();
    let owner = cargo_toml.github_owner().unwrap();
    let description = cargo_toml.package_description().unwrap();
    let keywords = cargo_toml.package_keywords();

    // get data from GitHub
    let json = github_client.send_to_github_api(github_api_get_repository(&owner, &repo_name));

    // get just the description and topis from json
    let gh_description = json.get("description").unwrap().as_str().unwrap();
    let gh_topics = json.get("topics").unwrap().as_array().unwrap();
    let gh_topics: Vec<String> = gh_topics
        .into_iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    // are description and topics both equal?
    if gh_description != description {
        let _json = github_client.send_to_github_api(github_api_update_description(
            &owner,
            &repo_name,
            &description,
        ));
    }

    // all elements must be equal, but not necessary in the same order
    let topics_is_equal = if gh_topics.len() == keywords.len() {
        let mut elements_is_equal = true;
        'outer: for x in gh_topics.iter() {
            let mut has_element = false;
            'inner: for y in keywords.iter() {
                if y == x {
                    has_element = true;
                    break 'inner;
                }
            }
            if !has_element {
                elements_is_equal = false;
                break 'outer;
            }
        }
        elements_is_equal
    } else {
        false
    };

    if !topics_is_equal {
        let _json = github_client
            .send_to_github_api(github_api_replace_all_topics(&owner, &repo_name, &keywords));
    }
}

/// GitHub api get repository
pub fn github_api_get_repository(
    owner: &str,
    repo_name: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository

        curl -L \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/OWNER/REPO
    */
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}");
    // return
    reqwest::blocking::Client::new()
        .get(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
}

/// Create a new github repository
pub fn github_api_repository_new(
    owner: &str,
    name: &str,
    description: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-for-the-authenticated-user

    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/repos \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repo!",
        "homepage":"https://github.com",
        "private":false,
        "is_template":true
    }'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let repos_url = format!("https://api.github.com/user/repos");
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "homepage": format!("https://{owner}.github.io/{name}"),
        "private":false,
        "has_issues":true,
        "has_projects":false,
        "has_wiki":false,
        // more settings...
        "has_discussions" :true
    });
    // Sadly there is no way in the API to set the settings: releases, packages and deployments
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub api update description
pub fn github_api_update_description(
    owner: &str,
    repo_name: &str,
    description: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#update-a-repository

    curl -L \
    -X PATCH \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/OWNER/REPO \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repository",
        "homepage":"https://github.com",
        "private":true,
        "has_issues":true,
        "topics": [
            "cat",
            "atom",
            "electron",
            "api"
            ],
        "has_projects":true,
        "has_wiki":true}'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}");
    let body = serde_json::json!({
        "description": description,
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .patch(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub API replace all topics
pub fn github_api_replace_all_topics(
    owner: &str,
    repo_name: &str,
    topics: &Vec<String>,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#replace-all-repository-topics
    curl -L \
      -X PUT \
      -H "Accept: application/vnd.github+json" \
      -H "Authorization: Bearer <YOUR-TOKEN>" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      https://api.github.com/repos/OWNER/REPO/topics \
      -d '{"names":["cat","atom","electron","api"]}'
     */
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}/topics");
    let body = serde_json::json!({
        "names": topics,
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .put(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// GitHub API create-a-github-pages-site
pub fn github_api_create_a_github_pages_site(
    owner: &str,
    repo_name: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
        https://docs.github.com/en/rest/pages/pages?apiVersion=2022-11-28#create-a-github-pages-site
        curl -L \
        -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/OWNER/REPO/pages \
        -d '
    {
        "source": {
            "branch": "main",
            "path": "/docs",
            "build_type": "workflow"
        }
    }'
         */
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}/pages");
    let body = serde_json::json!({
        "build_type": "workflow",
        "source": {
            "branch": "main",
            "path": "/docs"
        }
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}

/// Upload asset to github release  
pub fn github_api_upload_asset_to_release(
    github_client: &impl SendToGitHubApi,
    owner: &str,
    repo: &str,
    release_id: &str,
    path_to_file: &str,
) {
    println!("    {YELLOW}Uploading file to GitHub release: {path_to_file}{RESET}");
    let file = camino::Utf8Path::new(&path_to_file);
    let file_name = file.file_name().unwrap();

    let release_upload_url =
        format!("https://uploads.github.com/repos/{owner}/{repo}/releases/{release_id}/assets");
    let mut release_upload_url =
        <url::Url as std::str::FromStr>::from_str(&release_upload_url).unwrap();
    release_upload_url.set_query(Some(format!("{}={}", "name", file_name).as_str()));
    let file_size = std::fs::metadata(file).unwrap().len();
    println!("    {YELLOW}It can take some time to upload. File size: {file_size}. Wait...{RESET}");
    // region: async code made sync locally
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let file = tokio::fs::File::open(file).await.unwrap();
        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let body = reqwest::Body::wrap_stream(stream);

        let req = reqwest::Client::new()
            .post(release_upload_url.as_str())
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", file_size.to_string())
            .body(body);

        github_client.upload_to_github(req).await;
    });
    // endregion: async code made sync locally
}

/// Create new release on Github
pub fn github_api_create_new_release(
    owner: &str,
    repo: &str,
    tag_name_version: &str,
    name: &str,
    branch: &str,
    body_md_text: &str,
) -> reqwest::blocking::RequestBuilder {
    /*
    https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release
    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/OWNER/REPO/releases \
    -d '
    {
        "tag_name":"v1.0.0",
        "target_commitish":"master",
        "name":"v1.0.0",
        "body":"Description of the release",
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false
    }'

    Response (short)
    {
    "id": 1,
    ...
    }
    */
    let releases_url = format!("https://api.github.com/repos/{owner}/{repo}/releases");
    let body = serde_json::json!({
        "tag_name": tag_name_version,
        "target_commitish":branch,
        "name":name,
        "body":body_md_text,
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false,
    });
    let body = body.to_string();

    reqwest::blocking::Client::new()
        .post(releases_url.as_str())
        .header("Content-Type", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
}
