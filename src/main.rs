#[macro_use]
extern crate clap;
extern crate walkdir;

use indicatif::{ProgressBar, ProgressStyle};
use native_tls::TlsConnector;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

use chrono::{NaiveDateTime, Utc};
mod message;

use message::{Message, ServiceMessage, ServiceMsgType, ServiceType};
fn sendfile(filename: String, addr: String, id: String) {
    let mut file = File::open(&filename).unwrap();
    // TODO Take Chunks of data
    let mut buf = Vec::new();
    let filesize = file.metadata().unwrap().len();
    file.read_to_end(&mut buf).unwrap();

    let content = json!({
        "id"       : id,
        "msg_type" :  "write",
        "filename" :  filename.split('/').collect::<Vec<&str>>().last(),
        "filesize" :  filesize,
    })
    .to_string();

    let data = Message::Service(ServiceMessage {
        msg_type: ServiceMsgType::SERVICEINIT,
        service_type: ServiceType::Storage,
        content: content,
        uuid: id,
    });
    let msg_data = serde_json::to_string(&data).unwrap();
    //println!("{}",test["content"].as_str().unwrap(());

    let mut resp = [0; 512];

    let mut stream = TcpStream::connect(&addr).unwrap();
    /*
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(&addr).unwrap();
    let mut stream = connector.connect(&addr.split(":").collect::<Vec<&str>>()[0], stream).unwrap();
    */
    use std::io::BufWriter;
    let mut bufstream = BufWriter::new(stream.try_clone().unwrap());
    for i in msg_data.as_bytes() {
        bufstream.write(&[*i]).unwrap();
    }
    bufstream.flush().unwrap();
    //println!("{:?}", msg_data);
    //stream.write_all(msg_data.as_bytes()).unwrap();
    //stream.flush().unwrap();
    let no = stream.read(&mut resp).unwrap();

    if std::str::from_utf8(&resp[0..no]).unwrap() == "OK" {
        for i in buf {
            bufstream.write(&[i]).unwrap();
        }
        bufstream.flush().unwrap();

        // stream.write_all(&buf).unwrap();
        //     stream.flush().unwrap();
        println!("Sent Successfull");
    }
    let mut buffer = [0; 512];
    let no = stream.read(&mut buffer).unwrap();
    let mut data = std::str::from_utf8(&buffer[0..no]).unwrap();
    println!("Returned: {}", data);
}

pub fn getfile(filename: String, addr: String, id: String, dest: &String) {
    let content = json!({
        "msg_type" :  "read",
        "filename" :  filename,
        "id"       :  id,
    })
    .to_string();

    let data = Message::Service(ServiceMessage {
        msg_type: ServiceMsgType::SERVICEINIT,
        service_type: ServiceType::Storage,
        content: content,
        uuid: id,
    });

    let msg_data = serde_json::to_string(&data).unwrap();
    //println!("{}",test["content"].as_str().unwrap(());

    let mut resp = [0; 2048];
    let mut destbuffer = [0 as u8; 2048];

    let mut stream = TcpStream::connect(addr).unwrap();
    /*
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(&addr).unwrap();
    let mut stream = connector.connect(&addr.split(":").collect::<Vec<&str>>()[0], stream).unwrap();
    */

    //println!("{:?}", msg_data);
    stream.write_all(msg_data.as_bytes()).unwrap();
    stream.flush().unwrap();

    let no = stream.read(&mut resp).unwrap();
    println!("{}", std::str::from_utf8(&resp[0..no]).unwrap());
    let fsize: Value = serde_json::from_slice(&resp[0..no]).unwrap();
    let filesize = fsize["total_size"].as_u64().unwrap() as usize;

    stream.write_all(String::from("OK").as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut totalfilesize = 0 as usize;

    let pb = ProgressBar::new(filesize as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
    loop {
        let no = stream.read(&mut resp).unwrap();
        stream.write_all(String::from("OK").as_bytes()).unwrap();
        stream.flush().unwrap();
        //println!("val {:?}",std::str::from_utf8(&resp[0..no]).unwrap());
        let metadata: Value = serde_json::from_slice(&resp[0..no]).unwrap();
        //println!("{}",metadata);
        if metadata["msg_type"].as_str().unwrap() == "End" {
            break;
        }

        let size = metadata["size"].as_u64().unwrap() as usize;
        let index = metadata["index"].as_u64().unwrap();
        let mut total = 0 as usize;
        let mut bufvec: Vec<u8> = vec![];
        let mut destbuffer: Vec<u8> = vec![];
        loop {
            //let mut dno = stream.read_to_end(&mut destbuffer).unwrap();
            for byte in stream.try_clone().unwrap().bytes() {
                total += 1;
                destbuffer.push(byte.unwrap());
                if total >= size {
                    break;
                }
            }
            bufvec.append(&mut destbuffer[0..total].to_vec());
            //println!("Total: {} - Size {}",total,size);
            if total >= size {
                stream.write_all(String::from("OK").as_bytes()).unwrap();
                stream.flush().unwrap();
                break;
            }
        }
        totalfilesize += total;
        pb.set_position(totalfilesize as u64);
        {
            use std::fs::OpenOptions;
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(dest.clone())
                .unwrap();
            let val = file.seek(SeekFrom::Start(index * 4046848)).unwrap();
            //println!("seeked to offset {}",val);
            //let mut contents = vec![];
            //let mut handle = file.take(size)i;
            file.write_all(&bufvec.as_slice()).unwrap();
            file.flush().unwrap();
        }
        //println!("val {:?}",std::str::from_utf8(&resp[0..no]).unwrap());
        if totalfilesize >= filesize {
            println!("Total filesize downloaded : {}", totalfilesize);
            break;
        }
    }
    pb.finish_with_message("downloaded");
    println!(
        "File Download complete, Total File Size : {} bytes",
        totalfilesize
    );
}
/*
fn getfile(filename: String, addr: String, id: String) {
    let content = json!({
        "msg_type" :  "read",
        "filename" :  filename,
        "id"       :  id,
    })
    .to_string();

    let data = Message::Service(ServiceMessage {
        msg_type: ServiceMsgType::SERVICEINIT,
        service_type: ServiceType::Storage,
        content: content,
        uuid: id,
    });

    let msg_data = serde_json::to_string(&data).unwrap();
    //println!("{}",test["content"].as_str().unwrap(());

    let mut resp = [0; 2048];
    let mut destbuffer = [0 as u8; 2048];

    let mut stream = TcpStream::connect(addr).unwrap();
    //println!("{:?}", msg_data);
    stream.write_all(msg_data.as_bytes()).unwrap();
    stream.flush().unwrap();

    let no = stream.read(&mut resp).unwrap();
    let fsize: Value = serde_json::from_slice(&resp[0..no]).unwrap();
    let filesize = fsize["total_size"].as_u64().unwrap() as usize;

    let mut totalfilesize = 0 as usize;
    loop {
        let no = stream.read(&mut resp).unwrap();
        //println!("val {}",std::str::from_utf8(&resp[0..no]).unwrap());
        let metadata: Value = serde_json::from_slice(&resp[0..no]).unwrap();
        //println!("{}",metadata);
        if metadata["msg_type"].as_str().unwrap() == "End" {
            break;
        }

        let size = metadata["size"].as_u64().unwrap() as usize;
        let index = metadata["index"].as_u64().unwrap();
        let mut total = 0 as usize;
        let mut bufvec: Vec<u8> = vec![];
        stream.write_all(String::from("OK").as_bytes()).unwrap();
        stream.flush().unwrap();
        loop {
            // ERROR hangs when size is 13664 so fetch the total file size first and if   \
            //       the size is less than 65536 before reaching the end request for ret- \
            //       ransmission
            let mut dno = stream.read(&mut destbuffer).unwrap();
            if dno > size {
                dno = size;
            }
            total += dno;
            bufvec.append(&mut destbuffer[0..dno].to_vec());
            //println!("Total: {} - dno: {} - Size {}",total,dno,size);
            if total == size {
                break;
            }
        }

        {
            use std::fs::OpenOptions;
            let mut file = OpenOptions::new()
                .write(true)
                .open("./storage.bin")
                .unwrap();
            //file.set_len(21312864).unwrap();
            let val = file.seek(SeekFrom::Start(index * 65536)).unwrap();
            //println!("seeked to offset {}",val);
            //let mut contents = vec![];
            //let mut handle = file.take(size)i;
            file.write_all(&bufvec.as_slice()).unwrap();
            file.flush().unwrap();
        }
        totalfilesize += total;
        if totalfilesize == filesize {
            break;
        }
    }
    println!(
        "File Download complete, Total File Size : {} bytes",
        totalfilesize
    );
}
*/
fn query(query: String, addr: String, id: String) {
    let content = json!({
        "msg_type" :  "query",
        "queryname" :  query,
        "id"       :  id,
    })
    .to_string();

    let data = Message::Service(ServiceMessage {
        msg_type: ServiceMsgType::SERVICEINIT,
        service_type: ServiceType::Storage,
        content: content,
        uuid: id,
    });

    let msg_data = serde_json::to_string(&data).unwrap();
    //println!("{}",test["content"].as_str().unwrap(());

    let mut resp = [0; 2048];

    let mut stream = TcpStream::connect(addr).unwrap();

    /*
    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(&addr).unwrap();
    let mut stream = connector.connect(&addr.split(":").collect::<Vec<&str>>()[0], stream).unwrap();
    */

    //println!("{:?}", msg_data);
    stream.write_all(msg_data.as_bytes()).unwrap();
    stream.flush().unwrap();

    let no = stream.read(&mut resp).unwrap();
    let query_response: HashMap<String, HashMap<String, Value>> =
        serde_json::from_slice(&resp[0..no]).unwrap();
    println!("\n{:<15}  {:<10}", "Name", "Size");
    println!("{:<15}  {:<10}", "----", "----");
    for (k, v) in query_response {
        println!("{:<15}  {:<10}", k, v["size"]);
    }
    println!();
}

fn dirjson(dir: String) -> String {
    let mut directory: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() {
            directory.push(entry.path().to_str().unwrap().to_string());
        } else {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    let mut all: Vec<String> = Vec::new();
    for i in &files {
        let mut file = File::open(&i).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let test = format!(" {:?} : {:?} ", i, std::str::from_utf8(&buf).unwrap());
        all.push(test);
        //let format!("{}",test);
    }
    let dirs = format!("\"dirs\" : {:?}", directory);
    let file_name = format!("\"file_name\" : {:?}", files);
    let file_data = format!("\"files\" : {{ {} }}", all.join(",")).replace("\'", "\\u{27}");
    let all = format!(" {} , {} , {} ", dirs, file_name, file_data);
    all
}

fn main() {
    let matches = clap_app!(Cbnb_CLI =>
        (version: "0.1.0")
        (author: "nmbr_7")
        (about: "CloudBnB Service CLI")
        (@arg connect: -c --connect +takes_value +required "destination addr and port")
        (@arg userid: -uid --userid +takes_value +required "user api key")
(@subcommand paas =>
            (about: "Subcommand to deploy a  docker machines")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@subcommand deploy =>
            	(about: "deploy the app")
            	(version: "0.1.0")
            	(author: "nmbr_7")
             (@arg runtime: -rt --runtime +required +takes_value "app runtime language")
        )
            )
(@subcommand storage =>
            (about: "Subcommand to use Cbnb storage solutions")
            (version: "0.1.0")
            (author: "nmbr_7")
            )
(@subcommand faas =>
            (about: "subcommand to deploy your functions")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@subcommand create =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
             (@arg lang: -l --lang +takes_value "function language")
             (@arg prototype: -p --proto +takes_value "function language")
             (@arg dir: -d --dir +takes_value "Function Directory (Directory must contain the function prototype file, funtion definition file, dependency modules and config files MAX Size should be less than 5MB)")
             )
            (@subcommand update =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifie +required  +takes_value "function id")
            )
            (@subcommand delete =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifier +required +takes_value "function id")
                (@arg file: -f --file +required +takes_value "file to be stored")
            )
            (@subcommand publish =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifier +required +takes_value "function id")
            )
        )
    )
    .get_matches();

    let addr = matches.value_of("connect").unwrap();
    let userid = matches.value_of("userid").unwrap().to_string();
    match matches.subcommand() {
        ("paas", Some(paas_matches)) => {
            let mut stream = TcpStream::connect(addr).unwrap();
            /*
            let connector = TlsConnector::new().unwrap();
            let stream = TcpStream::connect(addr).unwrap();
            let mut stream = connector.connect(addr, stream).unwrap();
            */
            let msg_data = match paas_matches.subcommand() {
                ("deploy", Some(deploy_matches)) => {
                    if !Path::new("./app.toml").exists() {
                        println!("Error: app.toml file not found");
                        return;
                    }
                    let runtime = deploy_matches.value_of("runtime").unwrap().to_string();
                    //let parse_from_str = NaiveDateTime::parse_from_str;
                    //let a = parse_from_str(&utc, "%s").unwrap();
                    //let timestamp = Utc::now().timestamp().to_string();

                    let content = json!({
                        "msg_type": "deploy",
                        "runtime":runtime,
                    })
                    .to_string();

                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEINIT,
                        service_type: ServiceType::Paas,
                        content: content,
                        uuid: userid.clone(),
                    });
                    let msg = serde_json::to_string(&data).unwrap();
                    stream.write_all(msg.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    let mut buffer = [0; 512];
                    let no = stream.read(&mut buffer).unwrap();

                    let mut data: Value = serde_json::from_slice(&buffer[0..no]).unwrap();

                    let fname = data["filename"].as_str().unwrap().to_string();
                    let fname = format!("/tmp/{}", fname);
                    println!("Writing to zip file at {}", fname);
                    let output = Command::new("zip")
                        .args(&["-r", &fname, ".", "-x", "target*", ".git*"])
                        .output()
                        .expect("Failed to compress the file, Check whether zip is installed");

                    // Upload the app directory to the server
                    sendfile(fname.to_string(), addr.to_string(), userid.clone());
                    stream
                        .write_all(
                            json!({
                                "UploadStatus":"OK"
                            })
                            .to_string()
                            .as_bytes(),
                        )
                        .unwrap();
                    stream.flush().unwrap();
                    let no = stream.read(&mut buffer).unwrap();
                    let data = std::str::from_utf8(&buffer[0..no]).unwrap();
                    println!("{}", data);
                }
                _ => return,
            };
        }
        ("storage", Some(storage_matches)) => {
            print!("\x1B[H\x1B[2J");
            println!("Welcome to -- CBnB Storage -- shell");
            let coms: Vec<String> = vec!["ls", "upload", "download", "help", "exit"]
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
            loop {
                let mut line = String::new();
                print!("CBnB > ");
                std::io::stdout().flush().unwrap();
                let b1 = std::io::stdin().read_line(&mut line).unwrap();
                let args: Vec<&str> = line.trim().trim_matches('\n').split(' ').collect();
                match [args[0]] {
                    ["ls"] => {
                        if args.len() == 2 {
                            match [args[1]] {
                                ["remote"] => {
                                    query("ls".to_string(), addr.to_string(), userid.clone())
                                }
                                [cmd] => println!("No Subcommand : {}", cmd),
                            }
                        } else {
                            let output = Command::new("ls")
                                .output()
                                .expect("Error Running the function");
                            let response = std::str::from_utf8(&output.stdout).unwrap();
                            println!("{}", response.replace("\n", " "));
                        }
                    }

                    ["upload"] => {
                        if args.len() != 2 || args.len() > 2 {
                            println!("Please specify a filename to upload ");
                            continue;
                        }
                        match [args[1]] {
                            [filename] => {
                                sendfile(filename.to_string(), addr.to_string(), userid.clone());
                            }
                            _ => println!("Please specify a filename to upload"),
                        }
                    }
                    ["download"] => match [args[1]] {
                        [filename] => {
                            getfile(
                                filename.to_string(),
                                addr.to_string(),
                                userid.clone(),
                                &filename.to_string(),
                            );
                        }
                        _ => println!("Please specify a filename to download"),
                    },
                    ["clear"] => {
                        print!("\x1B[H\x1B[2J");
                    }
                    ["help"] => {
                        println!(" --------------------------\n The Available Commands are:\n {}\n --------------------------- ",coms.join("\n "));
                    }
                    ["exit"] => break,
                    _ => println!("Command not found: {}", line),
                }
            }
        }
        ("faas", Some(faas_matches)) => {
            println!("Deploy your Functions now");
            println!("client");
            let addr = matches.value_of("connect").unwrap();

            let mut stream = TcpStream::connect(addr).unwrap();
            /*
                        let connector = TlsConnector::new().unwrap();
                        let stream = TcpStream::connect(addr).unwrap();
                        let mut stream = connector.connect(addr, stream).unwrap();
            */
            let msg_data = match faas_matches.subcommand() {
                ("create", Some(create_matches)) => {
                    let lang = create_matches.value_of("lang").unwrap().to_string();
                    let prototype = create_matches.value_of("prototype").unwrap().to_string();
                    let dir = create_matches.value_of("dir").unwrap().to_string();
                    let djson = dirjson(dir);

                    // TODO Reimplement the json construction
                    let content = format!("{{ \"msg_type\": \"MANAGE\" , \"action\": \"create\",\"lang\": {:?}, \"prototype\": {:?}, {} }}",lang, prototype, djson);
                    //TEST

                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEINIT,
                        service_type: ServiceType::Faas,
                        content: content,
                        uuid: userid,
                    });

                    Ok(serde_json::to_string(&data).unwrap())

                    //println!("{}",data);   //stream.write(data.as_bytes()).unwrap();  //stream.flush().unwrap();
                }
                ("update", Some(update_matches)) => {
                    let id = update_matches.value_of("id");
                    let content = json!({
                        "msg_type": "MANAGE",
                    })
                    .to_string();

                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEUPDATE,
                        service_type: ServiceType::Faas,
                        content: content,
                        uuid: userid,
                    });

                    Ok(serde_json::to_string(&data).unwrap())
                }
                ("delete", Some(delete_matches)) => {
                    let id = delete_matches.value_of("id");
                    let content = json!({
                        "msg_type": "MANAGE", "action": "delete", "id": id
                    })
                    .to_string();
                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEUPDATE,
                        service_type: ServiceType::Faas,
                        content: content,
                        uuid: userid,
                    });

                    Ok(serde_json::to_string(&data).unwrap())
                }
                ("publish", Some(publish_matches)) => {
                    let id = publish_matches.value_of("id");
                    let content = json!({
                        "msg_type": "MANAGE",
                        "action": "publish",
                        "id": id
                    })
                    .to_string();
                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEUPDATE,
                        service_type: ServiceType::Faas,
                        content: content,
                        uuid: userid,
                    });

                    Ok(serde_json::to_string(&data).unwrap())
                }
                (&_, _) => Err("No valid subcommand was used"),
            };
            stream.write_all(msg_data.unwrap().as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("Sent");
            let mut buffer = [0; 512];
            let no = stream.read(&mut buffer).unwrap();
            let mut data = std::str::from_utf8(&buffer[0..no]).unwrap();
            println!("Returned: {}", data);
        }
        (&_, _) => println!("No valid subcommand was used"),
    };
}
