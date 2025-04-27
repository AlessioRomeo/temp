
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 85,
                        patch: 0,
                        pre: Prerelease::new("").unwrap(),
                        build: BuildMetadata::new("").unwrap(),
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.85.0 (4d91de4e4 2025-02-17)".to_owned(),
                    commit_hash: Some("4d91de4e48198da2e33413efdcd9cd2cc0c46688".to_owned()),
                    commit_date: Some("2025-02-17".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                    llvm_version: Some(LlvmVersion{ major: 19, minor: 1 }),
                }
            }
            