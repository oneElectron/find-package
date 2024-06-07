use find_common::{Pkg, PkgFile, PkgFileType};

pub(crate) async fn homebrew_get_package_list() -> Vec<Pkg> {
    println!("Homebrew started...");
    let client = reqwest::Client::builder().build().unwrap();
    let mut output: Vec<Pkg> = vec![];

    let resp = client
        .get("https://raw.githubusercontent.com/Homebrew/homebrew-command-not-found/master/executables.txt")
        .send()
        .await
        .unwrap();

    let txt = resp.text().await.unwrap();

    for line in txt.lines() {
        let s = parse_line(line);
        if s.is_none() {
            continue;
        }
        let s = s.unwrap();

        let pkg_files = {
            let mut output = vec![];

            for name in s.1 {
                output.push(PkgFile {
                    name: name.to_owned(),
                    file_type: PkgFileType::Binary,
                });
            }

            output
        };

        output.push(Pkg {
            name: s.0.to_owned(),
            pkg_files,
            pm_name: "homebrew",
        });
    }

    output
}

#[inline(always)]
fn parse_line(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut f = 0;
    let mut b = line.find(|c| c == '(' || c == ':').unwrap();

    let pkg_name = &line[f..b];
    let mut binaries: Vec<&str> = vec![];

    b = line.find(':').unwrap();

    loop {
        f = b + 1;
        b = 1
            + b
            + line[b + 1..].find(char::is_whitespace).unwrap_or_else(|| {
                binaries.push(&line[f..]);

                0
            });

        if f >= b {
            break;
        }

        binaries.push(&line[f..b])
    }

    if binaries.len() == 0 || binaries[0] == "" {
        return None;
    }

    Some((pkg_name, binaries))
}
