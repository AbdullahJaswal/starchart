use colortemp::RGB;
use serde::{Deserialize, Serialize};
use spv_rs::position::position;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CelestialObject {
    source_id: u64,
    ra: f64,
    dec: f64,
    parallax: f64,
    bp_rp: f64,
    bp_g: f64,
    g_rp: f64,
    phot_g_mean_mag: f64,
    phot_bp_mean_mag: f64,
    phot_rp_mean_mag: f64,
    teff_gspphot: f64,
    teff_gspphot_lower: f64,
    teff_gspphot_upper: f64,
    cartesian: [f64; 3],
}

#[allow(dead_code)]
impl CelestialObject {
    pub fn new(
        source_id: u64,
        ra: f64,
        dec: f64,
        parallax: f64,
        bp_rp: f64,
        bp_g: f64,
        g_rp: f64,
        phot_g_mean_mag: f64,
        phot_bp_mean_mag: f64,
        phot_rp_mean_mag: f64,
        teff_gspphot: f64,
        teff_gspphot_lower: f64,
        teff_gspphot_upper: f64,
    ) -> CelestialObject {
        let cartesian: [f64; 3] = position(ra, dec, parallax).into();

        CelestialObject {
            source_id,
            ra,
            dec,
            parallax,
            bp_rp,
            bp_g,
            g_rp,
            phot_g_mean_mag,
            phot_bp_mean_mag,
            phot_rp_mean_mag,
            teff_gspphot,
            teff_gspphot_lower,
            teff_gspphot_upper,
            cartesian,
        }
    }

    pub fn get_catesian(&self) -> [f64; 3] {
        self.cartesian
    }

    pub fn get_cartesian_x(&self) -> f64 {
        self.cartesian[0]
    }

    pub fn get_cartesian_y(&self) -> f64 {
        self.cartesian[1]
    }

    pub fn get_cartesian_z(&self) -> f64 {
        self.cartesian[2]
    }

    pub fn get_temp_in_kelvin(&self) -> f64 {
        self.teff_gspphot
    }

    pub fn get_temp_in_celsius(&self) -> f64 {
        self.get_temp_in_kelvin() - 273.15
    }

    pub fn get_temp_in_fahrenheit(&self) -> f64 {
        self.get_temp_in_celsius() * 9.0 / 5.0 + 32.0
    }

    pub fn get_color_in_rgb(&self) -> RGB {
        colortemp::temp_to_rgb(self.get_temp_in_kelvin().round() as i64)
    }

    pub fn get_color_in_hex(&self) -> String {
        let rgb = self.get_color_in_rgb();

        format!(
            "#{:02X}{:02X}{:02X}",
            rgb.r.clamp(0.0, 255.0) as u8,
            rgb.g.clamp(0.0, 255.0) as u8,
            rgb.b.clamp(0.0, 255.0) as u8,
        )
    }
}
