use std::str::Chars;

fn parse(input : &str) -> Vec<u8> {
    input.chars().flat_map( | char| {
        let nr = char.to_digit(16).unwrap();
        // must be MSB first so start with
        let shifts = 0..4;
        let shifts = shifts.rev();
        shifts.map(move |bit_pos| 1& (nr >> bit_pos) as u8)
    }).collect::<Vec<u8>>()
}

fn to_decimal(bits : &[u8]) -> u64 {
    bits.iter().enumerate().map( | (bit_pos, value)| {
        let shiftby : u64 = (bits.len() - 1 - bit_pos) as u64;
        let value = *value as u64;
        value << shiftby
    }).sum::<u64>()
}

pub struct Calculator {
    version_sum : u64,
}

impl Calculator {
    fn parse_instruction<'a>(&mut self, mut bits : &'a [u8]) -> ( u64, &'a [u8]) {
        let packet_version = to_decimal(&bits[0..3]);
        let packet_type = to_decimal(&bits[3..6]);
        println!("version {} and type {}", packet_version, packet_type);
        self.version_sum += packet_version;
        bits = &bits[6..];

        if packet_type == 4 {
            // start parsing literal packet
            let mut literal_value_bits = Vec::new();

            loop {
                let (stop_bit, number, remaining_bits) = get_digit_group(bits);
                bits = remaining_bits;
                literal_value_bits.extend(number);
                if stop_bit == 0 as u8 {
                    break;
                }
            }
            println!("{:?}", literal_value_bits);
            let number = to_decimal(&literal_value_bits);
            println!("literal value: {}", number);
            return (number, bits)
        }

        // determine if next is a 0 or 1
        let subpacket_indicator = bits[0];
        bits = &bits[1..];

        let mut results = Vec::new();

        if subpacket_indicator == 0 as u8 {
            // 15 bits indicate bit length of subpackets
            let length_indicator = to_decimal(&bits[0..15]) as usize;
            bits = &bits[15..];
            let mut bits_parsed: usize = 0;
            while bits_parsed < length_indicator {
                let (result, remaining_bits) = self.parse_instruction(bits);
                bits_parsed += bits.len() - remaining_bits.len();
                bits = remaining_bits;
                results.push(result)
            }
        } else {
            // 11 bits indicate nr of subpackets
            let length_indicator = to_decimal(&bits[0..11]);
            bits = &bits[11..];
            for _ in 0..length_indicator {
                let (result, remaining_bits) = self.parse_instruction(bits);
                results.push(result);
                bits = remaining_bits;
            }
        }
        let output = {
             match packet_type {
                0 => {results.iter().sum::<u64>()},
                1 => {results.iter().product()},
                2 => {*results.iter().min().unwrap()},
                3 => {*results.iter().max().unwrap()},
                5 => { (results[0] > results[1]) as u64 },
                6 => { (results[0] < results[1]) as u64},
                7 => { (results[0] == results[1]) as u64},
                _ => {0}
            }
        };
        (output, bits)
    }
}


fn get_digit_group(mut bits: &[u8]) -> (u8, &[u8], &[u8]) {
    let chunk: &[u8] = &bits[0..5];
    println!("{:?}", chunk);
    bits = &bits[5..];
    let (stop_bit, number) = chunk.split_at(1);
    (stop_bit[0], number, bits)
}

#[cfg(test)]
mod tests {
    use crate::day16::{Calculator, parse, to_decimal};

    #[test]
    fn test_parse() {
        assert_eq!(vec![0,0,0,0], parse("0"));
        assert_eq!(vec![1,1,1,1], parse("F"));

    }

    #[test]
    fn test_to_num() {
        assert_eq!(1, to_decimal(&vec![0,0,0,1]));
        assert_eq!(15, to_decimal(&vec![1,1,1,1]));

    }

    #[test]
    fn test_instruction() {
        let example = "D2FE28";
        let bit_string = parse(example);
        let mut calculator = Calculator{ version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(answer, 2021)
    }

    #[test]
    fn test_input() {
        let input = "220D62004EF14266BBC5AB7A824C9C1802B360760094CE7601339D8347E20020264D0804CA95C33E006EA00085C678F31B80010B88319E1A1802D8010D4BC268927FF5EFE7B9C94D0C80281A00552549A7F12239C0892A04C99E1803D280F3819284A801B4CCDDAE6754FC6A7D2F89538510265A3097BDF0530057401394AEA2E33EC127EC3010060529A18B00467B7ABEE992B8DD2BA8D292537006276376799BCFBA4793CFF379D75CA1AA001B11DE6428402693BEBF3CC94A314A73B084A21739B98000010338D0A004CF4DCA4DEC80488F004C0010A83D1D2278803D1722F45F94F9F98029371ED7CFDE0084953B0AD7C633D2FF070C013B004663DA857C4523384F9F5F9495C280050B300660DC3B87040084C2088311C8010C84F1621F080513AC910676A651664698DF62EA401934B0E6003E3396B5BBCCC9921C18034200FC608E9094401C8891A234080330EE31C643004380296998F2DECA6CCC796F65224B5EBBD0003EF3D05A92CE6B1B2B18023E00BCABB4DA84BCC0480302D0056465612919584662F46F3004B401600042E1044D89C200CC4E8B916610B80252B6C2FCCE608860144E99CD244F3C44C983820040E59E654FA6A59A8498025234A471ED629B31D004A4792B54767EBDCD2272A014CC525D21835279FAD49934EDD45802F294ECDAE4BB586207D2C510C8802AC958DA84B400804E314E31080352AA938F13F24E9A8089804B24B53C872E0D24A92D7E0E2019C68061A901706A00720148C404CA08018A0051801000399B00D02A004000A8C402482801E200530058AC010BA8018C00694D4FA2640243CEA7D8028000844648D91A4001088950462BC2E600216607480522B00540010C84914E1E0002111F21143B9BFD6D9513005A4F9FC60AB40109CBB34E5D89C02C82F34413D59EA57279A42958B51006A13E8F60094EF81E66D0E737AE08";
        let bit_string = parse(input);
        let mut calculator = Calculator { version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(843, calculator.version_sum);
        assert_eq!(answer, 5390807940351)
    }

    #[test]
    fn test_nested_instructions() {
        let example = "8A004A801A8002F478";
        let bit_string = parse(example);
        let mut calculator = Calculator{ version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(16, calculator.version_sum );

        let example = "620080001611562C8802118E34";
        let bit_string = parse(example);
        let mut calculator = Calculator{ version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(12, calculator.version_sum );

        let example = "C0015000016115A2E0802F182340";
        let bit_string = parse(example);
        let mut calculator = Calculator{ version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(23, calculator.version_sum );

        let example = "A0016C880162017C3686B18A3D4780";
        let bit_string = parse(example);
        let mut calculator = Calculator{ version_sum: 0 };

        let (answer, remaining_bits) = calculator.parse_instruction(&bit_string);
        assert_eq!(31, calculator.version_sum );
    }

}