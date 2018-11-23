// Defines the data contained in a node

pub enum DataType{
    String(String),
    Float(f64),
    Integer(u32)
}

// A data is composed of a single value
pub struct Data{
    name : String,
    value : DataType
}

impl Data{
    pub fn new(data_name : String, data_value : DataType) -> Data {
        Data{name: data_name, value: data_value}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &DataType {
        &self.value
    }

    pub fn set_value(&mut self, data_value : DataType) {
        self.value = data_value;
    }
}