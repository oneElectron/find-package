use super::common::*;

use crate::Package;
use flate2::read::GzDecoder;
use reqwest::Client;

pub(crate) async fn pacman_get_package_list() -> Vec<Package> {
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

async fn pacman_get_core(client: &Client) -> Vec<Package> {
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

async fn pacman_get_extra(client: &Client) -> Vec<Package> {
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

fn parse_tar_archive<R>(mut archive: tar::Archive<R>) -> Vec<Package>
where
    R: std::io::Read,
{
    let mut output = vec![];
    let tmp = get_tmp_folder();

    archive.unpack(&tmp).unwrap();

    for folder in std::fs::read_dir(&tmp).unwrap() {
        let mut name = String::new();
        let mut files: Vec<String> = vec![];

        for file in std::fs::read_dir(folder.unwrap().path()).unwrap() {
            let file = file.unwrap();
            let filename = file.file_name();
            let filename = filename.to_str().unwrap().trim();

            if filename == "desc" {
                let c = std::fs::read_to_string(file.path()).unwrap();

                let r = pacman::parse_pacman_desc_file(&c);

                name = r.get_key_value("NAME").unwrap().1[0].clone();
            } else if filename == "files" {
                let c = std::fs::read_to_string(file.path()).unwrap();

                let r = pacman::parse_pacman_files_file(&c);

                files = r;
            }
        }

        let pkg_files = super::common::filter_path_list(files);

        let p = Package {
            name: name.to_string(),
            pm_name: "pacman",
            pkg_files,
        };

        output.push(p);
    }

    output
}
