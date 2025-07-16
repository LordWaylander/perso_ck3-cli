// use std::collections::HashMap;
use std::fs::File;

use rand::prelude::*;


const EDUCATION_FILE: &str = "educations.json";
const PERSONNALITIES_FILE: &str = "personnalities.json";
const LIMIT_POINTS: i16 = 400;
// const CLI: bool = false;

#[derive(Debug, serde::Deserialize, Clone)]
struct Education {
        name: String,
        level: u8,
        points : u16,
        bonus: Vec<Bonus>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Personality {
        name: String,
        points : i16,
        bonus: Vec<Bonus>,
        incompatible: Vec<String>
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
struct Bonus {
    name: String,
    apttitudes: i8
}

#[derive(Debug)]
struct Personnage {
    education: Education,
    personnality: Vec<Personality>,
    statistiques: Statistiques,
    points_totaux: u16
}

#[derive(PartialEq)]
enum Signe {
    Increment,
    Decrement
}

#[derive(Debug)]
struct Statistiques {
    diplomatie: i8,
    martialite: i8,
    intendance: i8,
    intrigue: i8,
    erudition: i8,
    prouesse: i8
}

impl Statistiques {
    fn new() -> Statistiques {
        Statistiques {
            // valeur de départ de tout personnage créé de base ds le jeu
            diplomatie : 5,
            martialite: 5,
            intendance: 5,
            intrigue: 5,
            erudition: 5,
            prouesse: 5
        }
    }

    fn incremente_or_decremente_stats(&mut self, stat_name: &str, signe: Signe) -> i16 {
        let modifier = if signe == Signe::Decrement {-1} else {1};
        let val = match stat_name {
            "intrigue" => {
                self.intrigue = (self.intrigue + modifier).max(0);
                self.intrigue
            },
            "diplomatie" => {
                self.diplomatie = (self.diplomatie + modifier).max(0);
                self.diplomatie
            },
            "martialite" => {
                self.martialite = (self.martialite + modifier).max(0);
                self.martialite
            },
            "intendance" => {
                self.intendance = (self.intendance + modifier).max(0);
                self.intendance
            },
            "erudition" => {
                self.erudition = (self.erudition + modifier).max(0);
                self.erudition
            },
            "prouesse" => {
                self.prouesse = (self.prouesse + modifier).max(0);
                self.prouesse
            },
            _ => panic!("erreur incremente_statst, bonus_name = {}",stat_name)
        };

        if stat_name == "prouesse" {
            Statistiques::val_prouesse(val).into()
        } else {
            Statistiques::val_stats(val).into()
        }

    }

    fn val_stats(val : i8) -> i8 {
        match val {
            0..=4 => 2,
            5..=8 => 4,
            9..=12 => 7,
            13..=16 => 11,
            17..=100 => 17, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    fn val_prouesse(val : i8) -> i8 {
        match val {
            0..=4 => 1,
            5..=8 => 2,
            9..=12 => 4,
            13..=16 => 7,
            17..=100 => 11, // a vérifier sur l'ensemble des valeurs mais flemme (regardé juqu'a 30)
            _ => 0
       }
    } 

    fn calcule_cout_increment(&self, stat_name: &str) -> i16 {
        let val = match stat_name {
            "intrigue" => {
                self.intrigue
            },
            "diplomatie" => {
                self.diplomatie
            },
            "martialite" => {
                self.martialite
            },
            "intendance" => {
                self.intendance
            },
            "erudition" => {
                self.erudition
            },
            "prouesse" => {
                self.prouesse
            },
            _ => panic!("erreur calcule_cout_increment, bonus_name = {}",stat_name)
        };

        if stat_name == "prouesse" {
            Statistiques::val_prouesse(val+1).into()
        } else {
            Statistiques::val_stats(val+1).into()
        }
    }

    fn add_bonus_to_stats(&mut self, bonus: Bonus) {
        match bonus.name.as_str() {
            "intrigue" => {
                self.intrigue += bonus.apttitudes
            },
            "diplomatie" => {
                self.diplomatie += bonus.apttitudes
            },
            "martialite" => {
                self.martialite += bonus.apttitudes
            },
            "intendance" => {
                self.intendance += bonus.apttitudes
            },
            "erudition" => {
                self.erudition += bonus.apttitudes
            },
            "prouesse" => {
                self.prouesse += bonus.apttitudes
            },
            _ => panic!("erreur personnalité, bonus_name = {}",bonus.name)
        }
    }
}

fn load_data() -> (Vec<Education>, Vec<Personality>) {
    let education_file = File::open(EDUCATION_FILE).unwrap();
    let educations : Vec<Education> = serde_json::from_reader(education_file)
    .expect("error while reading or parsing");

    let personnalities_file = File::open(PERSONNALITIES_FILE).unwrap();
    let personnalities : Vec<Personality> = serde_json::from_reader(personnalities_file)
    .expect("error while reading or parsing");

    (educations, personnalities)
}

fn generate_personnage(datas: (Vec<Education>, Vec<Personality>)) -> Personnage {
    let mut rng = rand::thread_rng();
    /*
        25 ans = 67 pts
        + 5 stats à 5 pts = 12 pts
        + 6 prouesse à 5 pts
        = 67 + 65 = 133

        oui c'est à améliorer selon les stats, etc...
    */
    let mut points_personnage: i16 = 67 + 65; 
    let mut statistiques = Statistiques::new();

    let educations: Vec<Education> = datas.0;
    let personalities: Vec<Personality> =  datas.1;

    /* Education -> ------------------------------------------------------------------------------ */

    let education_personnage: Education;

    // if CLI {

    //     // c'est pas en place mais c'est en place

    //     let educations_cli = [
    //         "intrigue",
    //         "diplomatie",
    //         "martialite",
    //         "intendance",
    //         "erudition"
    //     ];

    //     let educ_wanted = "erudition"; // = value cli


    // nan mais.... faut prendre un filter que je suis con x)
    //     let mut hash_education = HashMap::new();
    //     for e in educations.clone() {
    //         hash_education.insert(e.name.clone(), e);
    //     }

    //     let education =  hash_education.get(educ_wanted).unwrap();

    //     education_personnage = education.clone();


    // } else {
        let percentage = rng.gen_range(0..100);

        if percentage >= 0 && percentage < 10 {
            let very_good_education: Vec<Education> = educations.clone().into_iter().filter(|educ| educ.level == 5).collect();
            let educ_index= rng.gen_range(0..very_good_education.len());
            education_personnage = very_good_education[educ_index].clone();
        } else if percentage >= 10 && percentage < 70 {
            
            let good_education: Vec<Education> = educations.clone().into_iter().filter(|educ| educ.level >= 3 && educ.level < 5).collect();
            let educ_index= rng.gen_range(0..good_education.len());
            education_personnage = good_education[educ_index].clone();
        } else {
            let education: Vec<Education> = educations.clone().into_iter().filter(|educ| educ.level < 3).collect();
            let educ_index= rng.gen_range(0..education.len());
            education_personnage = education[educ_index].clone();
        }
        
    // }

    points_personnage += education_personnage.points as i16;
    // println!("pts APRES SELECT EDUCATION = {points_personnage}");

    /* Personnality -> ------------------------------------------------------------------------------ */

    let mut personality_bonus: Vec<Personality> = Vec::new();
    let mut personality_neutral: Vec<Personality> = Vec::new();

    for personnality  in personalities.into_iter() {

        let mut match_bonus_education = false;
        let mut match_no_bonus_education = false;

        for bonus in personnality.bonus.iter() {
            if education_personnage.name == "martialite" && (bonus.name == education_personnage.name || bonus.name == "prouesse") {
                // car faut prendre la prouesse aussi un seigneur de guerre qui sait pas se battre il est inutile
                match_bonus_education = true;
            } else if bonus.name == education_personnage.name {
                match_bonus_education = true;
            } else {
                match_no_bonus_education = true;
            }
        }

        if match_bonus_education {
            personality_bonus.push(personnality);
        } else if match_no_bonus_education { 
            personality_neutral.push(personnality);
        }
    }

    let mut personnality_personnage: Vec<Personality> = Vec::new();

    // println!("*****BEFORE*****");
    // println!("personality_bonus : ");
    // println!("{:?}", personality_bonus);
    // println!("personality_neutral : ");
    // println!("{:?}", personality_neutral);

    while personnality_personnage.len() < 3 {
        let percentage= rng.gen_range(0..100);
        // 60% de chances d'obtenir une personnalité qui correspond à l'éducation
        if percentage < 60 {
            let pers_index= rng.gen_range(0..personality_bonus.len());

            // voir pour avoir moins souvent le trait ambitieux ?
            // parfois y'a deux trait identiques qui sortent comme si le remove foirais MAIS il foire pas

            personnality_personnage.push(personality_bonus[pers_index].clone());
            points_personnage += personality_bonus[pers_index].points;

            // println!("CHOIX : {:?}", personality_bonus[pers_index]);
            // println!("pers_index : {pers_index}");

            let traits_incompatibles = personality_bonus[pers_index].incompatible.clone();
            personality_bonus.remove(pers_index);

            traits_incompatibles.into_iter().for_each(
                |value| {
                    if let Some(index) = personality_bonus.iter().position(|pers| pers.name == value) {
                        personality_bonus.remove(index);
                    };

                    if let Some(index) = personality_neutral.iter().position(|pers| pers.name == value) {
                        personality_neutral.remove(index);
                    };
                }
            );
        } else {
            let pers_index= rng.gen_range(0..personality_neutral.len());

            personnality_personnage.push(personality_neutral[pers_index].clone());
            points_personnage += personality_neutral[pers_index].points;

            let traits_incompatibles = personality_neutral[pers_index].incompatible.clone();
            personality_neutral.remove(pers_index);

            traits_incompatibles.into_iter().for_each(
                |value| {
                    if let Some(index) = personality_bonus.iter().position(|pers| pers.name == value) {
                        personality_bonus.remove(index);
                    };

                    if let Some(index) = personality_neutral.iter().position(|pers| pers.name == value) {
                        personality_neutral.remove(index);
                    };
                }
            );
        }
    }   

    // println!("*****AFTER*****");
    // println!("personality_bonus : ");
    // println!("{:?}", personality_bonus);
    // println!("personality_neutral : ");
    // println!("{:?}", personality_neutral);

    /* Statistiques -> ------------------------------------------------------------------------------ */

    

    // println!("INITALIZATION");
    // println!("{:?}", statistiques);

    let stats = [
        "intrigue",
        "diplomatie",
        "martialite",
        "intendance",
        "erudition",
        "prouesse"
    ];

    let stats_filter: Vec<&str> = stats.clone().into_iter().filter(|name|*name != education_personnage.name).collect();

    /*
        C'est pas parfait, exemple :
         *** statistiques ***
            diplomatie : 6
            martialite : 7
            intendance : 20
            intrigue : 9
            erudition : 11
            prouesse : 12
            points_totaux : 390
        on pourrait augmenter la diplomatie de +2 pour avoir 398 pts
        mais en dehors de ça, ça fait le taf
    */

    while points_personnage <  LIMIT_POINTS {
        
        //10% de base d'obtenir +1 dans l'éducation choisie
        let percentage = rng.gen_range(0..100);

        let stat_name = if percentage < 10 {
            /*
                Si martialité, 80% de chances augmenter martialité et 20% prouesse ?
            */

            if education_personnage.name == "martialite" {
                if rng.gen_range(0..100) < 80 {
                    &education_personnage.name
                } else {
                    "prouesse"
                }
            } else {
                &education_personnage.name
            }

        } else {
            let index = rng.gen_range(0..stats_filter.len());
            stats_filter[index]
        };

        let cout = statistiques.calcule_cout_increment(stat_name);
       
        if points_personnage+cout <= LIMIT_POINTS {
            let num = statistiques.incremente_or_decremente_stats(stat_name, Signe::Increment);
            points_personnage += num
        } else {
            // todo -> si ça tombe ici parcourir stats_filter dans l'ordre si aucun passe break
            break;
        }
    }

    // println!("INCREMENTE STATS");
    // println!("{:?}", statistiques);

    for personality in personnality_personnage.clone() {
        for bonus in personality.bonus {
            statistiques.add_bonus_to_stats(bonus);
        }
    }

    // println!("BONUS PERSONNALITE");
    // println!("{:?}", statistiques);

    for bonus in education_personnage.bonus.clone() {
        statistiques.add_bonus_to_stats(bonus);
    }

    // println!("BONUS EDUCATION");
    // println!("{:?}", statistiques);

    let perso: Personnage = Personnage {
        education: education_personnage,
        personnality: personnality_personnage,
        statistiques,
        points_totaux: points_personnage as u16
    };

    perso

}

fn main() {

    let datas: (Vec<Education>, Vec<Personality>) = load_data();

    let personnage = generate_personnage(datas);

    println!(" *** education ***");
    println!("education : {}", personnage.education.name);
    println!("level : {}", personnage.education.level);

    println!(" *** personnality ***");
    for personalit in personnage.personnality {
        println!("{}", personalit.name);
    }
    
    println!(" *** statistiques ***");
    println!("diplomatie : {}", personnage.statistiques.diplomatie);
    println!("martialite : {}", personnage.statistiques.martialite);
    println!("intendance : {}", personnage.statistiques.intendance);
    println!("intrigue : {}", personnage.statistiques.intrigue);
    println!("erudition : {}", personnage.statistiques.erudition);
    println!("prouesse : {}", personnage.statistiques.prouesse);

    println!("points_totaux : {}", personnage.points_totaux);

}
