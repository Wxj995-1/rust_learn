use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use grpc::ServerBuilder;
use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;

use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;

const PORT: u16 = 50051;
const NEARBY_RADIUS: f32 = 1.0;

fn distance(a: &Location, b: &Location) -> f32 {
    let dx = a.get_latitude() - b.get_latitude();
    let dy = a.get_longitude() - b.get_longitude();
    (dx * dx + dy * dy).sqrt()
}

#[derive(Clone)]
struct FoobarServiceImpl {
    cabs: Arc<Mutex<HashMap<String, Location>>>,
}

impl FoobarServiceImpl {
    fn new() -> FoobarServiceImpl {
        FoobarServiceImpl {
            cabs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl FoobarService for FoobarServiceImpl {
    fn record_can_location(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<CabLocationRequest>,
        resp: ServerResponseUnarySink<CabLocationResposne>,
    ) -> grpc::Result<()> {
        let name = req.message.get_name().to_owned();
        let location = req.message.get_location().clone();

        println!(
            "record cab {} at ({}, {})",
            name,
            location.get_latitude(),
            location.get_longitude()
        );

        self.cabs.lock().unwrap().insert(name, location);

        let mut r = CabLocationResposne::new();
        r.set_accepted(true);
        resp.finish(r)
    }

    fn ger_cabs(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<GetCabRequest>,
        resp: ServerResponseUnarySink<GetCabResposne>,
    ) -> grpc::Result<()> {
        let target = req.message.get_location();
        let cabs = self.cabs.lock().unwrap();
        let mut r = GetCabResposne::new();

        for (name, location) in cabs.iter() {
            if distance(target, location) <= NEARBY_RADIUS {
                let mut cab = Cab::new();
                cab.set_name(name.clone());
                cab.set_location(location.clone());
                r.mut_cabs().push(cab);
            }
        }

        println!(
            "served {} cab(s) near ({}, {})",
            r.get_cabs().len(),
            target.get_latitude(),
            target.get_longitude()
        );

        resp.finish(r)
    }
}

fn main() {
    let mut server = ServerBuilder::new_plain();
    server.http.set_port(PORT);
    server.add_service(FoobarServiceServer::new_service_def(FoobarServiceImpl::new()));

    let _server = server.build().expect("failed to start foobar server");
    println!("foobar server listening on 127.0.0.1:{}", PORT);

    loop {
        thread::park();
    }
}
