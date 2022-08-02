//use std::{ process::exit, vec};
#[macro_use]
extern crate rocket;
use std::string;

//use chrono::Datelike;
use clap::{ArgEnum, Parser};
use select::{document::Document, predicate::Attr};
//use termion::{color, style};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
//use serde_json::Result;

#[derive(Deserialize, Serialize, Debug)]
struct Menu {
    sopa: String,
    vegetariano: String,
    principal: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct EmentaDia {
    almoco: Menu,
    jantar: Menu,
}

const WEEK_DAYS: [&str; 5] = [
    "Segunda-Feira",
    "Terça-Feira",
    "Quarta-Feira",
    "Quinta-Feira",
    "Sexta-Feira",
];

#[macro_export]
macro_rules! bold {
    ($text:expr) => {
        format!("{}{}{}", style::Bold, $text, style::Reset)
    };
}

#[macro_export]
macro_rules! underline {
    ($text:expr) => {
        format!("{}{}{}", style::Underline, $text, style::Reset)
    };
}

#[macro_export]
macro_rules! red {
    ($text:expr) => {
        format!("{}{}{}", color::Fg(color::Red), $text, style::Reset)
    };
}




#[get("/seg")]
async fn seg() -> String {
    serde_json::to_string_pretty(&ementa(WEEK_DAYS[0]).await).unwrap()
}

#[get("/ter")]
async fn ter() -> String {
    serde_json::to_string_pretty(&ementa(WEEK_DAYS[1]).await).unwrap()
}

#[get("/qua")]
async fn qua() -> String {
    serde_json::to_string_pretty(&ementa(WEEK_DAYS[2]).await).unwrap()
}

#[get("/qui")]
async fn qui() -> String {
    serde_json::to_string_pretty(&ementa(WEEK_DAYS[3]).await).unwrap()
}

#[get("/sex")]
async fn sex() -> String {
    serde_json::to_string_pretty(&ementa(WEEK_DAYS[4]).await).unwrap()
}


async fn ementa(day: &str) -> EmentaDia {
    let url = "https://eatdreamsmile.pt/";
    let mut info = Vec::<String>::new();

    let mut current_day = url.to_string();
    current_day.push_str(day.to_lowercase().replace("ç", "c").as_str());

    let response = reqwest::get(current_day).await.unwrap().text();
    let document = Document::from(response.await.unwrap().as_str());

    //println!("{}", day);
    let mut i = 0;
    for node in document.find(Attr("class", "wpb_wrapper")) {
        if i >= 3 {
            for child in node.children() {
                if child.name() == Some("h4") {
                    info.push(child.text().replace("\n", ""));
                }
            }
        }
        i += 1;
    }
    info[0] = info[0].replace("Sopa: ", "");
    info[1] = info[1].replace("Prato Mediterrânico: ", "");
    info[2] = info[2].replace("Prato Vegetariano: ", "");
    info[3] = info[3].replace("Sopa: ", "");
    info[4] = info[4].replace("Prato Mediterrânico: ", "");
    info[5] = info[5].replace("Prato Vegetariano: ", "");

    let almoço = Menu {
        sopa: info[0].to_string(),
        vegetariano: info[1].to_string(),
        principal: info[2].to_string(),
    };
    let jantar = Menu {
        sopa: info[3].to_string(),
        vegetariano: info[4].to_string(),
        principal: info[5].to_string(),
    };

    let ementa_dia = EmentaDia {
        almoco: almoço,
        jantar: jantar,
    };

    return ementa_dia;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum WeekdayArg {
    Seg,
    Ter,
    Qua,
    Qui,
    Sex,
}

/// Command line tool to fetch the menu of the IST Alameda canteen
///
/// When no argument is provided, today's menu is shown
#[derive(Parser, Debug)]
struct Args {
    /// Prints all the menus of the week
    #[clap(short, long)]
    all: bool,

    /// Prints the menu from that day
    #[clap(short, long, arg_enum)]
    day: Option<WeekdayArg>,
}

/* 
    let args = Args::parse();

    
 */

    // Convert the day argument from week day to an integer starting in monday with 0
    /* let day = match args.day {
        Some(WeekdayArg::Seg) => 0,
        Some(WeekdayArg::Ter) => 1,
        Some(WeekdayArg::Qua) => 2,
        Some(WeekdayArg::Qui) => 3,
        Some(WeekdayArg::Sex) => 4,
        None => chrono::offset::Local::today()
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap(),
    }; */

   /*  for i in WEEK_DAYS {
        ementa_semana.insert(i.to_string(), json!(ementa(i)));
    } */

    #[launch]   
    fn rocket() -> _ {
        rocket::build().mount("/", routes![seg,ter,qua,qui,sex])
    }
    //println!("{}", serde_json::to_string_pretty(&ementa_semana).unwrap());
