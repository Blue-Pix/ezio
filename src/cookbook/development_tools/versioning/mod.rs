use semver::{Version, SemVerError, Identifier, VersionReq};
use std::process::Command;

pub fn parse() -> Result<(), SemVerError> {
  let mut parsed_version = Version::parse("0.2.6")?;
  assert_eq!(
    parsed_version,
    Version {
      major: 0,
      minor: 2,
      patch: 6,
      pre: vec![],
      build: vec![],
    }
  );

  parsed_version.increment_patch();
  println!("New patch release: v{}", parsed_version.to_string());
  parsed_version.increment_minor();
  println!("New mindor release: v{}", parsed_version.to_string());
  parsed_version.increment_major();
  println!("New major release: v{}", parsed_version.to_string());

  Ok(())
}

pub fn complex_parse() -> Result<(), SemVerError> {
  let version_str = "1.0.49-125+g72ee7853";
  let parsed_version = Version::parse(version_str)?;
  assert_eq!(
    parsed_version,
    Version {
      major: 1,
      minor: 0,
      patch: 49,
      pre: vec![Identifier::Numeric(125)],
      build: vec![],
    }
  );
  assert_eq!(
    parsed_version.build,
    vec![Identifier::AlphaNumeric(String::from("g72ee7853"))]
  );

  let serialized_version = parsed_version.to_string();
  assert_eq!(&serialized_version, version_str);
  Ok(())
}

pub fn check_pre_release() -> Result<(), SemVerError> {
  let version_1 = Version::parse("1.0.0-alpha")?;
  let version_2 = Version::parse("1.0.0")?;
  assert!(version_1.is_prerelease());
  assert!(!version_2.is_prerelease());
  Ok(())
}

pub fn find_latest_version() -> Result<(), String> {
  assert_eq!(
    find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
    Some(Version::parse("1.0.0").unwrap())
  );

  assert_eq!(
    find_max_matching_version(
      ">1.2.3-alpha.3",
      vec![
        "1.2.3-alpha.3",
        "1.2.3-alpha.4",
        "1.2.3-alpha.10",
        "1.2.3-beta.4",
        "3.4.5-alpha.9",
      ]
    )?,
    Some(Version::parse("1.2.3-beta.4").unwrap())
  );
  Ok(())
}

fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I)
 -> Result<Option<Version>, String> 
 where I: IntoIterator<Item = &'a str>,
{
  let vreq = VersionReq::parse(version_req_str).expect("Failed to parse version req");
  Ok(
    iterable.into_iter()
            .filter_map(|s| Version::parse(s).ok())
            .filter(|s| vreq.matches(s))
            .max(),
  )
}

pub fn external_command_version() -> Result<(), String> {
  let version_constraint = "> 1.12.0";
  let version_test = VersionReq::parse(version_constraint).expect("Failed to parse version req");
  let output = Command::new("git").arg("--version").output().expect("Failed to execute command");

  if !output.status.success() {
    return Err(String::from("Command executed with failing error code"));
  }

  let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout string");
  let version = stdout.split(" ").last().ok_or_else(|| {
    "Invalid command output"
  })?;
  let parsed_version = Version::parse(version).expect("Failed to parse version");

  if !version_test.matches(&parsed_version) {
    return Err(
      format!(
        "Command version lower than minimum supported version (found {} need {})",
        parsed_version,
        version_constraint
      )
    )
  }
  println!("You are using git version: {}", parsed_version);
  Ok(())
}