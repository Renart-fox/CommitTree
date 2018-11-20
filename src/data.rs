// Defines the data contained in a node

pub enum DataType{
    String(String),
    Float(f64),
    Integer(u32)
}

// A data is composed of a single value
pub struct Data<T> {
    name : String,
    value : T
}

impl<T> Data<T>{
    pub fn new(data_name : String, data_value : T) -> Data<T> {
        Data{name: data_name, value: data_value}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, data_value : T) {
        self.value = data_value;
    }
}