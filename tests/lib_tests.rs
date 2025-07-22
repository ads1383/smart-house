use std::collections::HashMap;

use smart_house::{
    room,
    Report, Room, SmartDevice, SmartHouse, SmartHouseError, SmartSocket, SmartThermometer,
};

#[test]
fn test_add_and_get_device_in_room() {
    let thermometer = SmartDevice::Thermometer(SmartThermometer::new("T1", "Кухня"));
    let mut room = Room::new("Кухня", HashMap::new());

    room.add_device(thermometer);
    assert!(room.get_device("T1").is_some());
}

#[test]
fn test_remove_device_from_room() {
    let socket = SmartDevice::Socket(SmartSocket::new("S1", true, 10.0));
    let mut room = Room::new("Гостиная", HashMap::new());
    room.add_device(socket);
    let removed = room.remove_device("S1");

    assert!(removed.is_some());
    assert!(room.get_device("S1").is_none());
}

#[test]
fn test_add_and_remove_room_in_house() {
    let room = Room::new("Спальня", HashMap::new());
    let mut house = SmartHouse::new(HashMap::new());

    house.add_room(room);
    assert!(house.get_room("Спальня").is_some());

    let removed = house.remove_room("Спальня");
    assert!(removed.is_some());
    assert!(house.get_room("Спальня").is_none());
}

#[test]
fn test_get_device_from_house() {
    let socket = SmartDevice::Socket(SmartSocket::new("S1", true, 15.0));
    let room = room!("Кабинет", ("S1".to_string(), socket));
    let mut house = SmartHouse::new(HashMap::new());
    house.add_room(room);

    let device = house.get_device("Кабинет", "S1");
    assert!(device.is_ok());
}

#[test]
fn test_get_device_error_handling() {
    let house = SmartHouse::new(HashMap::new());
    let result = house.get_device("НетКомнаты", "Устройство");

    match result {
        Err(SmartHouseError::RoomNotFound(name)) => assert_eq!(name, "НетКомнаты"),
        _ => panic!("Ожидалась ошибка RoomNotFound"),
    }
}

#[test]
fn test_report_trait() {
    let socket = SmartDevice::Socket(SmartSocket::new("S1", true, 20.0));
    let room = room!("Ванная", ("S1".to_string(), socket));
    let house = SmartHouse::new(HashMap::from([("Ванная".to_string(), room)]));

    // Тестируем только, что метод вызывается без паники
    house.print_report();
}