

extern crate serde_json;
extern crate jsonxf;


pub mod requests {

    pub mod api_referencer {

        use colored::*;
        use std::path::Path;
        use std::fs::{self, File};

        use serde_json::Value as JsonValue;


        /// # Test function which sends a synchronous get request to any api and returns a response object which can be then parsed into json
        /// 
        /// Recommended to use the ```requests_rs::requests::api_referencer::get_and_save_json``` function instead of this.
        /// 
        /// ```
        /// use requests::requests::api_referencer::sync_get;
        /// 
        /// sync_get("https://api-url.com");
        /// 
        /// ```
        /// 
        pub fn sync_get(url: &str) -> reqwest::blocking::Response {
            reqwest::blocking::get(url).expect("Error referencing data")

        }


        /// ### Sends a get request to an api ,parses the json response and returns the json object
        /// 
        /// ```
        /// use requests_rs::requests::api_referencer::get_and_save_json;
        /// 
        /// Example 1
        /// 
        /// get_and_save_json("https://api-url.com", true, false).expect("Some error message!")
        /// 
        /// ```
        ///
        /// Having save=true will parse the json value and save it to a json file. 
        /// 
        /// Having silent_mode = false will pretty-print out the json response(Useful for debugging purposes?).
        /// 
        /// This function is asynchrounous so many get requests can be sent at a time(although idk why someone would need to do that.)
        /// 
        #[tokio::main]
        pub async fn get_and_save_json(url: &str, save: bool, silent_mode: bool) -> Result<JsonValue, reqwest::Error>{

            #[cfg(windows)] {
                control::set_virtual_terminal(true).unwrap();
            }

            
            let client = reqwest::Client::new()
                .get(url)
                .send()
                .await?;
                

            let status_code = client.status();

            let return_json: serde_json::value::Value = client.json().await?;

            let raw_json = return_json.to_string();

            let ugly_json = raw_json.as_str();

            let mut pretty_json = jsonxf::pretty_print(ugly_json).unwrap();

            if !silent_mode {

                // execute as intended

                if !save && status_code.is_success() {

                    print!("{}", "OK".green());
    
                    println!("[{}]", status_code.as_str().green() );
    
                    println!();
    
                    println!("{}", pretty_json);
                }
                else if save && status_code.is_success() { // if save just simplifies to if save == true
                    
                    // file_downloader::async_download_file(url, "./").expect("Failed to download file!");
    
                    print!("{}", "OK".green());
    
                    println!("[{}]", status_code.as_str().green() );
    
                    println!();
    
                    let file_name = Path::new(url).file_stem().unwrap().to_str().unwrap(); // file name
    
                    let file_extension = Path::new(url).extension().unwrap().to_str().unwrap(); // file extension
    
                    let download_path = "./";
    
                    File::create(format!("{}/{}.{}", download_path, file_name, file_extension)).expect("Error downloading file!");
                    fs::write(format!("{}/{}.{}", download_path, file_name, file_extension), &mut pretty_json).expect("Failed to write to file!");
    
                    print!("{}", "Saved => ".green());
    
                    print!("{}.{}", file_name, file_extension);
    
                    print!("{}", " at ".green());
    
                    println!("{}", download_path);
    
    
                    println!("{}", pretty_json.green());
    
                }
    
                else if status_code.is_client_error() || status_code.is_server_error() {
                    println!("{}", "Error fetching data!".red());
    
                    print!("{}", "Error code ".red());
    
                    print!("{}", status_code.as_str().red())
                }
    
                else {
                    print!("{}", "Error code ".red());
    
                    print!("{}", status_code.as_str().red());
                }
            }

            else if silent_mode {

                // print only status code and saves data.

                if save && status_code.is_success() {

                    print!("{}", "OK".green());
    
                    println!("[{}]", status_code.as_str().green() );
    
                    println!();

                    let file_name = Path::new(url).file_stem().unwrap().to_str().unwrap(); // file name
        
                    let file_extension = Path::new(url).extension().unwrap().to_str().unwrap(); // file extension
        
                    let download_path = "./";
        
                    File::create(format!("{}/{}.{}", download_path, file_name, file_extension)).expect("Error downloading file!");
                    fs::write(format!("{}/{}.{}", download_path, file_name, file_extension), &mut pretty_json).expect("Failed to write to file!");
                    
            
                }

                else if !save && status_code.is_success() {

                    // prints only the status code.

                    print!("{}", "OK".green());
    
                    println!("[{}]", status_code.as_str().green() );
    
                    println!();
                }

                else if status_code.is_client_error() || status_code.is_server_error() {
                    
                    println!("{}", "Error fetching data!".red());

                    print!("{}", "Error code ".red());
    
                    print!("{}", status_code.as_str().red())
                }
                
                else {
                    print!("{}", "Error code ".red());
    
                    print!("{}", status_code.as_str().red());
                }

            }

            Ok(return_json)
            
        }

        ///Returns XML data from an xml file as a string.
        /// Usage :
        /// ```
        /// use requests_rs::requests::api_referencer::get_and_save_xml;
        /// 
        /// Example 1
        /// 
        /// let xml_data = get_and_save_xml("https://xml-url.com").expect("Some error message!");
        /// 
        /// println!("{}", xml_data);
        /// 
        /// ```
        #[tokio::main]
        pub async fn get_and_save_xml(url: &str) -> Result<String, reqwest::Error> {

            #[cfg(windows)] {
                control::set_virtual_terminal(true).unwrap();
            }

            let client = reqwest::Client::new()
                .get(url)
                .send()
                .await?;

            let mut xml_data = "".to_string();

            if client.status().is_success() {
                let return_bytes = client.bytes().await?; // get the data in bytes

                xml_data = String::from_utf8(return_bytes.to_vec()).expect("Error parsing xml data from bytes!");

            }

            else {
                
                let status_code = client.status();

                println!("{}", status_code.as_str().red());


                println!("Url not reachable!");
            }
            
            Ok(xml_data)

        }
        

        
        // had a problem with async POST idk why

        // but this works anyways.

        // might add async post function in the future.


        /// # Sends a POST request to any api and returns the response json object
        /// 
        /// ```
        /// use requests_rs::requests::api_referencer::print_and_post_json;
        /// 
        /// Example 1
        /// 
        /// print_and_post_json("https://api-url.com", "path/to/json_file", true)
        /// 
        /// ```
        /// 
        /// If silent_mode is set to true then the function will silently send a POST request and return the response json object
        /// 
        /// If set to false then the function will send a POST request and pretty print out the response json, alongside returning it as a value as well
        ///
        pub fn print_and_post_json(url: &str, file_path:&str, silent_mode: bool) -> Result<JsonValue, reqwest::Error>{
            
            #[cfg(windows)] {
                control::set_virtual_terminal(true).unwrap();
            }
            let content = fs::read_to_string(file_path).expect("Error reading from file!");

            let json: serde_json::value::Value = serde_json::from_str(&content).expect("Error reading from content");

            let json_map = json.as_object().unwrap();

            let client = reqwest::blocking::Client::new()
                    .post(url)
                    .json(&json_map)
                    .send();
            
            let response = client.unwrap();

            let status = response.status();

            let response_json: serde_json::value::Value = response.json().unwrap();

            let response_text = response_json.to_string();

            
            if !silent_mode {

                // execute as intended 

                print!("{}", "Information to be posted: ".green());

                println!();

                println!("{}", content.green());

                println!();
                
                if status.is_success() {

                    print!("{}", "Response from api: ".green());

                    let ugly_response_str = response_text.as_str();

                    let pretty_json_str = jsonxf::pretty_print(ugly_response_str).unwrap();

                    println!("{}", pretty_json_str.green());
                }
                
                else  {
                    
                    print!("{}", "Error code ".red());
    
                    print!("{}", status.to_string().red());


                }
                
            }

            else if silent_mode {
                if status.is_success() {
                    print!("{}", "OK".green());
    
                    println!("[{}]", status.as_str().green() );
                }

                else {
                    print!("{}", "Error code ".red());
    
                    print!("{}", status.to_string().red());
                }
            }

            
               
        Ok(response_json)

        }
        

    }

    pub mod file_downloader {

        use std::path::Path;
        use std::fs::{self, File};

        use colored::*;


        /// ### Downloads any file asynchronously
        /// 
        /// ```
        /// use requests_rs::requests::file_downloader::async_download_file;
        /// 
        /// 
        /// async_download_file("https://download-the-file.exe", "your_download_path").expect("Some error message")
        /// ```
        #[tokio::main]
        pub async fn async_download_file(url: &str, download_path: &str) -> Result<(), reqwest::Error>{


            #[cfg(windows)] {
                control::set_virtual_terminal(true).unwrap();
            }

            
            let file_name = Path::new(url).file_stem().unwrap().to_str().unwrap(); // file name

            if Path::new(url).extension().is_some() {
                let file_extension = Path::new(url).extension().unwrap().to_str().unwrap(); // file extension

            

                print!("{}", "Downloading ".green());

                println!("{}.{}", file_name,file_extension);


                let client = reqwest::Client::new()
                    .get(url)
                    .send() // sends a GET request
                    .await?
                    .bytes() // receives it in bytes, for any file
                    .await?;



                File::create(format!("{}/{}.{}", download_path, file_name, file_extension)).expect("Error downloading file!");
                fs::write(format!("{}/{}.{}", download_path, file_name, file_extension), client).expect("Failed to write to file!");

                print!("{}", "Saved => ".green());

                print!("{}.{}", file_name, file_extension);

                print!("{}", " at ".green());

                println!("{}", download_path);

            }

            else if Path::new(url).extension().is_none() {

    
                print!("{}", "Downloading ".green());

                println!("{}", file_name);


                let client = reqwest::Client::new()
                    .get(url)
                    .send() // sends a GET request
                    .await?
                    .bytes() // receives it in bytes, for any file
                    .await?;




                File::create(format!("{}/{}", download_path, file_name)).expect("Error downloading file!");
                fs::write(format!("{}/{}", download_path, file_name), client).expect("Failed to write to file!");

                print!("{}", "Saved => ".green());

                print!("{}", file_name);

                print!("{}", " at ".green());

                println!("{}", download_path);

            }

            


            Ok(())
        }


    }

}