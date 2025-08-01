use capnp::{message::{Builder, ReaderOptions}, serialize_packed};
use protocol::kv_capnp::{request, response};
use std::io::Cursor;
use protocol::kv_capnp::Cmd::Set;

#[test]
fn test_request_serialization_deserialization() {
    let mut message = Builder::new_default();
    {
        let mut req = message.init_root::<request::Builder>();
        req.set_cmd(Set);
        req.set_key("my_key");
        req.set_value("my_value");
    }

    let mut buffer = Vec::new();
    serialize_packed::write_message(&mut buffer, &message).unwrap();

    let reader = serialize_packed::read_message(&mut Cursor::new(buffer), ReaderOptions::new()).unwrap();
    let req = reader.get_root::<request::Reader>().unwrap();

    assert_eq!(req.get_cmd().unwrap(), Set);
    assert_eq!(req.get_key().unwrap(), "my_key");
    assert_eq!(req.get_value().unwrap(), "my_value");
}

#[test]
fn test_response_serialization_deserialization() {
    let mut message = Builder::new_default();
    {
        let mut res = message.init_root::<response::Builder>();
        res.set_status("OK");
        res.set_value("some_value");
    }

    let mut buffer = Vec::new();
    serialize_packed::write_message(&mut buffer, &message).unwrap();

    let reader = serialize_packed::read_message(&mut Cursor::new(buffer), ReaderOptions::new()).unwrap();
    let res = reader.get_root::<response::Reader>().unwrap();

    assert_eq!(res.get_status().unwrap(), "OK");
    assert_eq!(res.get_value().unwrap(), "some_value");
}
