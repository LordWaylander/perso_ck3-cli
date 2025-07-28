use std::fs::File;
use clap::Parser;
mod structs;
use structs::*;

use rand::prelude::*;

const EDUCATION_FILE: &str = "./ressources/educations.json";
const PERSONNALITIES_FILE: &str = "./ressources/personnalities.json";
const LIMIT_POINTS: i32 = 400;

fn load_data() -> (Vec<Education>, Vec<Personality>) {
    let education_file = File::open(EDUCATION_FILE).unwrap();
    let educations : Vec<Education> = serde_json::from_reader(education_file)
    .expect("error while reading or parsing");

    let personnalities_file = File::open(PERSONNALITIES_FILE).unwrap();
    let personnalities : Vec<Personality> = serde_json::from_reader(personnalities_file)
    .expect("error while reading or parsing");

    (educations, personnalities)
}

fn remove_personnality(traits_incompatibles: Vec<String>, personality_bonus: &mut Vec<Personality>, personality_neutral: &mut Vec<Personality>) {
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

fn generate_personnage(datas: (Vec<Education>, Vec<Personality>)) -> Personnage {
    let mut rng = rand::rng();
    let args = Args::parse();
    let mut statistiques = Statistiques::new();
    let educations: Vec<Education> = datas.0;
    let personalities: Vec<Personality> =  datas.1;

    // dbg!(&args);
    /*
        @todo
        25 ans = 67 pts
        + 5 stats à 5 pts = 12 pts
        + 6 prouesse à 5 pts
        = 67 + 65 = 133

        oui c'est à améliorer selon les stats, l'age, etc...
    */
    // @todo : faire le liste des points de l'age
    let age: Age;
    if args.age.is_some() {
        age = Age(args.age.unwrap());
    } else {
        // un nombre ramdom nan ?
        age = Age::default();
    }
    let score_age = age.get_score_age();
    let mut points_personnage: i32 = score_age + 65; 

    /* Education -> ------------------------------------------------------------------------------ */
    let education_personnage: Education;

    let education_is_some = args.education.is_some();
    let mut educs_possible: Vec<Education> = educations;

    if education_is_some {
        match args.education.as_ref().unwrap().as_str() {
            "diplomatie" | "martialite" | "intrigue" | "intendance" | "erudition" => {
                let education_choosen = args.education.unwrap();
                educs_possible = educs_possible.into_iter().filter(|educ| educ.name == education_choosen).collect();
            },
            _ => {
               panic!("Education pas dans la liste")
            }
        }
    }

    let education_level_is_some = args.level.is_some();

    if education_level_is_some {
        match args.level.unwrap() {
            1 | 2 | 3 | 4 | 5 => {
                let education_level_choosen = args.level.unwrap() as u8;
                educs_possible = educs_possible.into_iter().filter(|educ| educ.level == education_level_choosen).collect();
            },
            _ => {
               panic!("level pas dans la liste")
            }
        }
    }

    
    if education_level_is_some || education_is_some {
        let idx = rng.random_range(0..educs_possible.len());
        education_personnage = educs_possible[idx].clone();
    } else {
        // sinon rien est rentré, donc full random mais avec une certaine chance d'obtenir une bonne éducation
        let percentage = rng.random_range(0..100);

        if percentage < 10 {
            let very_good_education: Vec<Education> = educs_possible.into_iter().filter(|educ| educ.level == 5).collect();
            let educ_index= rng.random_range(0..very_good_education.len());
            education_personnage = very_good_education[educ_index].clone();
        } else if percentage < 90 {
            let good_education: Vec<Education> = educs_possible.into_iter().filter(|educ| educ.level >= 3 && educ.level < 5).collect();
            let educ_index= rng.random_range(0..good_education.len());
            education_personnage = good_education[educ_index].clone();
        } else {
            let education: Vec<Education> =educs_possible.into_iter().filter(|educ| educ.level < 3).collect();
            let educ_index= rng.random_range(0..education.len());
            education_personnage = education[educ_index].clone();
        }
    }

    // dbg!(&education_personnage);

    points_personnage += education_personnage.points as i32;
    for bonus in education_personnage.bonus.clone() {
        statistiques.add_bonus_to_stats(bonus);
    }
    // dbg!("pts APRES SELECT EDUCATION = {points_personnage}");

    /* Personnality -> ------------------------------------------------------------------------------ */

    let mut personality_bonus: Vec<Personality> = Vec::new();
    let mut personality_neutral: Vec<Personality> = Vec::new();

    for personnality  in personalities.into_iter() {

        let mut match_bonus_education = false;
        let mut match_no_bonus_education = false;

        for bonus in personnality.bonus.iter() {
            if education_personnage.name == "martialite" && (bonus.name == education_personnage.name || bonus.name == "prouesse") && bonus.apttitudes > 0 {
                // car faut prendre la prouesse aussi un seigneur de guerre qui sait pas se battre il est inutile
                match_bonus_education = true;
            } else if bonus.name == education_personnage.name && bonus.apttitudes > 0 {
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

    // dbg!(&personality_bonus);
    // dbg!("*************************");
    // dbg!(&personality_neutral);

    let mut personnality_personnage: Vec<Personality> = Vec::new();

    // dbg!("*****BEFORE*****");
    // dbg!("personality_bonus : ");
    // dbg!("{:?}", personality_bonus);
    // dbg!("personality_neutral : ");
    // dbg!("{:?}", personality_neutral);

    while personnality_personnage.len() < 3 {
        let percentage= rng.random_range(0..100);
        // 60% de chances d'obtenir une personnalité qui correspond à l'éducation
        if percentage < 60 {
            let pers_index= rng.random_range(0..personality_bonus.len());

            // voir pour avoir moins souvent le trait ambitieux ?
            // parfois y'a deux trait identiques qui sortent comme si le remove foirais MAIS il foire pas

            personnality_personnage.push(personality_bonus[pers_index].clone());
            points_personnage += personality_bonus[pers_index].points as i32;

            // dbg!("CHOIX : {:?}", personality_bonus[pers_index]);
            // dbg!("pers_index : {pers_index}");

            let traits_incompatibles: Vec<String> = personality_bonus[pers_index].incompatible.clone();
            personality_bonus.remove(pers_index);

            remove_personnality(traits_incompatibles, &mut personality_bonus, &mut personality_neutral);
        } else {
            let pers_index= rng.random_range(0..personality_neutral.len());

            personnality_personnage.push(personality_neutral[pers_index].clone());
            points_personnage += personality_neutral[pers_index].points as i32;

            let traits_incompatibles = personality_neutral[pers_index].incompatible.clone();
            personality_neutral.remove(pers_index);

            remove_personnality(traits_incompatibles, &mut personality_bonus, &mut personality_neutral);
        }
    }   

    for personality in personnality_personnage.clone() {
        for bonus in personality.bonus {
            statistiques.add_bonus_to_stats(bonus);
        }
    }

    // dbg!("*****AFTER*****");
    // dbg!("personality_bonus : ");
    // dbg!(&personality_bonus);
    // dbg!("personality_neutral : ");
    // dbg!(&personality_neutral);

    /* Statistiques -> ------------------------------------------------------------------------------ */

    // dbg!("INITALIZATION");
    // dbg!("{:?}", statistiques);

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
        let percentage = rng.random_range(0..100);

        let stat_name = if percentage < 10 {
            /*
                Si martialité, 80% de chances augmenter martialité et 20% prouesse ?
            */

            if education_personnage.name == "martialite" {
                if rng.random_range(0..100) < 80 {
                    &education_personnage.name
                } else {
                    "prouesse"
                }
            } else {
                &education_personnage.name
            }

        } else {
            let index = rng.random_range(0..stats_filter.len());
            stats_filter[index]
        };

        let cout = statistiques.calcule_cout_increment(stat_name);
       
        if points_personnage+cout <= LIMIT_POINTS {
            let num = statistiques.incremente_or_decremente_stats(stat_name, Signe::Increment);
            points_personnage += num
        } else if points_personnage >= 390 && points_personnage < LIMIT_POINTS-1{
            // en gros si il reste entre 10 et 2 pts a attribuer autant essayer de rentabiliser un max
            // mais j'ai pas mieux que ce truc pour l'instant
            let mut bool_break = false;
            
            for stat_name in stats {
                let cout = statistiques.calcule_cout_increment(stat_name);
                if points_personnage+cout <= LIMIT_POINTS {
                    let num = statistiques.incremente_or_decremente_stats(stat_name, Signe::Increment);
                    points_personnage += num;
                    bool_break = false;
                } else {
                    bool_break = true;
                }
            }

            if bool_break {
                break;
            }

        } else {
            break;
        }
    }

    let perso: Personnage = Personnage {
        age : age,
        education: education_personnage,
        personnality: personnality_personnage,
        statistiques,
        points_totaux: points_personnage as u16
    };

    perso

}

use std::io;
fn main() {

    let datas: (Vec<Education>, Vec<Personality>) = load_data();

    let personnage = generate_personnage(datas);

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
