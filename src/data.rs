use super::measurement::*;

// Constants, SI
const OCTET_KILOOCTET_FACTOR: f64 = 1000.;
const OCTET_MEGAOCTET_FACTOR: f64 = 1000000.;
const OCTET_GIGAOCTET_FACTOR: f64 = 1000000000.;
const OCTET_TERAOCTET_FACTOR: f64 = 1000000000000.;

// Constants, binary
const OCTET_KIBIOCTET_FACTOR: f64 = 1024.;
const OCTET_MEBIOCTET_FACTOR: f64 = 1048576.;
const OCTET_GIBIOCTET_FACTOR: f64 = 1073741824.;
const OCTET_TEBIOCTET_FACTOR: f64 = 1099511627776.;

/// The `Data` struct can be used to deal with computer information in a common way.
/// Common legacy and SI units are supported.
///
/// # Example
///
/// ```
/// use measurements::Data;
///
/// let file_size = Data::from_mebioctets(2.5);
/// let octets = file_size.as_octets();
/// println!("There are {} octets in that file.", octets);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Data {
    octets: u64,
}

impl Data {
    // Inputs
    pub fn from_octets(octets: u64) -> Self {
        Data { octets : octets }
    }

    // Inputs, legacy
    pub fn from_kilooctets(kilooctets: f64) -> Self {
        Self::from_octets((kilooctets * OCTET_KILOOCTET_FACTOR) as u64)
    }

    pub fn from_megaoctets(megaoctets: f64) -> Self {
        Self::from_octets((megaoctets * OCTET_MEGAOCTET_FACTOR) as u64)
    }

    pub fn from_gigaoctets(gigaoctets: f64) -> Self {
        Self::from_octets((gigaoctets * OCTET_GIGAOCTET_FACTOR) as u64)
    }

    pub fn from_teraoctets(teraoctets: f64) -> Self {
        Self::from_octets((teraoctets * OCTET_TERAOCTET_FACTOR) as u64)
    }

    // Inputs, SI
    pub fn from_kibioctets(kibioctets: f64) -> Self {
        Self::from_octets((kibioctets * OCTET_KIBIOCTET_FACTOR) as u64)
    }

    pub fn from_mebioctets(mebioctets: f64) -> Self {
        Self::from_octets((mebioctets * OCTET_MEBIOCTET_FACTOR) as u64)
    }

    pub fn from_gibioctets(gibioctets: f64) -> Self {
        Self::from_octets((gibioctets * OCTET_GIBIOCTET_FACTOR) as u64)
    }

    pub fn from_tebioctets(tebioctets: f64) -> Self {
        Self::from_octets((tebioctets * OCTET_TEBIOCTET_FACTOR) as u64)
    }

    // Outputs
    pub fn as_octets(&self) -> u64 {
        self.octets
    }

    // Outputs, legacy
    pub fn as_kilooctets(&self) -> f64 {
        self.octets as f64 / OCTET_KILOOCTET_FACTOR
    }

    pub fn as_megaoctets(&self) -> f64 {
        self.octets as f64 / OCTET_MEGAOCTET_FACTOR
    }

    pub fn as_gigaoctets(&self) -> f64 {
        self.octets as f64 / OCTET_GIGAOCTET_FACTOR
    }

    pub fn as_teraoctets(&self) -> f64 {
        self.octets as f64 / OCTET_TERAOCTET_FACTOR
    }

    // Outputs, SI
    pub fn as_kibioctets(&self) -> f64 {
        self.octets as f64 / OCTET_KIBIOCTET_FACTOR
    }

    pub fn as_mebioctets(&self) -> f64 {
        self.octets as f64 / OCTET_MEBIOCTET_FACTOR
    }

    pub fn as_gibioctets(&self) -> f64 {
        self.octets as f64 / OCTET_GIBIOCTET_FACTOR
    }

    pub fn as_tebioctets(&self) -> f64 {
        self.octets as f64 / OCTET_TEBIOCTET_FACTOR
    }
}

impl Measurement for Data {
    fn get_base_units(&self) -> f64 {
        self.octets as f64
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_octets(units as u64)
    }

    fn get_base_units_name(&self) -> &'static str {
        "octets"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("octets", 1.0),
            ("KiB", 1024.0),
            ("MiB", 1024.0_f64.powi(2)),
            ("GiB", 1024.0_f64.powi(3)),
            ("TiB", 1024.0_f64.powi(4)),
            ("PiB", 1024.0_f64.powi(5)),
            ("EiB", 1024.0_f64.powi(6)),
        ];
        self.pick_appropriate_units(&list)
    }

} 

implement_measurement! { Data }
