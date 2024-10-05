#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
mod feature_serde;

#[cfg(feature = "sqlx-postgres")]
mod feature_sqlx_postgres;

macro_rules! define_country_codes {
    ($(($code:ident, $name:literal)),* $(,)?) => {
        /// Two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
        pub enum CountryCode {
            $(
                #[doc = $name]
                $code,
            )*
        }

        impl CountryCode {
            /// All possible country codes
            pub const ALL: [CountryCode; 249] = [$(CountryCode::$code, )*];

            /// Returns the country name in English
            #[must_use]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(
                        CountryCode::$code => $name,
                    )*
                }
            }
        }

        impl std::str::FromStr for CountryCode {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = if s.chars().any(|c| c.is_lowercase()) {
                    &s.to_uppercase()
                } else {
                    s
                };
                match s {
                    $(
                        stringify!($code) => Ok(CountryCode::$code),
                    )*
                    _ => Err(format!("Not a country code: {s}")),
                }
            }
        }

        impl From<CountryCode> for &'static str {
            fn from(value: CountryCode) -> Self {
                match value {
                    $(
                        CountryCode::$code => stringify!($code),
                    )*
                }
            }
        }
    };
}

impl CountryCode {
    /// An iterator over all possible country codes
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}

impl AsRef<str> for CountryCode {
    fn as_ref(&self) -> &str {
        <&str>::from(*self)
    }
}

impl core::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}", self.as_ref())
    }
}

define_country_codes! {
    (AD, "Andorra"),
    (AE, "United Arab Emirates"),
    (AF, "Afghanistan"),
    (AG, "Antigua and Barbuda"),
    (AI, "Anguilla"),
    (AL, "Albania"),
    (AM, "Armenia"),
    (AO, "Angola"),
    (AQ, "Antarctica"),
    (AR, "Argentina"),
    (AS, "American Samoa"),
    (AT, "Austria"),
    (AU, "Australia"),
    (AW, "Aruba"),
    (AX, "Åland Islands"),
    (AZ, "Azerbaijan"),
    (BA, "Bosnia and Herzegovina"),
    (BB, "Barbados"),
    (BD, "Bangladesh"),
    (BE, "Belgium"),
    (BF, "Burkina Faso"),
    (BG, "Bulgaria"),
    (BH, "Bahrain"),
    (BI, "Burundi"),
    (BJ, "Benin"),
    (BL, "Saint Barthélemy"),
    (BM, "Bermuda"),
    (BN, "Brunei Darussalam"),
    (BO, "Bolivia, Plurinational State of"),
    (BQ, "Bonaire, Sint Eustatius and Saba"),
    (BR, "Brazil"),
    (BS, "Bahamas"),
    (BT, "Bhutan"),
    (BV, "Bouvet Island"),
    (BW, "Botswana"),
    (BY, "Belarus"),
    (BZ, "Belize"),
    (CA, "Canada"),
    (CC, "Cocos (Keeling) Islands"),
    (CD, "Congo, Democratic Republic of the"),
    (CF, "Central African Republic"),
    (CG, "Congo"),
    (CH, "Switzerland"),
    (CI, "Côte d'Ivoire"),
    (CK, "Cook Islands"),
    (CL, "Chile"),
    (CM, "Cameroon"),
    (CN, "China"),
    (CO, "Colombia"),
    (CR, "Costa Rica"),
    (CU, "Cuba"),
    (CV, "Cabo Verde"),
    (CW, "Curaçao"),
    (CX, "Christmas Island"),
    (CY, "Cyprus"),
    (CZ, "Czechia"),
    (DE, "Germany"),
    (DJ, "Djibouti"),
    (DK, "Denmark"),
    (DM, "Dominica"),
    (DO, "Dominican Republic"),
    (DZ, "Algeria"),
    (EC, "Ecuador"),
    (EE, "Estonia"),
    (EG, "Egypt"),
    (EH, "Western Sahara"),
    (ER, "Eritrea"),
    (ES, "Spain"),
    (ET, "Ethiopia"),
    (FI, "Finland"),
    (FJ, "Fiji"),
    (FK, "Falkland Islands (Malvinas)"),
    (FM, "Micronesia, Federated States of"),
    (FO, "Faroe Islands"),
    (FR, "France"),
    (GA, "Gabon"),
    (GB, "United Kingdom of Great Britain and Northern Ireland"),
    (GD, "Grenada"),
    (GE, "Georgia"),
    (GF, "French Guiana"),
    (GG, "Guernsey"),
    (GH, "Ghana"),
    (GI, "Gibraltar"),
    (GL, "Greenland"),
    (GM, "Gambia"),
    (GN, "Guinea"),
    (GP, "Guadeloupe"),
    (GQ, "Equatorial Guinea"),
    (GR, "Greece"),
    (GS, "South Georgia and the South Sandwich Islands"),
    (GT, "Guatemala"),
    (GU, "Guam"),
    (GW, "Guinea-Bissau"),
    (GY, "Guyana"),
    (HK, "Hong Kong"),
    (HM, "Heard Island and McDonald Islands"),
    (HN, "Honduras"),
    (HR, "Croatia"),
    (HT, "Haiti"),
    (HU, "Hungary"),
    (ID, "Indonesia"),
    (IE, "Ireland"),
    (IL, "Israel"),
    (IM, "Isle of Man"),
    (IN, "India"),
    (IO, "British Indian Ocean Territory"),
    (IQ, "Iraq"),
    (IR, "Iran, Islamic Republic of"),
    (IS, "Iceland"),
    (IT, "Italy"),
    (JE, "Jersey"),
    (JM, "Jamaica"),
    (JO, "Jordan"),
    (JP, "Japan"),
    (KE, "Kenya"),
    (KG, "Kyrgyzstan"),
    (KH, "Cambodia"),
    (KI, "Kiribati"),
    (KM, "Comoros"),
    (KN, "Saint Kitts and Nevis"),
    (KP, "Korea, Democratic People's Republic of"),
    (KR, "Korea, Republic of"),
    (KW, "Kuwait"),
    (KY, "Cayman Islands"),
    (KZ, "Kazakhstan"),
    (LA, "Lao People's Democratic Republic"),
    (LB, "Lebanon"),
    (LC, "Saint Lucia"),
    (LI, "Liechtenstein"),
    (LK, "Sri Lanka"),
    (LR, "Liberia"),
    (LS, "Lesotho"),
    (LT, "Lithuania"),
    (LU, "Luxembourg"),
    (LV, "Latvia"),
    (LY, "Libya"),
    (MA, "Morocco"),
    (MC, "Monaco"),
    (MD, "Moldova, Republic of"),
    (ME, "Montenegro"),
    (MF, "Saint Martin (French part)"),
    (MG, "Madagascar"),
    (MH, "Marshall Islands"),
    (MK, "North Macedonia"),
    (ML, "Mali"),
    (MM, "Myanmar"),
    (MN, "Mongolia"),
    (MO, "Macao"),
    (MP, "Northern Mariana Islands"),
    (MQ, "Martinique"),
    (MR, "Mauritania"),
    (MS, "Montserrat"),
    (MT, "Malta"),
    (MU, "Mauritius"),
    (MV, "Maldives"),
    (MW, "Malawi"),
    (MX, "Mexico"),
    (MY, "Malaysia"),
    (MZ, "Mozambique"),
    (NA, "Namibia"),
    (NC, "New Caledonia"),
    (NE, "Niger"),
    (NF, "Norfolk Island"),
    (NG, "Nigeria"),
    (NI, "Nicaragua"),
    (NL, "Netherlands, Kingdom of the"),
    (NO, "Norway"),
    (NP, "Nepal"),
    (NR, "Nauru"),
    (NU, "Niue"),
    (NZ, "New Zealand"),
    (OM, "Oman"),
    (PA, "Panama"),
    (PE, "Peru"),
    (PF, "French Polynesia"),
    (PG, "Papua New Guinea"),
    (PH, "Philippines"),
    (PK, "Pakistan"),
    (PL, "Poland"),
    (PM, "Saint Pierre and Miquelon"),
    (PN, "Pitcairn"),
    (PR, "Puerto Rico"),
    (PS, "Palestine, State of"),
    (PT, "Portugal"),
    (PW, "Palau"),
    (PY, "Paraguay"),
    (QA, "Qatar"),
    (RE, "Réunion"),
    (RO, "Romania"),
    (RS, "Serbia"),
    (RU, "Russian Federation"),
    (RW, "Rwanda"),
    (SA, "Saudi Arabia"),
    (SB, "Solomon Islands"),
    (SC, "Seychelles"),
    (SD, "Sudan"),
    (SE, "Sweden"),
    (SG, "Singapore"),
    (SH, "Saint Helena, Ascension and Tristan da Cunha"),
    (SI, "Slovenia"),
    (SJ, "Svalbard and Jan Mayen"),
    (SK, "Slovakia"),
    (SL, "Sierra Leone"),
    (SM, "San Marino"),
    (SN, "Senegal"),
    (SO, "Somalia"),
    (SR, "Suriname"),
    (SS, "South Sudan"),
    (ST, "Sao Tome and Principe"),
    (SV, "El Salvador"),
    (SX, "Sint Maarten (Dutch part)"),
    (SY, "Syrian Arab Republic"),
    (SZ, "Eswatini"),
    (TC, "Turks and Caicos Islands"),
    (TD, "Chad"),
    (TF, "French Southern Territories"),
    (TG, "Togo"),
    (TH, "Thailand"),
    (TJ, "Tajikistan"),
    (TK, "Tokelau"),
    (TL, "Timor-Leste"),
    (TM, "Turkmenistan"),
    (TN, "Tunisia"),
    (TO, "Tonga"),
    (TR, "Türkiye"),
    (TT, "Trinidad and Tobago"),
    (TV, "Tuvalu"),
    (TW, "Taiwan, Province of China"),
    (TZ, "Tanzania, United Republic of"),
    (UA, "Ukraine"),
    (UG, "Uganda"),
    (UM, "United States Minor Outlying Islands"),
    (US, "United States of America"),
    (UY, "Uruguay"),
    (UZ, "Uzbekistan"),
    (VA, "Holy See"),
    (VC, "Saint Vincent and the Grenadines"),
    (VE, "Venezuela, Bolivarian Republic of"),
    (VG, "Virgin Islands (British)"),
    (VI, "Virgin Islands (U.S.)"),
    (VN, "Viet Nam"),
    (VU, "Vanuatu"),
    (WF, "Wallis and Futuna"),
    (WS, "Samoa"),
    (YE, "Yemen"),
    (YT, "Mayotte"),
    (ZA, "South Africa"),
    (ZM, "Zambia"),
    (ZW, "Zimbabwe"),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(CountryCode::AR.name(), "Argentina");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "AR".parse::<CountryCode>().unwrap(),
            CountryCode::AR,
            "uppercase"
        );
        assert_eq!(
            "ar".parse::<CountryCode>().unwrap(),
            CountryCode::AR,
            "lowercase"
        );
        assert!("bad".parse::<CountryCode>().is_err(), "error");
    }

    #[test]
    fn into_static_str() {
        assert_eq!(<&'static str>::from(CountryCode::AR), "AR");
    }

    #[test]
    fn as_ref_str() {
        assert_eq!(CountryCode::AR.as_ref(), "AR");
    }

    #[test]
    fn display() {
        assert_eq!(CountryCode::AR.to_string(), "AR");
    }

    #[test]
    fn all_count() {
        let mut all: Vec<_> = CountryCode::ALL.iter().collect();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 249);
    }

    #[test]
    fn iter() {
        assert_eq!(
            CountryCode::iter().next().unwrap(),
            CountryCode::AD,
            "first"
        );
        assert_eq!(CountryCode::iter().count(), 249, "count");
    }
}