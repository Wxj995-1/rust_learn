use futures::executor;
use grpc::ClientStubExt;
use grpc::RequestOptions;

use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;

const PORT: u16 = 50051;

fn make_location(latitude: f32, longitude: f32) -> Location {
    let mut location = Location::new();
    location.set_latitude(latitude);
    location.set_longitude(longitude);
    location
}

fn record_cab(client: &FoobarServiceClient, name: &str, latitude: f32, longitude: f32) -> bool {
    let mut req = CabLocationRequest::new();
    req.set_name(name.to_owned());
    req.set_location(make_location(latitude, longitude));

    let resp = client
        .record_can_location(RequestOptions::new(), req)
        .join_metadata_result();
    let (_, response, _) = executor::block_on(resp).expect("record_can_location failed");

    response.get_accepted()
}

fn get_cabs(client: &FoobarServiceClient, latitude: f32, longitude: f32) -> Vec<Cab> {
    let mut req = GetCabRequest::new();
    req.set_location(make_location(latitude, longitude));

    let resp = client
        .ger_cabs(RequestOptions::new(), req)
        .join_metadata_result();
    let (_, response, _) = executor::block_on(resp).expect("ger_cabs failed");

    response.get_cabs().to_vec()
}

fn main() {
    let client = FoobarServiceClient::new_plain("127.0.0.1", PORT, Default::default())
        .expect("failed to connect to foobar server");

    let cabs = [
        ("cab-1", 0.1_f32, 0.1_f32),
        ("cab-2", 0.5_f32, -0.5_f32),
        ("cab-3", 5.0_f32, 5.0_f32),
    ];

    for (name, latitude, longitude) in cabs.iter() {
        let accepted = record_cab(&client, name, *latitude, *longitude);
        println!("record {} accepted: {}", name, accepted);
    }

    let nearby = get_cabs(&client, 0.0, 0.0);
    println!("{} cab(s) near (0, 0):", nearby.len());
    for cab in nearby {
        let location = cab.get_location();
        println!(
            "  {} at ({}, {})",
            cab.get_name(),
            location.get_latitude(),
            location.get_longitude()
        );
    }
}
