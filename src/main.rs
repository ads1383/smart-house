use std::collections::HashMap;
use smart_house::{room, Report, SmartDevice, SmartHouse, SmartSocket, SmartThermometer};

fn main() {
    // Создаём устройства
    let t1 = SmartThermometer::new("Термометр1", "Гостиная");
    let t2 = SmartThermometer::new("Термометр2", "Кухня");
    let mut s1 = SmartSocket::new("Розетка1", false, 1500.0);
    let s2 = SmartSocket::new("Розетка2", false, 750.0);

    // Включаем первую розетку
    s1.turn_on();

    // Оборачиваем устройства в SmartDevice
    let d1 = SmartDevice::Thermometer(t1);
    let d2 = SmartDevice::Socket(s1);
    let d3 = SmartDevice::Thermometer(t2);
    let d4 = SmartDevice::Socket(s2);

    // Создаём комнаты
    let living_room = room!("Гостиная", (d1.name(), d1), (d2.name(), d2));
    let kitchen = room!("Кухня", (d3.name(), d3), (d4.name(), d4),);

    // Создаём умный дом
    let mut home = SmartHouse::new(
        HashMap::from([(living_room.name().clone(), living_room), (kitchen.name().clone(), kitchen)])
    );

    // Отчёт по всему дому
    home.print_report();

    // Демонстрация получения доступа по индексу
    println!("-- Состояние первого устройства в гостиной --");

    match home.get_device("Гостиная", "Термометр1") {
        Ok(device) => device.print_report(),
        Err(error) => println!("Error: {:?}", error),
    }

    // Включаем первую розетку на кухне через мутабельную ссылку
    match home.get_device_mut("Кухня", "Розетка2") {
        Ok(SmartDevice::Socket(socket)) => socket.turn_on(),
        Ok(device) => println!("Found another device: {:?}", device),
        Err(error) => println!("Error: {:?}", error),
    }

    // Ещё раз отчёт по дому
    home.print_report();
}
