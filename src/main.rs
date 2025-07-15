use std::collections::HashMap;
use std::fs::File;

use rand::prelude::*;


const EDUCATION_FILE: &str = "educations.json";
const PERSONNALITIES_FILE: &str = "personnalities.json";
const LIMIT_POINTS: i16 = 400;
const CLI: bool = false;

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

    fn incremente_stats(&mut self, stat_name: &str) -> i16 {
       let val = match stat_name {
            "intrigue" => {
                self.intrigue += 1;
                self.intrigue
            },
            "diplomatie" => {
                self.diplomatie += 1;
                self.diplomatie
            },
            "martialite" => {
                self.martialite += 1;
                self.martialite
            },
            "intendance" => {
                self.intendance += 1;
                self.intendance
            },
            "erudition" => {
                self.erudition += 1;
                self.erudition
            },
            "prouesse" => {
                self.prouesse += 1;
                self.prouesse
            },
            _ => panic!("stat inconnue")
        };

        Statistiques::val(val).into()

    }
    fn decremente_stats(&mut self, stat_name: &str) -> i16 {
        let val = match stat_name {
             "intrigue" => {
                if self.intrigue > 0 {
                    self.intrigue -= 1;
                } 
                self.intrigue
             },
             "diplomatie" => {
                if self.diplomatie > 0 {
                    self.diplomatie -= 1;
                } 
                 self.diplomatie
             },
             "martialite" => {
                if self.martialite > 0 {
                    self.martialite -= 1;
                }
                 self.martialite
             },
             "intendance" => {
                if self.intendance > 0 {
                    self.intendance -= 1;
                }
                 self.intendance
             },
             "erudition" => {
                if self.erudition > 0 {
                    self.erudition -= 1;
                }
                 self.erudition
             },
             "prouesse" => {
                self.prouesse += 1;
                self.prouesse
            },
             _ => panic!("stat inconnue")
         };
 
         // if stat_name NOT prouesse ->
         // ELSE AUTRE calcul (meme_value / 2 je crois)
        Statistiques::val(val).into()

     }

    fn val(val : i8) -> i8 {
        match val {
            0..=4 => 2,
            5..=8 => 4,
            9..=12 => 6,
            13..=16 => 8,
            17..=100 => 10,
            _ => 0
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
            _ => panic!("erreur personnalité")
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
    let mut points_personnage: i16 = 67; // 67 car age départ = 25 ans

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


    //     let mut hash_education = HashMap::new();
    //     for e in educations.clone() {
    //         hash_education.insert(e.name.clone(), e);
    //     }

    //     let education =  hash_education.get(educ_wanted).unwrap();

    //     education_personnage = education.clone();


    // } else {
        let percentage = rng.gen_range(0..=100);

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

    /* Personnality -> ------------------------------------------------------------------------------ */

    let mut personality_bonus: Vec<Personality> = Vec::new();
    let mut personality_neutral: Vec<Personality> = Vec::new();

    for personnality  in personalities.into_iter() {

        let mut match_bonus_education = false;
        let mut match_no_bonus_education = false;

        for bonus in personnality.bonus.iter() {
            if bonus.apttitudes > 0 {
                if education_personnage.name == "martialite" && (bonus.name == education_personnage.name || bonus.name == "prouesse") {
                    // car faut prendre la prouesse aussi un seigneur de guerre qui sait pas se battre il est inutile
                    match_bonus_education = true;
                } else if bonus.name == education_personnage.name {
                    match_bonus_education = true;
                } else {
                    match_no_bonus_education = true;
                }
            }
        }

        if match_bonus_education {
            personality_bonus.push(personnality);
        } else if match_no_bonus_education { 
            personality_neutral.push(personnality);
        }
    }

    let mut personnality_personnage: Vec<Personality> = Vec::new();

    while personnality_personnage.len() < 3 {
        if personality_bonus.len() != 0 {
            let pers_index= rng.gen_range(0..personality_bonus.len());

            personnality_personnage.push(personality_bonus[pers_index].clone());
            points_personnage += personality_bonus[pers_index].points;

            personality_bonus[pers_index].incompatible.clone().into_iter().for_each(
                |value| {
                    if let Some(index) = personality_bonus.iter().position(|pers| pers.name == value) {
                        personality_bonus.remove(index);
                    };

                    if let Some(index) = personality_neutral.iter().position(|pers| pers.name == value) {
                        personality_neutral.remove(index);
                    };
                }
            );

            personality_bonus.remove(pers_index);
        } else if personality_neutral.len() != 0 {
            let pers_index= rng.gen_range(0..personality_neutral.len());

            personnality_personnage.push(personality_neutral[pers_index].clone());
            points_personnage += personality_neutral[pers_index].points;

            personality_neutral[pers_index].incompatible.clone().into_iter().for_each(
                |value| {
                    if let Some(index) = personality_bonus.iter().position(|pers| pers.name == value) {
                        personality_bonus.remove(index);
                    };

                    if let Some(index) = personality_neutral.iter().position(|pers| pers.name == value) {
                        personality_neutral.remove(index);
                    };
                }
            );

            personality_neutral.remove(pers_index);
        }
    }   

    /* Statistiques -> ------------------------------------------------------------------------------ */

    let mut statistiques = Statistiques::new();

    println!("INITALIZATION");
    println!("{:?}", statistiques);

    for bonus in education_personnage.bonus.clone() {
        statistiques.add_bonus_to_stats(bonus);
    }

    println!("BONUS EDUCATION");
    println!("{:?}", statistiques);

    for personality in personnality_personnage.clone() {
        for bonus in personality.bonus {
            statistiques.add_bonus_to_stats(bonus);
        }
    }

    println!("BONUS PERSONNALITE");
    println!("{:?}", statistiques);

    let stats = [
        "intrigue",
        "diplomatie",
        "martialite",
        "intendance",
        "erudition",
        "prouesse"
    ];

    let stats_filter: Vec<&str> = stats.clone().into_iter().filter(|name|*name != education_personnage.name).collect();

    while points_personnage <  LIMIT_POINTS {
        
        //60% de base d'obtenir +1 dans l'éducation choisie
        let percentage = rng.gen_range(0..=100);

        if percentage < 60 {
            let num = statistiques.incremente_stats(&education_personnage.name);

            //if (LIMIT_POINTS+num).lt(&400) {
                points_personnage += num
            //}

            /*
                Si martialité, 80% augmenter martialité et 20% prouesse ?
                sinon cf au dessus ?
            */
            

        } else {
            let index = rng.gen_range(0..stats_filter.len());
            let education = stats_filter[index];

            let num = statistiques.incremente_stats(&education);

            //if (LIMIT_POINTS+num).lt(&400) {
                points_personnage += num
            //}
        }
    }

    println!("INCREMENTE STATS");
    println!("{:?}", statistiques);


    // je trouve pas mieux que de reboucler car avec le if (LIMIT_POINTS+num).lt(&400) je tombe sur un overflow
    while points_personnage >  LIMIT_POINTS {
        let index = rng.gen_range(0..educations.len());
        let education = educations[index].name.clone();

        let num = statistiques.decremente_stats(&education);
        points_personnage -= num
    }

    println!("DECREMENTE STATS");
    println!("{:?}", statistiques);

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


    // let mut rng = rand::thread_rng();
    // let an_index = rng.gen_range(0..AN.len());



    // println!("{:?}", datas.0);
    // println!("{:?}", datas.1);





}
