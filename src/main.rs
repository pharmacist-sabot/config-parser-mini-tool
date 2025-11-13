fn main() {
    //การใช้งานค่าคงที่ (Constants)
    // กำหนดค่าเริ่มต้นของระบบด้วยค่าคงที่ ค่าเหล่านี้ไม่ควรเปลี่ยนแปลง
    const DEFAULT_PORT: u16 = 8080;
    const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

    // string จำลองที่มาจากไฟล์ config หรือ input ของผู้ใช้
    let config_input = "host=127.0.0.1\nport=3000\nmax_connections=200";

    println!("--- Parsing Configuration ---");
    println!("Input:\n{}\n", config_input);
}
