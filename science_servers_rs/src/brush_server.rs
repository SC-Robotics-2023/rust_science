use science_servers_rs::GPIOServer;

fn main() {
    let subsystem = "science";
    let device = "brush";
    let pin_num = 13;
    let server = GPIOServer::new(subsystem, device, pin_num).unwrap();
    server.run();
}