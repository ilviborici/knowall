<h1 align="center">
    knowall 
</h1>

<div align="center">
  ⚡ 🦀 🔍
</div>
<div align="center">
  <strong>Just ask "knowall"!</strong>
</div>
<div align="center">
  The fastest way to identify anything
</div>

<br />

Can be used for identifying mysterious text or to analyze hard-coded strings from captured network packets, malwares, or just about anything.

<br />

<br />


## Usage

```shell
knowall [OPTIONS] <TEXT/FILENAME>
```
<details>
<summary>
JSON Output
</summary>

If you want output in JSON format, then pass `-j / --json` flag.
*e.g.* 
```shell
knowall UC11L3JDgDQMyH8iolKkVZ4w --json
``` 
<img align="center" src="https://media.discordapp.net/attachments/998569651183288351/1009151747194892288/lkjosn.png?width=1440&height=512" alt="demo" />
</details>

> Run `knowall --help` for all options!

<br />

---
### Build it from source 🎯

Clone repository
```shell
git clone https://github.com/swanandx/knowall && cd knowall
```

then build and run

```shell
cargo run -- <TEXT/FILENAME> [OPTIONS]
```

OR

```shell
cargo build --release
cd target/release/
./knowall <TEXT/FILENAME> [OPTIONS]
```

