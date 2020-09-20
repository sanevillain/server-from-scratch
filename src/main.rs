fn main() {
    if let Err(e) = server_from_scratch::listen_and_serve(9000) {
        panic!(e);
    }
}
