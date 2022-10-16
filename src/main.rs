#![feature(ip)]

use std::{
    convert::Infallible,
    env,
    future::Future,
    io,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use hyper::{server::conn::http1, Response};
use socket2::Socket;
use tokio::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let port: u16 = Option::or(
        env::args().nth(1).and_then(|p| p.parse().ok()),
        env::var("PORT").ok().and_then(|p| p.parse().ok()),
    )
    .unwrap_or(8080);

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server(port))
}

fn listener(port: u16) -> io::Result<TcpListener> {
    let socket = Socket::new(
        socket2::Domain::IPV6,
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;

    socket.set_only_v6(false)?;
    socket.set_nonblocking(true)?;

    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port).into();
    socket.bind(&addr)?;
    socket.listen(128)?;

    TcpListener::from_std(socket.into())
}

async fn server(port: u16) -> io::Result<()> {
    let listener = listener(port)?;

    loop {
        let (client, addr) = listener.accept().await?;
        tokio::spawn(handler(client, addr.ip()));
    }
}

async fn handler(client: TcpStream, addr: IpAddr) {
    http1::Builder::new(Executor)
        .http1_keep_alive(false)
        .serve_connection(
            client,
            hyper::service::service_fn(|req| async move {
                let addr = req
                    .headers()
                    .get_all("x-forwarded-for")
                    .into_iter()
                    .flat_map(|v| v.as_bytes().split(|b| *b == b','))
                    .filter_map(|v| std::str::from_utf8(v).ok())
                    .filter_map(|v| v.trim().parse::<IpAddr>().ok())
                    .map(|a| a.to_canonical())
                    .find(|a| a.is_global())
                    .unwrap_or_else(|| addr.to_canonical());

                Ok::<_, Infallible>(Response::new(addr.to_string()))
            }),
        )
        .await
        .unwrap()
}

#[derive(Clone, Copy)]
struct Executor;
impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::spawn(fut);
    }
}
