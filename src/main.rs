use reqwest::{self};
use tokio::fs::{self, File};
use std::fs::{self as s_fs, remove_dir_all, remove_file};
use tokio::io::{self, AsyncWriteExt};
use std::process::Command;
use std::string::String;
use encoding_rs::{GBK};
use lazy_static::lazy_static;
use std::env;
use std::ops::Deref;

lazy_static! {
    static ref BIN_PATH: String = {
        let output = Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("获取路径失败");
        let path = String::from_utf8_lossy(&(output.stdout));
        path.to_string().replace("\n", "")
    };
}

fn bin_rpath(filename: &str) -> String {
    let mut p = env::current_exe().unwrap();
    p.pop();
    format!("{}/{}", p.to_string_lossy(), filename)
}

async fn request_file() -> io::Result<()> {
    let client = reqwest::Client::builder().build().unwrap();
    let resp = match client.get("http://idea.medeming.com/jets/images/jihuoma.zip").send().await {
        Ok(resp) => resp,
        Err(err) => {
            println!("{}", err);
            return Err(io::Error::new(io::ErrorKind::NotConnected, err))
        }
    };
    let mut out = File::create("jihuoma.zip").await?;
    let bytes = resp.bytes().await.unwrap();
    out.write_all(&mut bytes.deref()).await?;
    // let mut bufs = bytes.to_vec();
    // let mut buf = 0;
    // loop {
    //     buf += out.write(&mut bufs[buf..]).await?;
    //     if buf >= bufs.len() {
    //         break
    //     }
    // }
    // println!("{}", bytes)
    Ok(())
}

#[cfg(target_os = "windows")]
fn open_code() {
    let mut output = Command::new("cmd")
        .args(&["/C", "7z", "x", "jihuoma.zip", "-ojihuoma/"])
        .output()
        .expect("failed to execute process");
    let d = GBK.decode(&mut output.stdout).0;
    println!("{}", d.to_string());

    let dirs = s_fs::read_dir("./jihuoma").unwrap();
    for dir in dirs {
        let f = dir.unwrap().file_name();
        let filename = f.to_string_lossy();
        match filename.find("2018") {
            Some(_) => {
                Command::new("cmd")
                    .args(&["/C", "notepad", &("jihuoma/".to_string() + &*filename.to_string())])
                    .output()
                    .expect("failed to execute process");
            },
            _ => ()
        }
    }
}

#[cfg(target_os = "macos")]
fn open_code() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("unzip -d jihuoma jihuoma.zip")
        .output()
        .expect("解压执行失败");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    let dirs = s_fs::read_dir("./jihuoma").unwrap();
    for dir in dirs {
        let f = dir.unwrap().file_name();
        let filename = f.to_string_lossy();
        match filename.find("2018") {
            Some(_) => {
                Command::new("sh")
                    // .args(&["-c", "open", "-e", format!("jihuoma/{}", filename).as_str()])
                    .arg("-c")
                    .arg(format!("pbcopy < \"jihuoma/{}\"", filename))
                    .output()
                    .expect("打开激活码失败");
                println!("激活码已复制到剪切板！");
            },
            _ => ()
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // let client = reqwest::Client::new();
    // let output = Command::new("sh")
    //     .arg("-c")
    //     .arg("pwd")
    //     .output()
    //     .expect("获取路径失败");
    // println!("{}", bin_rpath("jihuoma.zip"));
    request_file().await.unwrap();

    open_code();

    remove_file("jihuoma.zip").unwrap();
    remove_dir_all("./jihuoma").unwrap();

    // io::copy(&mut bytes.to_vec(), &mut out).await?;
    /*for mut b in &bytes.to_vec() {
        println!("{}", b);
    }*/
    // out.write(&mut bytes).await?;
    // io::copy(&mut resp, &mut out).unwrap();
    Ok(())
}
