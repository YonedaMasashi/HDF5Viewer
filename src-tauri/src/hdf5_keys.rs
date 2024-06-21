use tauri::command;

use hdf5::File;

use serde::Serialize;

#[command]
pub fn get_hdf5_keys(file_path: String) -> Result<String, String> {
    println!("get_hdf5_keys: file path: {:?}", file_path);
    match read_hdf5_keys(&file_path) {
        Ok(tree) => serde_json::to_string(&tree).map_err(|e| e.to_string()),
        Err(e) => {
            println!("get_hdf5_keys: ERR: {:?}", e.to_string());
            Err(e.to_string())
        },
    }
}


#[derive(Serialize)]
struct Hdf5Node {
    name: String,
    full_key: String,
    children: Vec<Hdf5Node>,
}

fn read_hdf5_keys(file_path: &str) -> Result<Hdf5Node, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let root = file.group("/")?;
    let root_node = Hdf5Node {
        name: "/".to_string(),
        full_key: "/".to_string(),
        children: read_group(&root, &"".to_string())?,
    };
    Ok(root_node)
}

fn read_group(group: &hdf5::Group, parent_key: &String) -> Result<Vec<Hdf5Node>, Box<dyn std::error::Error>> {
    //println!("read_group: parent_key:{}", parent_key);
    let mut nodes = Vec::new();
    for member_name in group.member_names()? {
        let obj = group.group(&member_name);
        let base_key = format!("{}/{}", parent_key.clone(), member_name.clone());
        println!("read_group: base_key:{}", base_key);
        if let Ok(subgroup) = obj {
            nodes.push(Hdf5Node {
                name: member_name.clone(),
                full_key: base_key.clone(),
                children: read_group(&subgroup, &base_key)?,
            });
        } else {
            nodes.push(Hdf5Node {
                name: member_name.clone(),
                full_key: base_key.clone(),
                children: vec![],
            });
        }
    }
    Ok(nodes)
}

