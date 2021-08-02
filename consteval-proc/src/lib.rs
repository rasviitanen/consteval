extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use std::io::Write;

#[proc_macro]
pub fn const_eval(ts: TokenStream) -> TokenStream {
    if std::env::var("SECOND_COMPILATION").is_err() {
        let path = std::env::temp_dir().join("const_crimes.rs");
        let gen_path = std::env::temp_dir();
        let mut file = std::fs::File::create(&path).unwrap();
        let ts = proc_macro2::TokenStream::from(ts);
        file.write_all(quote! { #ts }.to_string().as_bytes()).unwrap();

        let mut args = std::env::args_os();
        let mut link = std::ffi::OsString::new();
        while let Some(arg) = args.next() {
            if arg == "-L" {
                link = args.next().unwrap();
                break;
            }
        }
        let link = link.to_str().unwrap().split_once("=").unwrap().1;
        let _status = std::process::Command::new(std::env::args_os().next().unwrap())
            .args(&["--crate-name", "const_crimes"])
            .args(&["--extern", "quote"])
            .args(&["-L", link])
            .args(&["--crate-type", "proc-macro", path.to_str().unwrap(),
                    "--out-dir", gen_path.to_str().unwrap()]).status().unwrap();

        let mut args = std::env::args_os();
        let status = std::process::Command::new(args.next().unwrap())
            .args(&["--cfg", "second_compilation"])
            .args(args)
            .args(&["--extern", format!("const_crimes={}", gen_path.join("const_crimes.dll").to_str().unwrap()).as_str()])
            .env("SECOND_COMPILATION", "true")
            .status().unwrap();
        std::process::exit(status.code().unwrap_or(1));
    }

    ts
}