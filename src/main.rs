mod io;
#[cfg(test)]
mod tests;

//TODO: Args support
fn main() {
    let io = io::IO;
    io.parse(io.read("rops.txt"), 0x55000000);
}
