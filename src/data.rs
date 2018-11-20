// Defines the data contained in a node
pub struct Data<T> {
    value : T
}

impl<T> Data<T>{
    pub fn new(data_value : T) -> Data<T> {
        let data = Data{value: data_value};
        data
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}