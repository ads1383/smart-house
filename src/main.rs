use std::collections::HashMap;
use std::thread;
use smart_house::{room, run_socket_emulator, run_thermometer_emulator, Report, SmartDevice, SmartHouse, SmartSocket, SmartThermometer, TcpSocketDriver, Thermometer, UdpThermometerDriver};

fn main() {
    // // Создаём устройства
    // let t1 = SmartThermometer::new("Термометр1", "Гостиная");
    // let t2 = SmartThermometer::new("Термометр2", "Кухня");
    // let mut s1 = SmartSocket::new("Розетка1", false, 1500.0);
    // let s2 = SmartSocket::new("Розетка2", false, 750.0);
    //
    // // Включаем первую розетку
    // s1.turn_on();
    //
    // // Оборачиваем устройства в SmartDevice
    // let d1 = SmartDevice::Thermometer(t1);
    // let d2 = SmartDevice::Socket(s1);
    // let d3 = SmartDevice::Thermometer(t2);
    // let d4 = SmartDevice::Socket(s2);
    //
    // // Создаём комнаты
    // let living_room = room!("Гостиная", (d1.name(), d1), (d2.name(), d2));
    // let kitchen = room!("Кухня", (d3.name(), d3), (d4.name(), d4),);
    //
    // // Создаём умный дом
    // let mut home = SmartHouse::new(
    //     HashMap::from([(living_room.name().clone(), living_room), (kitchen.name().clone(), kitchen)])
    // );
    //
    // // Отчёт по всему дому
    // home.print_report();
    //
    // // Демонстрация получения доступа по индексу
    // println!("-- Состояние первого устройства в гостиной --");
    //
    // match home.get_device("Гостиная", "Термометр1") {
    //     Ok(device) => device.print_report(),
    //     Err(error) => println!("Error: {:?}", error),
    // }
    //
    // // Включаем первую розетку на кухне через мутабельную ссылку
    // match home.get_device_mut("Кухня", "Розетка2") {
    //     Ok(SmartDevice::Socket(socket)) => socket.turn_on(),
    //     Ok(device) => println!("Found another device: {:?}", device),
    //     Err(error) => println!("Error: {:?}", error),
    // }

    // // Ещё раз отчёт по дому
    // home.print_report();

    // Включаем эмуляторы в отдельных потоках
    std::thread::spawn(|| run_socket_emulator("127.0.0.1:4000", 1500.0));
    std::thread::spawn(|| run_thermometer_emulator("127.0.0.1:5000", 1000));

    std::thread::sleep(std::time::Duration::from_secs(1)); // дать эмуляторам запуститься

    let socket_driver = TcpSocketDriver::new("127.0.0.1:4000");
    let thermo_driver = UdpThermometerDriver::new("127.0.0.1:5000");

    let mut socket = SmartSocket::new("Розетка1", Box::new(socket_driver));
    let thermometer = SmartThermometer::new("Термометр1", "Гостиная", Box::new(thermo_driver));

    socket.turn_on();
    thread::sleep(std::time::Duration::from_millis(3000));
    println!("Мощность: {}", socket.current_power());
    println!("Температура: {}", thermometer.get_current_temperature());
}
