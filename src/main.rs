fn main() {
    //-- การใช้งานค่าคงที่ (Constants) --
    // กำหนดค่าเริ่มต้นของระบบด้วยค่าคงที่ ค่าเหล่านี้ไม่ควรเปลี่ยนแปลง
    const DEFAULT_PORT: u16 = 8080;
    const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

    // string จำลองที่มาจากไฟล์ config หรือ input ของผู้ใช้
    let config_input = "host=127.0.0.1\nport=3000\nmax_connections=100";

    println!("--- Parsing Configuration ---");
    println!("Input:\n{}\n", config_input);

    // -- การใช้งานตัวแปรแบบไม่เปลี่ยนรูป (Immutable Variables) --
    // เมื่อเราแยกค่า host แล้ว เราไม่ต้องการให้มันเปลี่ยนแปลงอีก
    let host = "localhost"; // ค่าเริ่มต้นก่อน parsing

    // -- การใช้งานการซ้อนทับตัวแปร (Shadowing) --
    // เราจะแปลงค่า port จาก string เป็น ตัวเลข
    // เริ่มต้นด้วย port เป็น string ที่ค่าเริ่มต้น
    let port = format!("{}", DEFAULT_PORT);
    println!("Initial host: {}, port {}", host, port);
}
