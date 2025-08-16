use getset::{Getters, Setters};
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use std::thread;

pub trait Report {
    fn print_report(&self);
}

pub trait SocketDriver: Send + Sync + Debug {
    fn turn_on(&mut self) -> Result<(), Box<dyn Error>>;
    fn turn_off(&mut self) -> Result<(), Box<dyn Error>>;
    fn is_on(&self) -> Result<bool, Box<dyn Error>>;
    fn current_power(&self) -> Result<f32, Box<dyn Error>>;
}

pub trait ThermometerDriver: Send + Sync + Debug {
    fn latest_temperature(&self) -> Result<f32, Box<dyn Error>>;
}

#[derive(Clone, Debug)]
pub struct TcpSocketDriver {
    addr: String,
}

impl TcpSocketDriver {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    fn send_cmd(&self, cmd: &str) -> Result<String, Box<dyn Error>> {
        let mut stream = TcpStream::connect(&self.addr)?;
        stream.write_all(cmd.as_bytes())?;
        stream.flush()?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        Ok(buf)
    }
}


impl SocketDriver for TcpSocketDriver {
    fn turn_on(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_cmd("ON")?;
        Ok(())
    }
    fn turn_off(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_cmd("OFF")?;
        Ok(())
    }

    fn is_on(&self) -> Result<bool, Box<dyn Error>> {
        let resp = self.send_cmd("STATE")?;
        Ok(resp.trim() == "ON")
    }


    fn current_power(&self) -> Result<f32, Box<dyn Error>> {
        let resp = self.send_cmd("POWER")?;
        Ok(resp.trim().parse()?)
    }
}

#[derive(Clone, Debug)]
pub struct MockSocketDriver {
    state: Arc<Mutex<(bool, f32)>>,
}

impl MockSocketDriver {
    pub fn new(initial_state: bool, power: f32) -> Self {
        Self { state: Arc::new(Mutex::new((initial_state, power))) }
    }
}

impl SocketDriver for MockSocketDriver {
    fn turn_on(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().0 = true;
        Ok(())
    }
    fn turn_off(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().0 = false;
        Ok(())
    }

    fn is_on(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self.state.lock().unwrap().0)
    }
    fn current_power(&self) -> Result<f32, Box<dyn Error>> {
        let (on, power) = *self.state.lock().unwrap();
        Ok(if on { power } else { 0.0 })
    }
}

#[derive(Clone, Debug)]
pub struct UdpThermometerDriver {
    latest_temp: Arc<Mutex<Option<f32>>>,
}

impl UdpThermometerDriver {
    pub fn new(bind_addr: &str) -> Self {
        let latest_temp = Arc::new(Mutex::new(None));
        let latest_temp_clone = Arc::clone(&latest_temp);

        let addr = bind_addr.to_string();
        thread::spawn(move || {
            let socket = UdpSocket::bind(&addr).expect("UDP bind failed");
            socket.set_nonblocking(true).unwrap();
            let mut buf = [0u8; 64];

            loop {
                if let Ok((len, _)) = socket.recv_from(&mut buf) {
                    if let Ok(s) = std::str::from_utf8(&buf[..len]) {
                        if let Ok(temp) = s.trim().parse::<f32>() {
                            *latest_temp_clone.lock().unwrap() = Some(temp);
                        }
                    }
                }
                thread::sleep(std::time::Duration::from_millis(200));
            }
        });

        Self { latest_temp }
    }
}

impl ThermometerDriver for UdpThermometerDriver {
    fn latest_temperature(&self) -> Result<f32, Box<dyn Error>> {
        self.latest_temp
            .lock()
            .unwrap()
            .ok_or_else(|| "Нет данных от термометра".into())
    }
}

#[derive(Clone, Debug)]
pub struct MockThermometerDriver {
    temp: f32,
}

impl MockThermometerDriver {
    pub fn new(temp: f32) -> Self {
        Self { temp }
    }
}

impl ThermometerDriver for MockThermometerDriver {
    fn latest_temperature(&self) -> Result<f32, Box<dyn Error>> {
        Ok(self.temp)
    }
}

#[derive(Debug)]
pub struct SmartThermometer {
    name: String,
    location: String,
    driver: Box<dyn ThermometerDriver>,
}

pub trait Thermometer {
    fn get_current_temperature(&self) -> f32;
}

impl SmartThermometer {
    pub fn new(name: &str, location: &str, driver: Box<dyn ThermometerDriver>) -> Self {
        Self {
            name: name.to_string(),
            location: location.to_string(),
            driver
        }
    }
}

impl Thermometer for SmartThermometer {
    fn get_current_temperature(&self) -> f32 {
        self.driver.latest_temperature().unwrap_or_else(|_| 0.0)
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
    driver: Box<dyn SocketDriver>
}

impl SmartSocket {
    pub fn new(name: &str, driver: Box<dyn SocketDriver>) -> Self {
        Self {
            name: name.to_string(),
            driver,
        }
    }

    pub fn turn_on(&mut self) {
        self.driver.turn_on().expect("Ошибка включения розетки");
    }

    pub fn turn_off(&mut self) {
        self.driver.turn_off().expect("Ошибка выключения розетки");
    }

    pub fn is_on(&self) -> bool {
        self.driver.is_on().unwrap_or_else(|_| false)
    }

    pub fn current_power(&self) -> f32 {
        self.driver.current_power().unwrap_or_else(|_| 0.0)
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Розетка '{}' сейчас {}. Мощность: {:.1} Вт",
            self.name,
            if self.is_on() {
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

//Имитаторы

//TCP-розетка
pub fn run_socket_emulator(addr: &str, initial_power: f32) {
    let listener = TcpListener::bind(addr).unwrap();
    let state = Arc::new(Mutex::new((false, initial_power)));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let st = Arc::clone(&state);
            thread::spawn(move || handle_client(stream, st));
        }
    }
}

fn handle_client(mut stream: TcpStream, state: Arc<Mutex<(bool, f32)>>) {
    let mut buf = [0u8; 64];
    if let Ok(size) = stream.read(&mut buf) {
        let cmd = String::from_utf8_lossy(&buf[..size]).trim().to_string();
        let mut st = state.lock().unwrap();
        match cmd.as_str() {
            "ON" => { st.0 = true; let _ = stream.write_all(b"OK"); }
            "OFF" => { st.0 = false; let _ = stream.write_all(b"OK"); }
            "POWER" => {
                let resp = if st.0 { st.1 } else { 0.0 };
                let _ = stream.write_all(resp.to_string().as_bytes());
            }
            "STATE" => {
                let resp = if st.0 { "ON" } else { "OFF" };
                let _ = stream.write_all(resp.as_bytes());
            }
            _ => { let _ = stream.write_all(b"ERR"); }
        }
    }
}

//UDP-термометр
pub fn run_thermometer_emulator(target_addr: &str, period_ms: u64) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    loop {
        let temp = rand::random::<f32>() * 30.0;
        let msg = format!("{:.2}", temp);
        let _ = socket.send_to(msg.as_bytes(), target_addr);
        thread::sleep(std::time::Duration::from_millis(period_ms));
    }
}
