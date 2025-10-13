pub struct IcmpEcho {
    pub id: u16,
    pub seq: u16,
    pub payload: Vec<u8>,
}

impl IcmpEcho {
    // create echo packet
    pub fn new(id: u16, seq: u16, payload: Vec<u8>) -> Self {
        Self { id, seq, payload }
    }

    pub fn set_payload(&mut self, v: Vec<u8>) {
        self.payload = v;
    }

    // serialize ICMP echo
    pub fn encode(&self) -> Vec<u8> {
        // ICMP Echo Request: type=8, code=0
        let mut buf = Vec::with_capacity(8 + self.payload.len());
        buf.push(8); // type
        buf.push(0); // code
        buf.extend_from_slice(&[0, 0]); // checksum placeholder
        put_u16_be(&mut buf, self.id);
        put_u16_be(&mut buf, self.seq);
        buf.extend_from_slice(&self.payload);

        // compute checksum over the whole ICMP message
        let cs = icmp_checksum(&buf);
        let [hi, lo] = cs.to_be_bytes();
        buf[2] = hi;
        buf[3] = lo;
        buf
    }

    /// Parse ICMP Echo from a buffer of raw ICMP bytes (not including IP header).
    /// Validate that Type=8 or Type=0 depending on your use case, and verify checksum.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
        // minimal length check: 8 bytes header
        if buf.len() < 8 {
            return Err("ICMP packet too short".into());
        }
        let icmp_type = buf[0];
        let icmp_code = buf[1];
        // accept echo request (8)/echo reply (0), both must have code 0
        if !(icmp_type == 8 || icmp_type == 0) {
            return Err(format!("Unexpected ICMP type: {}", icmp_type));
        }
        if icmp_code != 0 {
            return Err(format!("Unexpected ICMP code: {}", icmp_code));
        }
        // verify checksum ICMP message
        if icmp_checksum(buf) != 0 {
            return Err("Bad ICMP checksum".into());
        }
        // read id/seq (big-endian)
        let id = read_u16_be(buf, 4).ok_or("Truncated id field")?;
        let seq = read_u16_be(buf, 6).ok_or("Truncated seq field")?;
        // copy payload
        let payload = buf[8..].to_vec();
        Ok(Self { id, seq, payload })
    }
}

/// Если перед ICMP есть IPv4-заголовок — отрежем его.
/// IHL (Internet Header Length): младшие 4 бита первого байта, длина в 32-битных словах (умножаем на 4)
pub fn strip_ipv4_header(package: &[u8]) -> &[u8] {
    if package.len() >= 20 && (package[0] >> 4) == 4 {
        let ihl_bytes = ((package[0] & 0x0F) as usize) * 4;
        if ihl_bytes >= 20 && ihl_bytes <= package.len() {
            &package[ihl_bytes..]
        } else {
            package
        }
    } else {
        package
    }
}

/// Compute ICMP 1's-complement checksum over `buf` (header + payload).
/// RFC 1071: везде big-endian, идёт суммирование слов по 16 бит. если не хватило - добивание нулями, в конце инвертирование
pub fn icmp_checksum(buf: &[u8]) -> u16 {
    let mut checksum: u32 = 0;

    let mut i = 0;
    while i + 1 < buf.len() {
        let first = buf[i];
        let second = buf[i + 1];
        let word = u16::from_be_bytes([first, second]) as u32;
        checksum += word;
        i += 2;
    }

    if i < buf.len() {
        let first = buf[i];
        let second = 0_u8;
        let word = u16::from_be_bytes([first, second]) as u32;
        checksum += word;
    };

    while checksum > 0xffff {
        checksum = (checksum >> 16) + (checksum & 0xffff);
    }

    !(checksum as u16)
}

#[inline]
fn put_u16_be(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_be_bytes());
}

/// Чтение из массива байт, читаются слова по 2 байта, offset - смещение в байтах
#[inline]
fn read_u16_be(buf: &[u8], offset: usize) -> Option<u16> {
    if offset + 2 > buf.len() {
        None
    } else {
        Some(u16::from_be_bytes([buf[offset], buf[offset + 1]]))
    }
}
