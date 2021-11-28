use std::fs;

pub const LAST_LEVEL: u8 = 4;

// load_level: used to load a level (UNUSED FOR NOW)
pub(crate) fn parse_level(filename: &str) -> Vec<Vec<String>> {
    //this function returns a list of the different objects in our scene
    //separated into the different parameters for each object
    let mut results: Vec<Vec<String>> = vec!();
    let os_specific_linebreak = if std::env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    };
    for a in fs::read_to_string("src/levels/".to_owned()+filename)
        .unwrap()
        .split(os_specific_linebreak)
        .collect::<Vec<&str>>() {
        let result = a.split("-").collect::<Vec<&str>>();
        let mut newresult: Vec<String> = vec!();
        for r in result {
            newresult.push(r.to_string());
        }
        results.push(newresult);
    }
    results
}
