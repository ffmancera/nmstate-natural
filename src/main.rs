use std::{fs::File, io::Read};

use clap::Parser;
use nmstate::NetworkState;

/// Tool to generate natural language description from a Nmstate network state.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// YAML file path for the network state
    #[arg(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", state_to_natural_language(cli.file));
}

fn state_to_natural_language(path: String) -> String {
    let mut output = String::new();

    let mut file = File::open(path).unwrap();
    let mut state_str = String::new();
    file.read_to_string(&mut state_str).unwrap();

    let network_state = NetworkState::new_from_yaml(state_str.as_str()).unwrap();

    for iface in network_state.interfaces.iter() {
        match iface.base_iface().state {
            nmstate::InterfaceState::Up => output.push_str("Configure "),
            nmstate::InterfaceState::Down | nmstate::InterfaceState::Absent => {
                output.push_str("Remove ")
            }
            _ => {}
        }

        match iface.iface_type() {
            nmstate::InterfaceType::Ethernet => output.push_str("ethernet interface "),
            _ => {}
        }

        output.push_str(format!("{} ", iface.base_iface().name).as_str());

        match &iface.base_iface().ipv4 {
            Some(ipv4) => {
                if ipv4.enabled {
                    output.push_str("with ");

                    if ipv4.addresses.is_some() {
                        let addresses = ipv4.addresses.as_ref().unwrap();
                        if addresses.len() > 1 {
                            output.push_str("addresses ");
                        } else {
                            output.push_str("address ");
                        }

                        for address in addresses {
                            output.push_str(format!("{}", address.ip.to_string()).as_str());

                            if address.prefix_length != 24 {
                                output.push_str(format!("/{} ", address.prefix_length).as_str());
                            } else {
                                output.push_str(" ");
                            }
                        }

                        if ipv4.dhcp.is_some_and(|option| option) {
                            output.push_str("and dhcp ");
                        }
                    } else {
                        if ipv4.dhcp.is_some_and(|option| option) {
                            output.push_str("dhcp ");
                        }
                    }
                }
            }
            None => {}
        }
    }

    output
}
