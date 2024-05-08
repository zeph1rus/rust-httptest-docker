use std::convert::Infallible;
use std::env::vars;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::client::legacy::pool::Reservation;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

static HTML_PREAMBLE: &'static str = "<!DOCTYPE html><HTML><HEAD><TITLE>Web Test</TITLE>";
static HTML_BODYSTART: &'static str = "<BODY><TABLE><TR><TH>Name</TH><TH>Value</TH></TR>";
static HTML_POSTAMBLE: &'static str = "</TABLE></BODY></HTML>";

static HTML_STYLES: &'static str = "<style>
    code {
        font-size: large;
        color: dimgrey;
    }
    body {
        font-family: Arial,serif;
    }
    th {
        background-color: darkslateblue;
        color: #FFFFFF;
    }
    td {
        background-color: whitesmoke;
    }
  th, td {
  padding: 5px;
}
</style>";

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut output = Vec::new();
    output.push(HTML_PREAMBLE.to_owned());
    output.push(HTML_STYLES.to_owned());
    output.push(HTML_BODYSTART.to_owned());

    println!("{:?}", req);
    for (k, v) in vars() {
        let out_str = format!("<TR><TD>{}</TD><TD>{}</TD></TR>", k,v,);
        output.push(out_str)
    }
    output.push(HTML_POSTAMBLE.to_owned());
    Ok(Response::new(Full::new(Bytes::from(output.join("\n")))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0,0,0,0], 5000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection {:?}", err)
            }
        });
    }
}
