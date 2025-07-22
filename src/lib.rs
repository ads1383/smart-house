use getset::{Getters, Setters};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub trait Report {
    fn print_report(&self);
}

#[derive(Debug)]
pub struct SmartThermometer {
    name: String,
    location: String,
}

pub trait Thermometer {
    fn get_current_temperature(&self) -> f32;
}

impl SmartThermometer {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            name: name.to_string(),
            location: location.to_string(),
        }
    }
}

impl Thermometer for SmartThermometer {
    fn get_current_temperature(&self) -> f32 {
        let rng = &mut rand::rng();
        rng.random_range(0.0..30.0)
    }
}

impl Display for SmartThermometer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Термометр '{}' в '{}' показывает {:.1}°C",
            self.name,
            self.location,
            self.get_current_temperature()
        )
    }
}

#[derive(Debug)]
pub struct SmartSocket {
    pub name: String,
    pub is_on: bool,
    pub power: f32,
}

impl SmartSocket {
    pub fn new(name: &str, is_on: bool, power: f32) -> Self {
        Self {
            name: name.to_string(),
            is_on,
            power,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn current_power(&self) -> f32 {
        if self.is_on { self.power } else { 0.0 }
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Розетка '{}' сейчас {}. Мощность: {:.1} Вт",
            self.name,
            if self.is_on {
                "включена"
            } else {
                "выключена"
            },
            self.current_power()
        )
    }
}

#[derive(Debug)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl SmartDevice {
    pub fn name(&self) -> String {
        match self {
            SmartDevice::Thermometer(t) => t.name.clone(),
            SmartDevice::Socket(s) => s.name.clone(),
        }
    }
}

impl Report for SmartDevice {
    fn print_report(&self) {
        match self {
            SmartDevice::Thermometer(t) => println!("{}", t),
            SmartDevice::Socket(s) => println!("{}", s),
        }
    }
}

#[derive(Debug, Getters, Setters)]
pub struct Room {
    #[getset(get = "pub")]
    name: String,
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new(name: &str, devices: HashMap<String, SmartDevice>) -> Self {
        Self {
            name: name.to_string(),
            devices,
        }
    }

    pub fn add_device(&mut self, device: SmartDevice) {
        self.devices.insert(device.name().clone(), device);
    }

    pub fn remove_device(&mut self, device_name: &str) -> Option<SmartDevice> {
        self.devices.remove(device_name)
    }

    pub fn get_device(&self, key: &str) -> Option<&SmartDevice> {
        self.devices.get(key)
    }

    pub fn get_device_mut(&mut self, key: &str) -> Option<&mut SmartDevice> {
        self.devices.get_mut(key)
    }
}

impl Report for Room {
    fn print_report(&self) {
        println!("Отчёт для комнаты '{}':", self.name);
        self.devices.values().for_each(|dev| dev.print_report());
    }
}

#[derive(Debug)]
pub struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        Self { rooms }
    }

    pub fn get_room(&self, key: &str) -> Option<&Room> {
        self.rooms.get(key)
    }

    pub fn get_room_mut(&mut self, key: &str) -> Option<&mut Room> {
        self.rooms.get_mut(key)
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name().clone(), room);
    }

    pub fn remove_room(&mut self, key: &str) -> Option<Room> {
        self.rooms.remove(key)
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDevice, SmartHouseError> {
        self
            .rooms
            .get(room_name)
            .ok_or_else(|| SmartHouseError::RoomNotFound(room_name.to_string()))?
            .devices
            .get(device_name)
            .ok_or_else(|| SmartHouseError::DeviceNotFound {
                room: room_name.to_string(),
                device: device_name.to_string(),
            })
    }

    pub fn get_device_mut(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&mut SmartDevice, SmartHouseError> {
        self
            .rooms
            .get_mut(room_name)
            .ok_or_else(|| SmartHouseError::RoomNotFound(room_name.to_string()))?
            .devices
            .get_mut(device_name)
            .ok_or_else(|| SmartHouseError::DeviceNotFound {
                room: room_name.to_string(),
                device: device_name.to_string(),
            })
    }
}

impl Report for SmartHouse {
    fn print_report(&self) {
        println!("== Отчёт по всему дому ==");
        self.rooms.values().for_each(|dev| dev.print_report());
    }
}

#[derive(Debug)]
pub enum SmartHouseError {
    RoomNotFound(String),
    DeviceNotFound { room: String, device: String },
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::RoomNotFound(room) => write!(f, "Комната '{}' не найдена", room),
            SmartHouseError::DeviceNotFound { room, device } => {
                write!(f, "Устройство '{}' не найдено в комнате '{}'", device, room)
            }
        }
    }
}

impl Error for SmartHouseError {}

#[macro_export]
macro_rules! room {
    ( $room_name:expr,  $(($device_key:expr, $device:expr)),*$(,)?) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(temp_map.insert($device_key, $device);)*
            $crate::Room::new($room_name, temp_map)
        }
    };
}
