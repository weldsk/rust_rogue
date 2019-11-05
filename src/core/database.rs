use std::collections::HashMap;

pub struct Database<T>
{
    database: HashMap<String, T>
}

impl<T> Database<T>
{
    pub fn new() -> Self
    {
        Self
        {
            database: HashMap::new(),
        }
    }

    /// DBにデータを追加する(id_name: 識別名, data: データ型)
    pub fn insert(&mut self, id_name: &'static str, data: T) -> bool
    {
        // すでに存在していない場合，追加
        if self.database.get(id_name).is_none()
        {
            self.database.insert(id_name.to_string(), data);
            return true;
        }
        return false;
    }
    
    pub fn get(&self, id_name: &'static str) -> Option<&T> {
        self.database.get(id_name)
    }
}