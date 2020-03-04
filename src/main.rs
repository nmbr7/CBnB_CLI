#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(Cbnb_CLI =>
        (version: "0.1.0")
        (author: "nmbr_7")
        (about: "CloudBnB Service CLI")
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
            (@arg file: -f --file +takes_value "file to be stored")
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
                (@arg file: -f --file +takes_value "file to be stored")
            )
            (@subcommand update =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifier +takes_value "function id")
            )
            (@subcommand delete =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifier +takes_value "function id")
                (@arg file: -f --file +takes_value "file to be stored")
            )
            (@subcommand publish =>
            	(about: "list all the files")
            	(version: "0.1.0")
            	(author: "nmbr_7")
                (@arg id: -id --identifier +takes_value "function id")
            )
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("vm", Some(vm_matches)) => println!("Request you remote vm"),
        ("docker", Some(docker_matches)) => println!("Deploy your docker machine"),
        ("storage", Some(storage_matches)) => println!("Cbnb Storage at your service"),

        ("faas", Some(faas_matches)) => {
            println!("Deploy your Functions now");
            //TODO Use the rsless library
        }

        _ => println!("No valid subcommand was used"),
    };
}
