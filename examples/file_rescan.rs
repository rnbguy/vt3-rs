use vt3::VtClient;

fn main() {
    let api_key = match std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| "Please provide the api key!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };
    let file = "44d88612fea8a8f36de82e1278abb02f";

    let vt_client = VtClient::new(&api_key);
    match vt_client.file_rescan(file) {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
