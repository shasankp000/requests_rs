A simple library, written in rust for sending GET/POST requests, included with an async file downloader. Intended for mostly small projects which need to make quick GET/POST requests or download files.

src/main.rs is for the manual tests I did while coding the src/lib.rs

This repository will be used in an even larger upcoming project of mine


# Functions 

# sync_get
>
> Test function which sends a synchronous get request to any api and returns a response object which can be then parsed into json
>
> Recommended to use the requests_rs::requests::api_referencer::get_and_save_json function instead of this.

# get_and_save_json
>
> Sends a get request to an api ,parses the json response and returns the json object

> Example 1

> use requests_rs::requests::api_referencer::get_and_save_json;
>
> get_and_save_json("https://api-url.com", true, false).expect("Some error message!")
>
>Having save=true will parse the json value and save it to a json file. 
>
>Having silent_mode = false will pretty-print out the json response(Useful for debugging purposes?).
>
>This function is asynchrounous so many get requests can be sent at a time.

# print_and_post_json
>
> Sends a POST request to any api and returns the response json object
>
> Example 1
>
> use requests_rs::requests::api_referencer::print_and_post_json;
>
> print_and_post_json("https://api-url.com", "path/to/json_file", true)
>
> If silent_mode is set to true then the function will silently send a POST request and return the response json object
>
> If set to false then the function will send a POST request and pretty print out the response json, alongside returning > it as a value as well

# async_download_file 
>
> Downloads any file asynchronously
>
> use requests_rs::requests::file_downloader::async_download_file;
>
> async_download_file("https://download-the-file.exe", "your_download_path").expect("Some error message")
