#[repr(u8)]
#[derive(Debug)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!("unknown packet type: {}", value),
        }
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal(usize),
    Operator(Vec<Packet>),
}

trait Resync {
    type Output;

    fn resync(&self, shift: usize) -> Self::Output;
}

impl<A: AsRef<[u8]>> Resync for A {
    type Output = Vec<u8>;

    fn resync(&self, shift: usize) -> Self::Output {
        let arr = self.as_ref();

        let mut shift = shift;
        let mut skip = 0;

        while shift >= 8 {
            skip += 1;
            shift -= 8;
        }

        if shift == 0 {
            return arr[skip..].to_vec();
        }

        let mut resynced = Vec::new();
        let mut byte = 0;
        let mask = !((1 << shift) - 1);

        for &b in arr.iter().skip(skip) {
            byte |= (b >> (8 - shift)) & !mask;
            resynced.push(byte);
            byte = (b << shift) & mask;
        }

        resynced.push(byte);
        resynced.remove(0);

        resynced
    }
}

impl PacketBody {
    fn parse_literal(data: &[u8]) -> (Self, usize) {
        let mut data = data.resync(6);
        let mut number = 0;
        let mut bits = 6;

        loop {
            let last = data[0] & 0x80 == 0;
            number <<= 4;
            number |= (data[0] as usize >> 3) & 0x0f;
            bits += 5;

            if last {
                return (Self::Literal(number), bits);
            }

            data = data.resync(5);
        }
    }

    fn parse_15(data: &[u8]) -> (PacketBody, usize) {
        let data = data.resync(7);

        let bits = (data[0] as usize) << 7 | (data[1] as usize) >> 1;

        let mut data = data.resync(15);
        let mut used = 0;
        let mut packets = Vec::new();

        while used < bits {
            let (packet, packet_bits) = Packet::parse_bytes(&data);
            packets.push(packet);
            data = data.resync(packet_bits);
            used += packet_bits;
        }

        (PacketBody::Operator(packets), 7 + 15 + bits)
    }

    fn parse_11(data: &[u8]) -> (PacketBody, usize) {
        let data = data.resync(7);

        let mut bits = 18;
        let npackets = (data[0] as usize) << 3 | (data[1] as usize) >> 5;
        let mut data = data.resync(11);
        let mut packets = Vec::new();

        for _ in 0..npackets {
            let (packet, packet_bits) = Packet::parse_bytes(&data);
            packets.push(packet);
            data = data.resync(packet_bits);
            bits += packet_bits;
        }

        (PacketBody::Operator(packets), bits)
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    body: PacketBody,
}

impl Packet {
    fn parse_bytes(data: &[u8]) -> (Self, usize) {
        let byte = data[0];
        let version = (byte >> 5) & 0x7;
        let packet_type = ((byte >> 2) & 0x7).into();

        let (body, bits) = match packet_type {
            PacketType::Literal => PacketBody::parse_literal(data),
            _ if byte & 0x2 == 0 => PacketBody::parse_15(data),
            _ => PacketBody::parse_11(data),
        };

        (
            Self {
                version,
                packet_type,
                body,
            },
            bits,
        )
    }

    fn evaluate(&self) -> usize {
        match (&self.packet_type, &self.body) {
            (PacketType::Literal, PacketBody::Literal(value)) => *value,
            (PacketType::Sum, PacketBody::Operator(packets)) => {
                packets.iter().map(|p| p.evaluate()).sum::<usize>()
            }
            (PacketType::Product, PacketBody::Operator(packets)) => {
                packets.iter().map(|p| p.evaluate()).product::<usize>()
            }
            (PacketType::Minimum, PacketBody::Operator(packets)) => {
                packets.iter().map(|p| p.evaluate()).min().unwrap()
            }
            (PacketType::Maximum, PacketBody::Operator(packets)) => {
                packets.iter().map(|p| p.evaluate()).max().unwrap()
            }
            (PacketType::GreaterThan, PacketBody::Operator(packets)) => {
                let values: Vec<usize> = packets.iter().map(|p| p.evaluate()).collect();
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            (PacketType::LessThan, PacketBody::Operator(packets)) => {
                let values: Vec<usize> = packets.iter().map(|p| p.evaluate()).collect();
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            (PacketType::EqualTo, PacketBody::Operator(packets)) => {
                let values: Vec<usize> = packets.iter().map(|p| p.evaluate()).collect();
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("unsupported packet"),
        }
    }
}

impl From<Vec<u8>> for Packet {
    fn from(data: Vec<u8>) -> Self {
        Self::parse_bytes(&data).0
    }
}

fn main() {
    let packet = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
    let packet: Packet = hex::decode(packet.trim()).unwrap().into();

    println!("Packet value: {}", packet.evaluate());
}
