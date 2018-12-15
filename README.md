# Papago Rust Wrapper
[![Build Status](https://travis-ci.com/kkweon/papago-rust.svg?branch=master)](https://travis-ci.com/kkweon/papago-rust)


## Library

```rust
let auth = config::get_auth_from_env().unwrap();
let payload = PapagoReq {
    source: Lang::En,
    target: Lang::Ko,
    text: "HELLO WORLD".to_owned(),
};
let resp = send_request(NMT_URL, &auth, payload).unwrap();
assert_eq!(resp, "안녕 세계".to_owned());
```

## Bin File

### Set up Environment Variables

```bash
export NAVER_CLIENT_ID="..."
export NAVER_CLIENT_SECRET="..."
```

### Help
```
Papago API

USAGE:
    papago [FLAGS] [OPTIONS] <TEXT>

FLAGS:
    -h, --help       Prints help information
        --nmt        USE NMT API (default)
        --smt        USE SMT API
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>    Source language [default: en]  [possible values: ko, en, cn, tw, es, fr, vi, th, id]
    -t, --target <target>    Target language [default: ko]  [possible values: ko, en, cn, tw, es, fr, vi, th, id]

ARGS:
    <TEXT>    Text to translate
```
