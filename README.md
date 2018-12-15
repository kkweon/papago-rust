# Papago Rust Wrapper

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
