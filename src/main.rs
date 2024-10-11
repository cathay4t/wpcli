// SPDX-License-Identifier: Apache-2.0

fn get_socket_path() -> String {
    format!("/run/user/{}/pipewire-0", nix::unistd::getuid())
}

fn main() {
    println!("HAHA {:?}", get_socket_path());
}
