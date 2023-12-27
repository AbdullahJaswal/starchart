pub fn fetch_non_null_astronomical_data(limit: usize) -> String {
    format!("
    SELECT * FROM astronomical_data
    WHERE source_id IS NOT NULL AND
      ra IS NOT NULL AND
      dec IS NOT NULL AND
      parallax IS NOT NULL AND
      bp_rp IS NOT NULL AND
      bp_g IS NOT NULL AND
      g_rp IS NOT NULL AND
      phot_g_mean_mag IS NOT NULL AND
      phot_bp_mean_mag IS NOT NULL AND
      phot_rp_mean_mag IS NOT NULL AND
      teff_gspphot IS NOT NULL AND
      teff_gspphot_lower IS NOT NULL AND
      teff_gspphot_upper IS NOT NULL
    LIMIT {};", limit)
}