@0xda1f2fd30ec2fb4f;

struct Request {
  cmd @0 :Cmd;
  key @1 :Text;
  value @2 :Text;
}

struct Response {
  status @0 :Text;
  value @1 :Text;
}

enum Cmd {
  set @0;
  get @1;
  del @2;
}