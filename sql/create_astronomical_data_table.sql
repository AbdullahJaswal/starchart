CREATE TABLE astronomical_data
(
    source_id          INTEGER PRIMARY KEY,
    ra                 REAL, -- Right Ascension
    dec                REAL, -- Declination
    parallax           REAL, -- Parallax (indirect measure of distance)
    bp_rp              REAL, -- Color index (Blue Photometer - Red Photometer)
    bp_g               REAL, -- Color index (Blue Photometer - Green)
    g_rp               REAL, -- Color index (Green - Red Photometer)
    phot_g_mean_mag    REAL, -- G-band mean magnitude (luminosity indicator)
    phot_bp_mean_mag   REAL, -- BP-band mean magnitude (luminosity indicator)
    phot_rp_mean_mag   REAL, -- RP-band mean magnitude (luminosity indicator)
    teff_gspphot       REAL, -- Effective temperature from Gaia's spectro-photometric data
    teff_gspphot_lower REAL, -- Lower bound of effective temperature estimate
    teff_gspphot_upper REAL  -- Upper bound of effective temperature estimate
);
