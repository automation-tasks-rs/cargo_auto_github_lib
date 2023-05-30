
/*
to get the releaseId use this:

curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer GITHUB_TOKEN"\
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/bestia-dev/cargo_auto_github_lib/releases
*/


/* use only on local machine when github_token is in env variable
#[test]
fn upload_asset_1() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        println!("upload_asset_1");
        cargo_auto_github_lib::auto_github_upload_asset_to_release(
            "bestia-dev",
            "cargo_auto_github_lib",
            "105426789",
            "tests/upload_test.txt",
        )
        .await;
    });
} */
