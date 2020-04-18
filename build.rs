// Copyright 2020 Sebastian Wiesner <sebastian@swsnr.de>

// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use std::env;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::process::Command;

fn generate_manpage<P: AsRef<Path>>(outdir: P) -> Result<()> {
    let result = Command::new("asciidoctor")
        .arg("-b")
        .arg("manpage")
        .arg("-o")
        .arg(outdir.as_ref().join("git-gone.1"))
        .arg("git-gone.1.adoc")
        .spawn()?
        .wait()?;
    if !result.success() {
        Err(Error::new(
            ErrorKind::Other,
            format!("asciidoctor failed with code: {:?}", result.code()),
        ))
    } else {
        Ok(())
    }
}

fn main() {
    println!("cargo:rerun-if-changed=git-gone.1.adoc");
    let out = env::var_os("OUT_DIR").expect("Cargo did not define $OUT_DIR");
    if let Err(error) = generate_manpage(&out) {
        println!("cargo:warning=Failed to generate manpage: {}", error);
    }
}
