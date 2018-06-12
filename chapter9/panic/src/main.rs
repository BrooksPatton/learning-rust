fn main() {
    // panic!("Hello, world!"); // crash but clean up the memory first unless abort is set in cargo.toml file. Then yolo, the memory is now the os's problem

    let v = vec![1, 2, 3];

    v[99];
}
