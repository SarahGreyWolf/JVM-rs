//FIXME: This isn't ideal
#![feature(cursor_remaining)]
/// [Class File Format](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A376%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C590%2Cnull%5D)
mod class_file;
/// [Constants](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A2201%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C256%2Cnull%5D)
mod constants;
/// [Attributes](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf#%5B%7B%22num%22%3A1244%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C72%2C564%2Cnull%5D)
mod attributes;
mod errors;
mod access_flags;
/// [JVM Spec](https://docs.oracle.com/javase/specs/jvms/se17/jvms17.pdf)
struct VirtualMachine {}
fn main() {
    println!("Hello, world!");
}
