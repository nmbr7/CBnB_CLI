#[macro_use]
extern crate clap;
extern crate walkdir;

use serde_json::{json, Value};
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use walkdir::WalkDir;

mod message;

use message::{Message, ServiceMessage, ServiceMsgType, ServiceType};

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
        (@subcommand vm =>
            (about: "Subcommand to request remote virtual machines")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg ram: -r --ram +required +takes_value "Give the amount of required ram x(Mb/Gb)")
            (@arg cpu_cores: -c --cpu_cores +required +takes_value  "Give cpu core count")
        )
(@subcommand docker =>
            (about: "Subcommand to deploy a  docker machines")
            (version: "0.1.0")
            (author: "nmbr_7")
        )
(@subcommand storage =>
            (about: "Subcommand to use Cbnb storage solutions")
            (version: "0.1.0")
            (author: "nmbr_7")
            (@arg file: -f --file +required +takes_value "file to be stored")
            (@subcommand ls =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
            )
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

    match matches.subcommand() {
        ("vm", Some(vm_matches)) => println!("Request you remote vm"),
        ("docker", Some(docker_matches)) => println!("Deploy your docker machine"),
        ("storage", Some(storage_matches)) => {
            println!("Cbnb Storage at your service");
            let fname = storage_matches.value_of("file").unwrap().to_string();
            let mut file = File::open(&fname).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();

            let content = json!({
                "filename" :  fname,
                "content" : std::str::from_utf8(&buf).unwrap()})
            .to_string();

            let data = Message::Service(ServiceMessage {
                msg_type: ServiceMsgType::SERVICEINIT,
                service_type: ServiceType::Storage,
                content: content,
            });
            let msg_data = serde_json::to_string(&data).unwrap();
            //println!("{}",test["content"].as_str().unwrap(());
            let addr = matches.value_of("connect");
            let mut stream = TcpStream::connect(addr.unwrap()).unwrap();
            stream.write_all(msg_data.as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("Sent");
            let mut buffer = [0; 512];
            let no = stream.read(&mut buffer).unwrap();
            let mut data = std::str::from_utf8(&buffer[0..no]).unwrap();
            println!("Returned: {}", data);
            match storage_matches.subcommand() {
                ("ls", Some(ls)) => {}
                (_, _) => {}
            }
        }
        ("faas", Some(faas_matches)) => {
            println!("Deploy your Functions now");
            println!("client");
            let addr = matches.value_of("connect");
            let mut stream = TcpStream::connect(addr.unwrap()).unwrap();

            let msg_data = match faas_matches.subcommand() {
                ("create", Some(create_matches)) => {
                    let lang = create_matches.value_of("lang").unwrap().to_string();
                    let prototype = create_matches.value_of("prototype").unwrap().to_string();
                    let dir = create_matches.value_of("dir").unwrap().to_string();
                    let djson = dirjson(dir);
                    let content = format!("{{ \"msg_type\": \"MANAGE\" , \"action\": \"create\",\"lang\": {:?}, \"prototype\": {:?}, {} }}",lang, prototype, djson);
                    //TEST

                    let data = Message::Service(ServiceMessage {
                        msg_type: ServiceMsgType::SERVICEINIT,
                        service_type: ServiceType::Faas,
                        content: content,
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
