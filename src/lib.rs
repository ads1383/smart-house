use rand::Rng;
use std::fmt::{self, Display, Formatter};

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

pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl SmartDevice {
    pub fn print_status(&self) {
        match self {
            SmartDevice::Thermometer(t) => println!("{}", t),
            SmartDevice::Socket(s) => println!("{}", s),
        }
    }
}

pub struct Room {
    pub name: String,
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(name: &str, devices: Vec<SmartDevice>) -> Self {
        Self {
            name: name.to_string(),
            devices,
        }
    }

    pub fn get_device(&self, idx: usize) -> &SmartDevice {
        self.devices
            .get(idx)
            .expect("Индекс устройства вне диапазона")
    }

    pub fn get_device_mut(&mut self, idx: usize) -> &mut SmartDevice {
        self.devices
            .get_mut(idx)
            .expect("Индекс устройства вне диапазона")
    }

    pub fn print_report(&self) {
        println!("Отчёт для комнаты '{}':", self.name);
        self.devices.iter().for_each(|dev| dev.print_status());
    }
}

pub struct SmartHouse {
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    pub fn get_room(&self, idx: usize) -> &Room {
        self.rooms.get(idx).expect("Индекс комнаты вне диапазона")
    }

    pub fn get_room_mut(&mut self, idx: usize) -> &mut Room {
        self.rooms
            .get_mut(idx)
            .expect("Индекс комнаты вне диапазона")
    }

    pub fn print_report(&self) {
        println!("== Отчёт по всему дому ==");
        self.rooms.iter().for_each(|dev| dev.print_report());
    }
}
