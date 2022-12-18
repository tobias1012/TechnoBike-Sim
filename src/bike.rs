
#[derive(Clone)]
pub struct Bike {
    pub id: String,
    pub watt: u16,
    pub watt_percentage: u8,
    pub rpm: u8,
    pub fps: u8, //FPS is Functional Threshold Power? is a percentage
    
    
}


impl Bike {
    pub fn new(id: &str) -> Bike{
        Bike {
            id: String::from(id),
            watt: 0,
            watt_percentage: 0,
            rpm: 0,
            fps: 0
        }
    }


}