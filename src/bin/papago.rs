use papago::argparse::get_args;
use papago::send_request;

fn main() {
    let args = get_args();
    let auth = papago::config::get_auth_from_env().unwrap();
    let result = send_request(args.api, &auth, args.payload);

    println!("{}", result.unwrap());
}
