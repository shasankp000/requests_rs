// use std::fs::{self, File};

use requests_rs::requests;
use colored::*;

// extern crate serde_json;
// extern crate serde;
// extern crate jsonxf;


// #[macro_use]

// extern crate serde_derive;


// use serde_json::Value as JsonValue;
// use serde_json::json;

// #[derive(Debug,Serialize,Deserialize)]
// struct Foo {
//     a: i32,
//     b: String
// }

// #[derive(Debug,Serialize,Deserialize)]
// struct Todo1 {
//     #[serde(rename="userId")] // converting camel case to snake case
//     user_id: i32,
//     id: Option<i32>,
//     title: String,
//     completed: bool,
// }




// impl Foo {
//     fn generate_json(&self) -> JsonValue {

//         let json = json!(
//             {
//                 "a": self.a,
//                 "b": self.b

//             }

//         );

//         return json;
//     }
// }

// #[allow(dead_code)]
// impl Todo1 {
//     fn generate_json(&self) -> JsonValue {

//         let json = json!(
//             {
//                 "userId": self.user_id,
//                 "id": self.id,
//                 "title": self.title,
//                 "completed": self.completed
//             }
//         );

//         json

//     }
// }



fn main() {

    // MANUAL TESTS

    //let a = requests::requests::api_referencer::sync_get("https://launchermeta.mojang.com/mc/game/version_manifest.json");

    //println!("{:#?}", a);


    //requests::api_referencer::get_and_save_json("https://launchermeta.mojang.com/mc/game/version_manifest.json", true, false).expect("Error fetching data!");

    let return_xml_data = requests::api_referencer::get_and_save_xml("https://maven.fabricmc.net/net/fabricmc/fabric-installer/maven-metadata.xml").expect("Error accessing xml url!");

    println!("{}", return_xml_data.as_str().green());

    //requests::requests::file_downloader::asymc_download_file("https://www.pixelstalk.net/wp-content/uploads/2016/12/Cloudy-Sky-Background-Widecsreen.jpg", "D:\\rust_projects\\requests").expect("Error downloading file!");


    //requests::requests::file_downloader::async_download_file("https://cdn.modrinth.com/data/AANobbMI/versions/rAfhHfow/sodium-fabric-mc1.19.2-0.4.4%2Bbuild.18.jar", "D:\\rust_projects\\requests").expect("Error downloading file!");

    //JSON 

    // let foo = Foo { a: 42, b: "bar".to_owned() };

    // let value = foo.generate_json();

    // let mut json_data = serde_json::to_string_pretty(&value).unwrap();

    // println!("{:#?}", json_data);

    // File::create("./foo.json").expect("Error creating file!");
    // fs::write("./foo.json", &mut json_data).expect("Error writing to file!");


    // let title_1 = String::from("Hello world!");

    // let todo = Todo1 {
    //     user_id: 1,
    //     id: None,
    //     title: title_1,
    //     completed: true 

    // };

    // let a = todo.generate_json();

    // println!("{:#?}", a.to_string());

    // Things work a bit differently if jsonValue is built from a struct.

    // if .as_str() is directly used, None value is returned from the enum :/

    // String => &str => pretty_print()

    // let ugly_json_string = a.to_string();

    // let ugly_json_str = ugly_json_string.as_str();

    // let pretty_json_str = jsonxf::pretty_print(ugly_json_str).unwrap();


    // File::create("./todo.json").expect("Error creating file!");
    // fs::write("./todo.json", pretty_json_str).expect("Error writing to file!");

    // requests::api_referencer::print_and_post_json("https://jsonplaceholder.typicode.com/todos", "D:\\rust_projects\\requests\\todo.json", true).expect("Error posting data!");




}