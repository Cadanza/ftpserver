#[path = "."]
#[allow(dead_code)]

/// # Module to manipulate path and file system
///  
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod file_system_handler {

    use std::fs;


    pub fn get_absolute_path(root : String, actual_path : String) -> String {
        let mut cut_path : Vec<&str> = actual_path.split("/").collect();
        let mut cut_root : Vec<&str> = root.split("/").collect();

        if cut_path.last() == Some(&"") {
            cut_path.pop();
        }

        if cut_root.last() == Some(&"") {
        cut_root.pop();
        }

        for d in cut_path {
            cut_root.push(d);
        }

        return cut_root.join("/")
    }

    pub fn change_path(actual_path : String, path : String) -> Option<String> {
        let mut cut_path : Vec<&str> = path.split("/").collect();
        let mut cut_act_path : Vec<&str> = actual_path.split("/").collect();

        if cut_path.last() == Some(&"") {
            cut_path.pop();
        }

        if cut_act_path.last() == Some(&"") {
            cut_act_path.pop();
        }


        if cut_path[0] == "." || cut_path[0] == "" {
            cut_path.remove(0);
        } else {
            let clone = cut_path.clone();
            for d in clone{
                if d == ".." {
                    cut_act_path.pop();
                    cut_path.retain(|&x |x!=d);
                }
            }
        }

        if cut_act_path.len() == 0 {
            println!("Non!");
            return None;
        }

        for d in cut_path {
            cut_act_path.push(d);
        }

        let ret : String = cut_act_path.join("/");
        
        return Some(ret);
    }

    /// Transform a relative path to an absolute path
    /// 
    /// # Arguments
    /// 
    /// - **dir** *String* : relative path to directory
    /// 
    /// # Returns
    /// 
    /// - *Option<String>*:
    ///     - *String* : absolute path of directory
    ///     - *None* : absolute path was above the root
    /// 
    pub fn relative_to_absolute_path(root : String, actual_path : String, path : String) -> Option<String> {
        let chang_path : Option<String> = change_path(actual_path, path);

        match chang_path {
            Some(p) => {
                return Some(get_absolute_path(root, p));
            },
            None => {
                return None;
            }
        }

    }

    /// Say if a decomposed path is under the server root directory
    /// 
    /// # Arguments
    /// 
    /// - **root** *String* : server root directory
    /// - **cut_path** *&mut Vec<&str>* : decomposed path
    /// 
    /// # Returns
    /// 
    /// - *bool* : 
    ///     - *true* : if decomposed path is under the server root
    /// 
    fn cut_path_is_under_root(root : String, cut_path : &mut Vec<&str>) -> bool {
        let mut cut_root : Vec<&str> = root.split("/").collect();
        cut_root.pop();

        if cut_path.last() == Some(&"") {
            cut_path.pop();
        }

        return cut_path.len() < cut_root.len();
    }

    /// Say if path is folder or not
    /// 
    /// # Arguments
    /// 
    /// - **path** *String* 
    /// 
    /// # Returns
    /// 
    /// - *bool* :
    ///     - *true* : is path is folder
    /// 
    pub fn is_folder(path : String) -> bool {
        return fs::metadata(path).unwrap().is_dir();
    }

    /// Say if path exist or not
    /// 
    /// # Arguments
    /// 
    /// - **dir** *String* : directory to test
    /// 
    /// # Returns
    /// 
    /// - *bool* : true if directory exist
    /// 
    pub fn directory_exist(dir : String) -> bool {
        return fs::metadata(dir).is_ok();
    }

    /// Create a folder
    /// 
    /// # Arguments
    /// 
    /// - **path** *String* : path where folder will be create
    /// 
    /// # Returns
    /// 
    /// - *bool* : if folder was correctly create or not
    /// 
    pub fn mkdir(path : String) -> bool {
        let r = fs::create_dir(path);

        return r.is_ok();
    }

    /// Remove a folder
    /// 
    /// # Arguments
    /// 
    /// - **path** *String* : path of folder to remove
    /// 
    /// # Return
    /// 
    /// - *bool* : 
    ///     - *true* : folder was correctly removed
    ///     - *false* : path is not a folder or folder was not removed
    /// 
    pub fn remove_folder(path : String) -> bool {
        if is_folder(format!("{}", path)){
            let r = fs::remove_dir(format!("{}", path));
            return r.is_ok()
        } else {
            return false;
        }
    }

    /// Transform name of folder to relativ path 
    /// 
    /// # Arguments
    /// 
    /// - **last_name** *String* : last name of directory. Is the absolute path of directory
    /// - **new_name** *String* : new name of directory. It's just the name and not a path
    /// 
    /// # Returns
    /// 
    /// - *String* absolute path with last_name was replace by new_name
    /// 
    pub fn name_to_absolute_path(last_name : String, new_name : String) -> String {
        let mut cut_ln : Vec<&str> = last_name.split("/").collect();
            let cut_name : &str = &*new_name;

            cut_ln.pop();
            cut_ln.push(cut_name);

            let ret : String = cut_ln.join("/");

            return format!("{}", ret);
    }

    /// Rename a directory
    /// 
    /// # Arguements
    /// 
    /// - **last_name** *String* : absolute path of actual name of directory
    /// - **new_name** *String* : absolute path of new name of directory
    /// 
    /// # Returns
    /// 
    /// - *bool* : directory was correctly created or not
    /// 
    pub fn rename_dir(last_name : String, new_name : String) -> bool {
        return fs::rename(last_name, new_name).is_ok();
    }
}