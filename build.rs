use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    EmitBuilder::builder()
        .git_sha(true)
        .git_describe(true, true, None)
        .git_branch()
        .git_commit_timestamp()
        .emit()?;

    Ok(())
}
