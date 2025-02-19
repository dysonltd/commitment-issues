use core::str;

use chrono::Utc;
use git2::{Commit, DescribeOptions, Repository, StatusOptions};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::Error;

const HEADER: [u8; 4] = [0xFF, 0xFE, 0xFD, 0xFC];
const FOOTER: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
const SCHEMA_VERSION: u8 = 1;

const BUILD_TIME_LENGTH: usize = 20;
const HASH_LENGTH: usize = 10;
const TAG_LENGTH: usize = 20;
const AUTHOR_LENGTH: usize = 20;

#[macro_use]
mod macros;

#[proc_macro]
pub fn include_metadata(_: TokenStream) -> TokenStream {
    let header = HEADER;
    let build_time = build_time();

    let repo = propagate_error!(get_repo());
    let last_commit = propagate_error!(get_last_commit(&repo));

    let short_hash = get_short_hash(&last_commit);
    let is_dirty = propagate_error!(is_dirty(&repo));
    let tag_describe = propagate_error!(get_tag_describe(&repo));
    let last_author = propagate_error!(get_last_author(&last_commit));
    let footer = FOOTER;

    quote! {
        #[allow(unused_macros)]
        #[macro_use]
        mod metadata{
            use core::str;

            #[repr(C, packed)]
            struct Metadata {
                header: [u8; 4],
                schema: u8,
                compile_time: [u8; #BUILD_TIME_LENGTH],
                short_hash: [u8; #HASH_LENGTH],
                is_dirty: bool,
                tag_describe: [u8; #TAG_LENGTH],
                last_author: [u8; #AUTHOR_LENGTH],
                footer: [u8; 4],
            }

            #[link_section = ".metadata"]
            #[used]
            static METADATA: Metadata =
                    Metadata {
                        header: [#(#header),*],
                        schema: #SCHEMA_VERSION,
                        compile_time: [#(#build_time),*],
                        short_hash: [#(#short_hash),*],
                        is_dirty: #is_dirty,
                        tag_describe: [#(#tag_describe),*],
                        last_author: [#(#last_author),*],
                        footer: [#(#footer),*],
            };

            pub fn schema() -> u8 {
                METADATA.schema
            }

            pub fn compile_time<'a>() -> &'a str {
                let length = get_populated_length(&METADATA.compile_time);
                unsafe { str::from_utf8_unchecked(&METADATA.compile_time[..length]) }
            }

            pub fn short_hash<'a>() -> &'a str {
                unsafe { str::from_utf8_unchecked(&METADATA.short_hash) }
            }

            pub fn is_dirty() -> bool {
                METADATA.is_dirty
            }

            pub fn tag_describe<'a>() -> &'a str {
                let length = get_populated_length(&METADATA.tag_describe);
                unsafe { str::from_utf8_unchecked(&METADATA.tag_describe[..length]) }
            }

            pub fn last_author<'a>() -> &'a str {
                let length = get_populated_length(&METADATA.last_author);
                unsafe { str::from_utf8_unchecked(&METADATA.last_author[..length]) }
            }

            /// The byte slice is traversed in reverse order in this function.
            /// If we make the assumption that on the whole byte slices to this function are likely to be at least half filled, we can save cycles by starting at the end and working backwards.
            fn get_populated_length(bytes: &[u8]) -> usize {
                bytes.len() - bytes
                    .iter()
                    .rev()
                    .position(|byte| *byte != 0)
                    .unwrap_or_default()
            }
        }
    }
    .into()
}

fn build_time() -> [u8; BUILD_TIME_LENGTH] {
    copy_from(&Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
}

fn get_repo() -> Result<Repository, Error> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .output()
        .map_err(|error| {
            Error::new(
                Span::call_site(),
                format!("failed to run `cargo metadata`: {error}"),
            )
        })?;
    let cargo_metadata = core::str::from_utf8(&output.stdout).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("output from `cargo metadata` was not valid UTF8: {error}"),
        )
    })?;
    let pattern = Regex::new(r#"\"workspace_root\":\"(.*)\","#).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("failed to generate Regex: {error}"),
        )
    })?;
    let primary_package_dir = pattern
        .captures(cargo_metadata)
        .ok_or(Error::new(
            Span::call_site(),
            "key \"workspace_root\" not found in cargo metadata".to_owned(),
        ))?
        .get(1)
        .ok_or(Error::new(
            Span::call_site(),
            "key \"workspace_root\" not found in cargo metadata".to_owned(),
        ))?
        .as_str();
    Repository::open(primary_package_dir).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("failed to open repository at location {primary_package_dir}: {error}"),
        )
    })
}

fn get_last_commit(repo: &Repository) -> Result<Commit, Error> {
    let hash = repo
        .head()
        .map_err(|error| Error::new(Span::call_site(), format!("failed to get HEAD: {error}")))?
        .target()
        .ok_or(Error::new(
            Span::call_site(),
            "failed to get HEAD target hash",
        ))?;
    repo.find_commit(hash).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("failed to find commit with hash {hash}: {error}"),
        )
    })
}

fn get_short_hash(commit: &Commit) -> [u8; HASH_LENGTH] {
    let hash = commit.id().to_string();
    copy_from(&hash)
}

fn is_dirty(repo: &Repository) -> Result<bool, Error> {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    let is_clean = repo
        .statuses(Some(&mut status_options))
        .map_err(|error| {
            Error::new(
                Span::call_site(),
                format!("failed to get repo status: {error}"),
            )
        })?
        .is_empty();
    Ok(!is_clean)
}

fn get_tag_describe(repo: &Repository) -> Result<[u8; TAG_LENGTH], Error> {
    match repo.describe(DescribeOptions::new().describe_tags()) {
        Ok(describe) => {
            let tag = describe.format(None).map_err(|error| {
                Error::new(Span::call_site(), format!("failed to format tag: {error}"))
            })?;
            Ok(copy_from(&tag))
        }
        Err(_) => Ok([0; TAG_LENGTH]),
    }
}

fn get_last_author(commit: &Commit) -> Result<[u8; AUTHOR_LENGTH], Error> {
    let author = commit.author();
    let name = author.name().ok_or(Error::new(
        Span::call_site(),
        "failed to get last commit author",
    ))?;
    Ok(copy_from(name))
}

fn copy_from<const N: usize>(src: &str) -> [u8; N] {
    let mut buffer = [0; N];
    let src_bytes = src.as_bytes();
    let length = if buffer.len() < src_bytes.len() {
        buffer.len()
    } else {
        src_bytes.len()
    };

    let mut index = 0;
    while index < length {
        buffer[index] = src_bytes[index];

        index += 1;
    }

    buffer
}
