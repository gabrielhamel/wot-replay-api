use crate::input::battle_information::VehicleResults;

#[derive(GraphQLObject)]
pub struct Vehicle {
    pub nation: String,
    pub name: String
}


impl From<&VehicleResults> for Vehicle {
    fn from(value: &VehicleResults) -> Self {
        let vehicle_type = value.vehicle_type.split(':').collect::<Vec<&str>>();
        Vehicle {
            nation: String::from(vehicle_type[0]),
            name: String::from(vehicle_type[1])
        }
    }
}
