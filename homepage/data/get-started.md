 - title = Get Started
 - url = get-started
 - datetime = 2020-09-24T20:13:17.909933+08:00
 - template = article.html
 - draw = false


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
