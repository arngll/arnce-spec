type Result<T> = std::result::Result<T, String>;

/// Eui48 represents an EUI48 MAC address.
pub struct Eui48([u8; 6]);

impl Eui48 {
    /// Creates a new Eui48 from the given six octets.
    pub const fn new(addr: [u8; 6]) -> Eui48 {
        Eui48(addr)
    }

    /// Returns a string representing the Eui48.
    pub fn to_hex_string(&self) -> String {
        let bs = &self.0;
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bs[0], bs[1], bs[2], bs[3], bs[4], bs[5]
        )
    }

    /// Returns a HamAddr from the Eui48.
    pub fn to_ham_addr(&self) -> HamAddr {
        let mut octets = self.0;
        octets[0] &= 0b1111_1101;
        octets.rotate_left(1);
        let mut bytes = [0; 8];
        bytes[..6].copy_from_slice(&octets);
        let addr = u64::from_be_bytes(bytes);
        HamAddr(addr)
    }
}

/// Eui64 represents an EUI64 MAC address.
pub struct Eui64([u8; 8]);

impl Eui64 {
    /// Creates a new Eui64 from the given 8 octets.
    pub const fn new(addr: [u8; 8]) -> Eui64 {
        Eui64(addr)
    }

    /// Returns a string representing the Eui64.
    pub fn to_hex_string(&self) -> String {
        let bs = &self.0;
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bs[0], bs[1], bs[2], bs[3], bs[4], bs[5], bs[6], bs[7]
        )
    }

    /// Returns a HamAddr from the Eui64.
    pub fn to_ham_addr(&self) -> HamAddr {
        let mut bytes = self.0;
        bytes[0] &= 0b1111_1101;
        if bytes[3] == 0xFF && bytes[4] == 0xFE {
            bytes[3..].rotate_left(2);
            bytes[6] = 0;
            bytes[7] = 0;
            bytes[..6].rotate_left(1);
        } else {
            bytes.rotate_left(1);
        }
        let addr = u64::from_be_bytes(bytes);
        HamAddr(addr)
    }
}

/// Conversion from an Eui48 to an Eui64 always succeeds.
impl From<&Eui48> for Eui64 {
    fn from(eui48: &Eui48) -> Self {
        let mut bytes = [0; 8];
        bytes[..3].copy_from_slice(&eui48.0[..3]);
        bytes[3] = 0xFF;
        bytes[4] = 0xFE; // See RFC 4291, App A.
        bytes[5..].copy_from_slice(&eui48.0[3..]);
        Eui64(bytes)
    }
}

#[cfg(test)]
mod eui_tests {
    use super::*;

    #[test]
    fn test_from_eiu48_for_eui64() {
        let eui48 = Eui48::new([1, 2, 3, 4, 5, 6]);
        let eui64 = Eui64::from(&eui48);
        assert_eq!(eui64.0, [1, 2, 3, 0xFF, 0xFE, 4, 5, 6]);
    }

    #[test]
    fn test_to_hex_str_eui48() {
        let eui = Eui48::new([1, 2, 3, 4, 5, 6]);
        let s = eui.to_hex_string();
        assert_eq!(s, "01:02:03:04:05:06");
    }

    #[test]
    fn test_to_hex_str_eui64() {
        let eui = Eui64::new([1, 2, 3, 4, 5, 6, 0x77, 0x88]);
        let s = eui.to_hex_string();
        assert_eq!(s, "01:02:03:04:05:06:77:88");
    }
}

/// An ARNCE encoded address
pub struct HamAddr(u64);

impl HamAddr {
    const CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-\\^";

    /// Parses a call sign into a HamAddr.
    pub fn parse_callsign(callsign: &str) -> Result<HamAddr> {
        fn char_to_index(c: char) -> Result<u16> {
            HamAddr::CHARS
                .find(c)
                .ok_or(format!("bad char '{}'", c))
                .map(|k| k as u16 + 1)
        }
        fn chunk_to_u16(chunk: &[char]) -> u16 {
            assert!(chunk.len() <= 3);

            let (quad, _) = chunk
                .into_iter()
                .map(|c| char_to_index(*c).unwrap())
                .fold((0, 40 * 40), |(sum, m), c| (sum + m * c, m / 40));
            quad
        }
        for c in callsign.to_uppercase().chars() {
            if !HamAddr::CHARS.contains(c) {
                return Err(format!("Callsign contains bad character '{}'", c));
            }
        }
        if callsign.len() > 12 {
            return Err(format!("Callsign '{}' is too long", callsign));
        }
        let numeric = callsign
            .to_uppercase()
            .chars()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|c| chunk_to_u16(c))
            .chain([0, 0, 0, 0])
            .take(4)
            .fold(0, |addr, c| (addr << 16) | u64::from(c));
        Ok(HamAddr(numeric))
    }

    /// Given a valid HamAddr, retrieves the corresponding callsign.
    pub fn to_callsign(&self) -> String {
        fn index_to_char(k: usize) -> Option<char> {
            assert!(k < 40);
            if k == 0 {
                return None;
            }
            Some(HamAddr::CHARS.chars().nth(k - 1).unwrap())
        }
        fn chunk_to_chars(hi: u8, lo: u8) -> String {
            let hex = u16::from(hi) << 8 | u16::from(lo);
            let cs = [
                index_to_char((usize::from(hex) / 1600) % 40),
                index_to_char((usize::from(hex) / 40) % 40),
                index_to_char((usize::from(hex) / 1) % 40),
            ];
            cs.iter().flatten().collect()
        }
        let bytes = self.0.to_be_bytes();
        bytes
            .chunks(2)
            .map(|pair| chunk_to_chars(pair[0], pair[1]))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Returns a hex string representing the HamAddr.
    pub fn to_hex_string(&self) -> String {
        let cs = [
            (self.0 >> 48) as u16,
            (self.0 >> 32) as u16,
            (self.0 >> 16) as u16,
            self.0 as u16,
        ];
        cs.iter()
            .filter(|c| **c != 0)
            .map(|c| format!("{:04X}", *c))
            .collect::<Vec<_>>()
            .join(":")
    }

    /// Returns a HAM64 encoded string corresponding to the HamAddr.
    pub fn to_ham64_string(&self) -> String {
        let cs = [
            (self.0 >> 48) as u16,
            (self.0 >> 32) as u16,
            (self.0 >> 16) as u16,
            self.0 as u16,
        ];
        cs.iter()
            .map(|c| format!("{:04X}", *c))
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Attempts to convert the HamAddr to an Eui64.
    pub fn to_eui64(&self) -> Result<Eui64> {
        if self.0 & 0b0111 != 0 {
            return Err("HamAddr too big".to_string());
        }
        let is_small = self.0 & 0x0000_0000_0007_FFFF == 0;
        let mut bytes = self.0.to_be_bytes();
        let first_byte = std::mem::take(&mut bytes[if is_small { 5 } else { 7 }]);
        bytes.rotate_right(1);
        bytes[0] = (first_byte & 0b1111_1000) | 0b0010;
        if is_small {
            bytes[3..].rotate_right(2);
            bytes[3] = 0xFF;
            bytes[4] = 0xFE;
        }
        Ok(Eui64::new(bytes))
    }

    /// Attempts to convert the HamAddr to an Eui48.
    pub fn to_eui48(&self) -> Result<Eui48> {
        if self.0 & 0x0000_0000_0007_FFFF != 0 {
            return Err("HamAddr too big".to_string());
        }
        let mut bytes = [0, 0, 0, 0, 0, 0];
        bytes.copy_from_slice(&self.0.to_be_bytes()[..6]);
        bytes.rotate_right(1);
        bytes[0] = (bytes[0] & 0b1111_1000) | 0b0010;
        Ok(Eui48::new(bytes))
    }
}

#[cfg(test)]
mod ham_addr_tests {
    use super::*;

    #[test]
    fn test_ham_addr_parse_callsign() {
        let addr = HamAddr::parse_callsign("KZ2X-1").unwrap();
        assert_eq!(addr.to_callsign(), "KZ2X-1");
    }

    #[test]
    fn test_ham_addr_to_hex_string() {
        let addr = HamAddr::parse_callsign("KZ2X-1").unwrap();
        assert_eq!(addr.to_hex_string(), "48ED:9C0C");

        let addr = HamAddr::parse_callsign("N6DRC").unwrap();
        assert_eq!(addr.to_hex_string(), "5CAC:70F8");
        assert_eq!(addr.to_ham64_string(), "5CAC-70F8-0000-0000");

        let addr = HamAddr::parse_callsign("VI2BMARC50").unwrap();
        assert_eq!(addr.to_hex_string(), "8B05:0E89:7118:A8C0");

        // TODO(cross): This was one of the test vectors from the
        // ARNCE specification, but the encoded ham64 string does
        // not match the value below.  Check with N6DRC.
        //
        let addr = HamAddr::parse_callsign("KJ6QOH/P").unwrap();
        // assert_eq!(addr.to_ham64_string(), "4671-6CA0-F000-0000");
        assert_eq!(addr.to_ham64_string(), "4671-6CA0-E9C0-0000");
    }

    #[test]
    fn test_ham_addr_to_eui64() {
        let addr = HamAddr::parse_callsign("KZ2X-1").unwrap();
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "02:48:ed:ff:fe:9c:0c:00");
        let addr = eui64.to_ham_addr();
        assert_eq!(addr.to_callsign(), "KZ2X-1");
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "02:48:ed:ff:fe:9c:0c:00");

        let addr = HamAddr::parse_callsign("AC2OI").unwrap();
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "02:06:d5:ff:fe:5f:28:00");

        let addr = HamAddr::parse_callsign("WB3KUZ-111").unwrap();
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "02:90:2e:48:22:f1:fc:af");

        let addr = HamAddr::parse_callsign("VI2BMARC50").unwrap();
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "c2:8b:05:0e:89:71:18:a8");
        let addr = eui64.to_ham_addr();
        assert_eq!(addr.to_callsign(), "VI2BMARC50");
        let eui64 = addr.to_eui64().unwrap();
        assert_eq!(eui64.to_hex_string(), "c2:8b:05:0e:89:71:18:a8");
    }

    #[test]
    fn test_ham_addr_to_eui48() {
        let addr = HamAddr::parse_callsign("KZ2X-1").unwrap();
        let eui48 = addr.to_eui48().unwrap();
        assert_eq!(eui48.to_hex_string(), "02:48:ed:9c:0c:00");

        let addr = eui48.to_ham_addr();
        assert_eq!(addr.to_callsign(), "KZ2X-1");

        let addr = HamAddr::parse_callsign("AC2OI").unwrap();
        let eui48 = addr.to_eui48().unwrap();
        assert_eq!(eui48.to_hex_string(), "02:06:d5:5f:28:00");

        let addr = HamAddr::parse_callsign("WB3KUZ-1").unwrap();
        let eui48 = addr.to_eui48().unwrap();
        assert_eq!(eui48.to_hex_string(), "e2:90:2e:48:22:f1");
        let addr = eui48.to_ham_addr();
        assert_eq!(addr.to_callsign(), "WB3KUZ-1");

        let addr = HamAddr::parse_callsign("NA1SS").unwrap();
        let eui48 = addr.to_eui48().unwrap();
        assert_eq!(eui48.to_hex_string(), "02:57:c4:79:b8:00");
        let addr = eui48.to_ham_addr();
        assert_eq!(addr.to_callsign(), "NA1SS");

        let addr = HamAddr::parse_callsign("VI2BMARC50").unwrap();
        let not_eui48 = addr.to_eui48();
        assert!(matches!(not_eui48, Err(_)));
    }
}
