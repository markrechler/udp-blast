use clap::{App, Arg};
use socket2::{Domain, Socket, Type};
use std::{io, net::SocketAddr, net::UdpSocket, rc::Rc};

fn main() -> io::Result<()> {
    // Arg Parsing
    let matches = App::new("udp-blast")
        .version("0.1.0")
        .about("Clone and send UDP packets to n destinations.")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("SOCKETADDR")
                .default_value("0.0.0.0:8125")
                .help("Sets the address and port to listen on.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("buffer")
                .short("b")
                .long("buffer")
                .value_name("INT")
                .default_value("1500")
                .help("Sets the buffer size.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .default_value("false")
                .help("Print debug satements.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("receivers")
                .short("r")
                .long("receivers")
                .value_name("SOCKETADDR<[,]SOCKETADDR...>")
                .help("Sets the destinations receiving cloned UDP packets.")
                .required(true)
                .use_delimiter(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("port-reuse")
                .short("p")
                .long("port-reuse")
                .help("Allow multiple processes to listen and balance packets on the same port. LINUX ONLY.")
                .takes_value(false),
        )
        .get_matches();

    let listen_addr = matches
        .value_of("listen")
        .unwrap()
        .parse::<SocketAddr>()
        .unwrap();

    let print_debug: bool = matches.is_present("debug");

    let buf_size: usize = matches.value_of("buffer").unwrap().parse().unwrap();

    let recv_vec: Vec<_> = matches
        .values_of("receivers")
        .unwrap()
        .map(|sock_addr| sock_addr.parse::<SocketAddr>().unwrap())
        .collect();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    #[cfg(target_os = "linux")]
    socket.set_reuse_port(matches.is_present("port-reuse"))?;
    socket.bind(&listen_addr.into())?;
    let sock: UdpSocket = socket.into();

    println!("Starting listener on {:?}.", listen_addr);
    println!("Forwarding packets to {:?}.", recv_vec);

    let sock_clone = Rc::new(sock);

    let mut buf = vec![0u8; buf_size];
    loop {
        let (len, src) = sock_clone.recv_from(&mut buf)?;
        if print_debug {
            println!("Received {:?} bytes from {:?}.", len, src);
        }
        for recv in &recv_vec {
            let send_rc = recv.clone();
            let send_clone = sock_clone.clone();
            send_clone.send_to(&buf[..len], send_rc)?;
            if print_debug {
                println!("Sent {:?} bytes to {:?}.", len, send_rc);
            }
        }
    }
}
