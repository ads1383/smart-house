use smart_house::{Room, SmartDevice, SmartHouse, SmartSocket, SmartThermometer};

fn main() {
    // Создаём устройства
    let t1 = SmartThermometer::new("Термометр1", "Гостиная");
    let t2 = SmartThermometer::new("Термометр2", "Кухня");
    let mut s1 = SmartSocket::new("Розетка1", false, 1500.0);
    let s2 = SmartSocket::new("Розетка2", true, 750.0);

    // Включаем первую розетку
    s1.turn_on();

    // Оборачиваем устройства в SmartDevice
    let d1 = SmartDevice::Thermometer(t1);
    let d2 = SmartDevice::Socket(s1);
    let d3 = SmartDevice::Thermometer(t2);
    let d4 = SmartDevice::Socket(s2);

    // Создаём комнаты
    let living_room = Room::new("Гостиная", vec![d1, d2]);
    let kitchen = Room::new("Кухня", vec![d3, d4]);

    // Создаём умный дом
    let mut home = SmartHouse::new(vec![living_room, kitchen]);

    // Отчёт по всему дому
    home.print_report();

    // Демонстрация получения доступа по индексу
    println!("-- Состояние первого устройства в гостиной --");
    let living = home.get_room(0);
    living.get_device(0).print_status();

    // Включаем вторую розетку на кухне через мутабельную ссылку
    let kitchen = home.get_room_mut(1);
    if let SmartDevice::Socket(socket) = kitchen.get_device_mut(1) {
        socket.turn_on();
    }

    // Ещё раз отчёт по дому
    home.print_report();
}
