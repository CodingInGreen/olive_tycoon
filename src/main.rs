use clap::{App, Arg, SubCommand};

#[derive(Debug)]
enum OliveTreeSpecies {
    Galega,
    Cobrancosa,
    Cordovil,
    Verdeal,
}

impl OliveTreeSpecies {
    fn from_str(species: &str) -> Option<Self> {
        match species {
            "Galega" => Some(Self::Galega),
            "Cobrancosa" => Some(Self::Cobrancosa),
            "Cordovil" => Some(Self::Cordovil),
            "Verdeal" => Some(Self::Verdeal),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct OliveTree {
    species: OliveTreeSpecies,
    orchard_name: String,
    color: String,
    age: i32,
    planter: OlivePicker,
}

#[derive(Debug, Clone)]
struct OlivePicker {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct Orchard {
    name: String,
    price: f32,
}

impl Orchard {
    fn new(name: String, price: f32) -> Self {
        Self { name, price }
    }
}

fn main() {
    let matches = App::new("Olive Tycoon")
        .version("0.0.1")
        .author("Adam Hott")
        .about("Rule the world through olives.")
        .subcommand(
            SubCommand::with_name("buy")
                .about("Buys a new orchard")
                .arg(Arg::with_name("name")
                    .help("The name of the orchard")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("price")
                    .help("The price of the orchard")
                    .required(true)
                    .index(2)),
        )
        .subcommand(
            SubCommand::with_name("plant")
                .about("Plants a new olive tree")
                .arg(Arg::with_name("species")
                    .help("The species of the olive tree")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("color")
                    .help("The color of the olive tree")
                    .required(true)
                    .index(2))
                .arg(Arg::with_name("age")
                    .help("The age of the olive tree")
                    .required(true)
                    .index(3))
                .arg(Arg::with_name("name")
                    .help("The name of the orchard where to plant the tree")
                    .required(true)
                    .index(4)),
        )
        .subcommand(
            SubCommand::with_name("hire")
                .about("Hires a new olive picker")
                .arg(Arg::with_name("first_name")
                    .help("The first name of the olive picker")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("last_name")
                    .help("The last name of the olive picker")
                    .required(true)
                    .index(2)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("buy") {
        let name = matches.value_of("name").unwrap();
        let price: f32 = matches.value_of("price").unwrap().parse().expect("Price must be a number");
        let orchard = Orchard::new(name.to_string(), price);
        println!("You bought {:?}", orchard);
    }

    if let Some(matches) = matches.subcommand_matches("hire") {
        let first_name = matches.value_of("first_name").unwrap();
        let last_name = matches.value_of("last_name").unwrap();
        let olive_picker = OlivePicker {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };
        println!("You hired {} {}", olive_picker.first_name, olive_picker.last_name);
    }

    if let Some(matches) = matches.subcommand_matches("plant") {
        let species_str = matches.value_of("species").unwrap();
        let species = OliveTreeSpecies::from_str(species_str).expect("Invalid species");
        let color = matches.value_of("color").unwrap();
        let age: i32 = matches.value_of("age").unwrap().parse().expect("Age must be a number");
        let orchard_name = matches.value_of("name").unwrap();
        // Example of using a placeholder. In a real application, we'd fetch or create a real OlivePicker based on application logic.
        let planter = OlivePicker { first_name: "Placeholder".to_string(), last_name: "Picker".to_string() };
        let tree = OliveTree {
            species,
            orchard_name: orchard_name.to_string(),
            color: color.to_string(),
            age,
            planter,
        };
        println!("A {} olive tree of the species {:?} and {} days old has been planted in {} by {} {}", tree.color, tree.species, tree.age, tree.orchard_name, tree.planter.first_name, tree.planter.last_name);
    }
}
