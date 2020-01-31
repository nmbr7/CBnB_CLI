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
(@subcommand fas =>
            (about: "subcommand to deploy your functions")
            (version: "0.1.0")
            (author: "nmbr_7")
        )
    )
    .get_matches();

    match matches.subcommand_name() {
        Some("vm") => println!("Request you remote vm"),
        Some("docker") => println!("Deploy your docker machine"),
        Some("storage") => println!("Cbnb Storage at your service"),
        Some("fas") => println!("Deploy your Functions now"),
        _ => println!("No valid subcommand was used"),
    };

    /*if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    if let Some(o) = matches.value_of("i") {
        println!("Value for output: {}", o);
    }*/
}
