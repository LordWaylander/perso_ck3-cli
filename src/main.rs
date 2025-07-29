use clap::Parser;
use core::load_data;
use core::generate_personnage;

use core::structs::*;

/// Simple program to generate a ck3 player
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Possible values : [martialite, diplomatie, intrigue, intendance, erudition]
    #[arg(short, long)]
    pub education: Option<String>,
    /// Possible values : [1, 2, 3, 4, 5]
    #[arg(short, long)]
    pub level: Option<i8>,
    /// @TODO !
    #[arg(short, long)]
    pub age: Option<i8>
}

fn main() {
    let args = Args::parse();
    let datas: (Vec<Education>, Vec<Personality>) = load_data();

    let params = Parameters {
        education: args.education,
        level: args.level,
        // @TODO !
        age: args.age
    };



    let personnage = generate_personnage(datas, params);

    println!(" *** age ***");
    println!("age : {}", personnage.age);

    println!(" *** education ***");
    println!("education : {}", personnage.education.name);
    println!("level : {}", personnage.education.level);

    println!(" *** personnality ***");
    for personalit in personnage.personnality {
        println!("{}", personalit.name);
    }
    
    println!(" *** statistiques ***");
    println!("diplomatie : {}", personnage.statistiques.diplomatie.base + personnage.statistiques.diplomatie.bonus);
    println!("martialite : {}", personnage.statistiques.martialite.base + personnage.statistiques.martialite.bonus);
    println!("intendance : {}", personnage.statistiques.intendance.base + personnage.statistiques.intendance.bonus);
    println!("intrigue : {}", personnage.statistiques.intrigue.base + personnage.statistiques.intrigue.bonus);
    println!("erudition : {}", personnage.statistiques.erudition.base + personnage.statistiques.erudition.bonus);
    println!("prouesse : {}", personnage.statistiques.prouesse.base + personnage.statistiques.prouesse.bonus);

    println!("points_totaux : {}", personnage.points_totaux);

}
