enum CitySize {
    TOWN,
    CITY,
    METROPOLIS,
    AREA,
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> Self {
        let (description, residents) = match city_size {
            CitySize::TOWN => {
                let residents: u64 = 1_000;
                (format!("Town with {} residents", residents), residents)
            }
            CitySize::CITY => {
                let residents: u64 = 1_0000;
                (format!("Town with {} residents", residents), residents)
            }
            CitySize::METROPOLIS => {
                let residents: u64 = 1_000_000;
                (format!("Town with {} residents", residents), residents)
            }
            CitySize::AREA => {
                let residents: u64 = 1_00_00_000;
                (format!("Town with {} residents", residents), residents)
            }
        };
        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let jammu = City::new(CitySize::METROPOLIS, true);
    println!("This is {}", jammu.description);
}
