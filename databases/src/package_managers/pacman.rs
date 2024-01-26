use crate::Pkg;
use flate2::read::GzDecoder;
use reqwest::Client;
use std::io::prelude::*;

// I did not try to make this efficient, so don't go thinking I am a bad programmer.

// const MIRROR_LIST: &str = include_str!("pacman.mirrorlist"); // for later

struct ArchLinuxPkg {}

pub(crate) async fn pacman_get_package_list() -> Vec<Pkg> {
    println!("Pacman started...");
    let client = Client::builder().build().unwrap();

    client
        .get("https://archlinux.org/mirrorlist/all/")
        .send()
        .await
        .unwrap();

    let core = pacman_get_core(&client);
    let extra = pacman_get_extra(&client);

    let (mut core, mut extra) = tokio::join!(core, extra);

    let mut output = vec![];

    output.append(&mut core);
    output.append(&mut extra);

    output
}

async fn pacman_get_core(client: &Client) -> Vec<Pkg> {
    let r = client
        .get("https://mirror.sunred.org/archlinux/core/os/x86_64/core.files.tar.gz")
        .send()
        .await
        .unwrap();

    let r = r.bytes().await.unwrap().as_ref().to_vec();

    let g = GzDecoder::new(r.as_slice());

    let tar = tar::Archive::new(g);

    parse_tar_archive(tar)
}

async fn pacman_get_extra(client: &Client) -> Vec<Pkg> {
    let r = client
        .get("https://mirror.sunred.org/archlinux/extra/os/x86_64/extra.files.tar.gz")
        .send()
        .await
        .unwrap();

    let r = r.bytes().await.unwrap().as_ref().to_vec();

    let g = GzDecoder::new(r.as_slice());

    let tar = tar::Archive::new(g);

    parse_tar_archive(tar)
}

// This is a horrible function, please rewrite it at some point
fn parse_tar_archive<T>(mut archive: tar::Archive<T>) -> Vec<Pkg>
where
    T: std::io::Read,
{
    let start_time = std::time::Instant::now();

    let mut output = vec![];

    let mut real_names: Vec<(String, String)> = vec![];

    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.header().path().unwrap().to_path_buf();
        let name = path.parent().unwrap().to_string_lossy().to_string();

        let mut buf: String = String::new();

        entry.read_to_string(&mut buf).unwrap();

        if entry.header().size().unwrap() != 0 && path.file_name().unwrap() == "files" {
            let mut pkg: Pkg = Pkg {
                name: path.parent().unwrap().to_string_lossy().to_string(),
                pkg_files: vec![],
                pm_name: "pacman",
            };

            for line in buf.lines() {
                if line.starts_with('%') {
                    continue;
                }

                let b = if let Some(s) = line.rfind('/') {
                    s
                } else {
                    continue;
                };

                let f = if let Some(s) = &line[..b].rfind('/') {
                    s + 1
                } else {
                    continue;
                };

                if &line[f..b] == "bin" && !line[b + 1..].is_empty() {
                    pkg.pkg_files.push(crate::package::PkgFile {
                        name: line[b + 1..].to_string(),
                        file_type: crate::package::PMFileType::Binary,
                    })
                }
            }

            output.push(pkg);
        }

        if entry.header().size().unwrap() != 0 && path.file_name().unwrap() == "desc" {
            let mut i = buf.lines().into_iter();

            loop {
                if let Some(line) = i.next() {
                    if line.contains("%NAME%") {
                        real_names.push((i.next().unwrap().trim().to_string(), name));
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    for (real_name, name_with_version) in real_names {
        replace_name_in_pkg(real_name, name_with_version, &mut output);
    }

    let end_time = std::time::Instant::now();

    println!(
        "Time taken for parse_tar_archive: {} ms",
        (end_time - start_time).as_millis()
    );

    output
}

fn replace_name_in_pkg(real_name: String, target_name: String, buf: &mut [Pkg]) {
    for pkg in buf {
        if pkg.name == target_name {
            pkg.name = real_name;
            break;
        }
    }
}
