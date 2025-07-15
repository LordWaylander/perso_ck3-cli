use std::collections::HashMap;
use std::fs::File;
use std::vec;

use rand::prelude::*;


const EDUCATION_FILE: &str = "educations.json";
const PERSONNALITIES_FILE: &str = "personnalities.json";
const LIMIT_POINTS: u16 = 400;
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
    erudition: i8

}

impl Statistiques {
    fn new() -> Statistiques {
        Statistiques {
            // valeur de départ de tout personnage créé de base ds le jeu
            diplomatie : 5,
            martialite: 5,
            intendance: 5,
            intrigue: 5,
            erudition: 5
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



    // let mut hash_education = HashMap::new();
    // for e in educations {
    //     hash_education.insert(e.name.clone(), e);
    // }

    // let mut hash_personnality = HashMap::new();
    // for p in personnalities {
    //     hash_personnality.insert(p.name.clone(), p);
    // }

    // (hash_education, hash_personnality)
}

fn generate_personnage(datas: (Vec<Education>, Vec<Personality>)) {
    let mut rng = rand::thread_rng();
    let mut points_personnage: i16 = 67; // 67 car age départ = 25 ans

    let educations: Vec<Education> = datas.0;
    let personalities: Vec<Personality> =  datas.1;

    /* Education -> ------------------------------------------------------------------------------ */

    let education_personnage: Education;

    if CLI {

        // c'est pas en place mais c'est en place

        let educations_cli = [
            "intrigue",
            "diplomatie",
            "martialite",
            "intendance",
            "erudition"
        ];

        let educ_wanted = "erudition"; // = value cli


        let mut hash_education = HashMap::new();
        for e in educations {
            hash_education.insert(e.name.clone(), e);
        }

        let education =  hash_education.get(educ_wanted).unwrap();

        education_personnage = education.clone();


    } else {
        let educ_index= rng.gen_range(0..educations.len());
        education_personnage = educations[4].clone();
    }

    points_personnage += education_personnage.points as i16;

    /* Personnality -> ------------------------------------------------------------------------------ */

    let mut personality_bonus: Vec<Personality> = Vec::new();
    let mut personality_neutral: Vec<Personality> = Vec::new();

    for personnality  in personalities.into_iter() {

        let mut match_bonus_education = false;
        let mut match_no_bonus_education = false;

        for bonus in personnality.bonus.iter() {
            if bonus.apttitudes > 0 {
                if bonus.name == education_personnage.name {
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

    for bonus in &education_personnage.bonus {
        match bonus.name.as_str() {
            "intrigue" => {
                statistiques.intrigue += bonus.apttitudes
            },
            "diplomatie" => {
                statistiques.diplomatie += bonus.apttitudes
            },
            "martialite" => {
                statistiques.martialite += bonus.apttitudes
            },
            "intendance" => {
                statistiques.intendance += bonus.apttitudes
            },
            "erudition" => {
                statistiques.erudition += bonus.apttitudes
            },
            _ => panic!("pas normal")
        }
    }


    // while points_personnage <  LIMIT_POINTS {
        
    // }

    let perso: Personnage = Personnage {
        education: education_personnage,
        personnality: personnality_personnage,
        statistiques,
        points_totaux: points_personnage as u16
    };

    println!("{:?}", perso);

}



// compé base perso = 5
// point 25 ans = 67

fn main() {

    let datas: (Vec<Education>, Vec<Personality>) = load_data();

    generate_personnage(datas);


    // let mut rng = rand::thread_rng();
    // let an_index = rng.gen_range(0..AN.len());



    // println!("{:?}", datas.0);
    // println!("{:?}", datas.1);





}
