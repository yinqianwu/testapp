
pub enum SectionType {
    Unknown = 0,

    /* 通信行业 */
    Telecoms = 0x00010000,

    /* 电子行业 */
    Electronics = 0x00020000,

    /* 食品饮料行业 */
    FoodBeverage = 0x00030000,

    // 酒
    Liquor = 0x00030100,

    /* 互联网行业 */
    Internet = 0x00040000,

    /* 机械行业 */
    Mechanics = 0x00050000,
    // 汽车
    Automotive = 0x00050100,
    // 新能源汽车
    NewEnergyVehicles = 0x00050101,


}

impl SectionType {
    pub fn from_u32(value: u32) -> SectionType {
        match value {
            0x00010000 => SectionType::Telecoms,
            0x00020000 => SectionType::Electronics,
            0x00030000 => SectionType::FoodBeverage,
            0x00030100 => SectionType::Liquor,
            0x00050100 => SectionType::Automotive,
            0x00050101 => SectionType::NewEnergyVehicles,
            _ => SectionType::Unknown,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            SectionType::Telecoms => String::from("通信"),
            SectionType::Electronics => String::from("电子"),
            SectionType::FoodBeverage => String::from("食品饮料"),
            SectionType::Liquor => String::from("酒"),
            SectionType::Automotive => String::from("汽车"),
            SectionType::NewEnergyVehicles => String::from("新能源汽车"),
            _ => String::from("未知标签"),
        }
    }

    pub fn vi32_to_vstring(section_values: Vec<i32>) -> Vec<String> {

        let mut v: Vec<String> = Vec::new();
        for section_value in section_values {
            let section_type = SectionType::from_u32(section_value as u32);
            match section_type {
                SectionType::Unknown => (),
                _ => v.push(section_type.to_string()),
            }
        }
        v
    }
}


