static BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

static PADDING: char = '=';

fn base64_encode_hex(hex: String) -> String {
    let mut result = String::new();
    let bytes = hex_to_bytes(hex);
    let mut mask: u32;
    let mut buf: u32 = 0;

    for chunk in bytes.chunks(3) {

        for ch in chunk {
            buf <<= 8;
            buf |= *ch as u32;
        }

        //padding
        if chunk.len() < 3 {
            buf <<= 8 * (3 - chunk.len());
        }

        let sxt = 6;
        for jump in (0..=3).rev() {
            if chunk.len() < 3 && chunk.len() == jump { break; }
            mask = ((1 << sxt) - 1) << jump * sxt;
            mask &= buf;
            mask >>= jump * sxt;
            result.push(BASE64_TABLE[(mask) as usize]);
        }

        buf = 0;
    }

    while result.len() % 4!= 0 { result.push(PADDING); }
    
    result
}

fn hex_to_bytes(hex: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for ch in hex.as_bytes().chunks(2) {
        let mut byte: u8 = 0;

        for b in ch {
            if ch.len() == 2 { byte <<= 4; }

            match b {
                b'0'..=b'9' => byte |= b - b'0',
                b'A'..=b'F' => byte |= b - b'A' + 10,
                b'a'..=b'f' => byte |= b - b'a' + 10,
                _ => panic!("INVALID HEX CHARACTER FOUND -> '{}'", b),
            }
        }

        bytes.push(byte);
    }
    
    bytes
}
fn main() {
    println!("{}", base64_encode_hex("deadbeef".to_string()));
}
