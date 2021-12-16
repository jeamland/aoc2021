#[repr(u8)]
#[derive(Debug)]
enum PacketType {
    Operator0 = 0,
    Operator1 = 1,
    Operator2 = 2,
    Operator3 = 3,
    Literal = 4,
    Operator5 = 5,
    Operator6 = 6,
    Operator7 = 7,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Operator0,
            1 => Self::Operator1,
            2 => Self::Operator2,
            3 => Self::Operator3,
            4 => Self::Literal,
            5 => Self::Operator5,
            6 => Self::Operator6,
            7 => Self::Operator7,
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

    fn version_sum(&self) -> usize {
        let version = self.version as usize;
        let sub_version = if let PacketBody::Operator(packets) = &self.body {
            packets.iter().map(|p| p.version_sum()).sum::<usize>()
        } else {
            0
        };

        version + sub_version
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

    println!("Version sum: {}", packet.version_sum());
}
