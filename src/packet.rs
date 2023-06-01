use super::errors::BufferParseError;
use std::mem;

#[derive(Clone, Debug)]
pub struct Packet {
    pub size: i32,
    pub id: i32,
    pub packet_type: PacketType,
    pub body: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub enum PacketType {
    Auth,
    AuthResponse,
    ExecCommand,
    ResponseValue,
}

impl Packet {
    pub fn new(packet_type: PacketType, body: Vec<u8>) -> Self {
        Self {
            body: body.clone(),
            packet_type,
            size: 10 + body.len() as i32,
            ..Default::default()
        }
    }
}

impl Default for Packet {
    fn default() -> Self {
        Self {
            size: 10,
            id: 1,
            packet_type: PacketType::ExecCommand,
            body: Vec::new(),
        }
    }
}

impl From<&str> for Packet {
    fn from(value: &str) -> Self {
        Self {
            size: 10 + value.len() as i32,
            body: value.chars().map(|c| c as u8).collect(),
            ..Default::default()
        }
    }
}

impl TryFrom<&Vec<u8>> for Packet {
    type Error = BufferParseError;

    fn try_from(value: &Vec<u8>) -> Result<Self, BufferParseError> {
        if value.len() < 4 {
            return Err(BufferParseError::from(
                "Failed to obtain packet size. Packet is smaller than 4 bytes",
            ));
        }

        let size;
        unsafe {
            size = mem::transmute::<[u8; 4], i32>(*(value.as_ptr() as *const [u8; 4]));
        }

        if value.len() < size as usize + 4 {
            return Err(BufferParseError::from(
                "Buffer size is lower than the packet size",
            ));
        }

        let id;
        let packet_type;

        unsafe {
            id = mem::transmute::<[u8; 4], i32>(*(value.as_ptr().add(4) as *const [u8; 4]));
            packet_type =
                mem::transmute::<[u8; 4], i32>(*(value.as_ptr().add(8) as *const [u8; 4]));
        }

        let packet_type = match packet_type {
            3 => PacketType::Auth,
            2 => PacketType::AuthResponse,
            0 => PacketType::ResponseValue,
            _ => return Err(BufferParseError::from("Invalid packet type")),
        };

        let body = value[12..size as usize - 2].to_vec();

        Ok(Self {
            size,
            id,
            packet_type,
            body,
        })
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let mut buffer = Vec::new();

        let type_id: i32 = match self.packet_type {
            PacketType::Auth => 3,
            PacketType::AuthResponse => 2,
            PacketType::ExecCommand => 2,
            PacketType::ResponseValue => 0,
        };

        buffer.append(&mut self.size.to_le_bytes().to_vec());
        buffer.append(&mut self.id.to_le_bytes().to_vec());
        buffer.append(&mut type_id.to_le_bytes().to_vec());
        buffer.append(&mut self.body.clone());
        buffer.push(0);
        buffer.push(0);

        buffer
    }
}
