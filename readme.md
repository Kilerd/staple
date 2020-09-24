<div align="center">
    <h1>Staple</h1>
    <p>powerful static site generator.</p>
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/kilerd/staple/Develop%20Build"> <img alt="Crates.io" src="https://img.shields.io/crates/v/staple"> <img src='https://coveralls.io/repos/github/Kilerd/staple/badge.svg?branch=master' alt='Coverage Status' /> <img alt="Crates.io (recent)" src="https://img.shields.io/crates/dr/staple"> <img alt="Crates.io" src="https://img.shields.io/crates/l/staple">
</div>

## Installation
currently, staple provide two ways to download its stable binary version. for those who is using rust can download it via Cargo:
```shell script
cargo install staple
```

or, you can download the latest version in [Github Release](https://github.com/Kilerd/staple/releases).
### Developer version
if you really need a developer version to taste some new feature which are not stable yet at the first time, you can download the latest staple code and compile it via Cargo:
```shell script
cargo install --git https://github.com/Kilerd/staple.git
```

## Usage
`staple` has full cli help text, run `staple --help` to get help text if you don't know how to use it.

### Init a staple project

for now, there are two ways to create a staple project for a folder.
 - for existed folder, you can use command `staple init` to initialize current working folder as a new staple project.
 - for non-existed folder, using command `staple new TARGET_FOLDER` to create a new folder for staple
