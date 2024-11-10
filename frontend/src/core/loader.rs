use common::AppData;

pub async fn load_apps() -> Vec<AppData> {
    let apps = installed::list().unwrap();
    let mut loaded_apps = vec![];
    for app in apps {
        let name = app.name().into_owned();
        let mut path=String::new();
        if name!=""{
            let dump = app.dump();
            if let Some(start) = dump.find("DisplayIcon: ") {
                // Offset by length of "DisplayIcon: "
                let start = start + "DisplayIcon: ".len();
                // Find the end of the path by looking for the next newline
                if let Some(end) = dump[start..].find('\n') { 
                       path= dump[start..start + end].trim().replace('"', "").replace(",0", "").to_string()
                }
            }
            loaded_apps.push(AppData{
                name,
                path,
            });
        }
        
    }
    loaded_apps
}