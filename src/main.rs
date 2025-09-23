use rand::Rng;
use std::{net::UdpSocket, vec};
struct DNSHeader {
    transaction_id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_auths: u16,
    num_resources: u16,
}

impl DNSHeader
{
    fn encode(&self) ->Vec<u8>
    {
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend(self.transaction_id.to_be_bytes());
        byte_vec.extend(self.flags.to_be_bytes());
        byte_vec.extend(self.num_questions.to_be_bytes());
        byte_vec.extend(self.num_answers.to_be_bytes());
        byte_vec.extend(self.num_auths.to_be_bytes());
        byte_vec.extend(self.num_resources.to_be_bytes());
        byte_vec
    }
}

struct DNSQuestion {
    name: Vec<u8>, 
    type_: u16,
    class: u16
}

impl DNSQuestion {
    fn encode(&self) -> Vec<u8>
    {
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend(&self.name);
        byte_vec.extend(self.type_.to_be_bytes());
        byte_vec.extend(self.class.to_be_bytes());
        byte_vec
    }
}

fn encode_dns_name(name: &str) -> Vec<u8>
{
    let mut bytes: Vec<u8> = Vec::new();
    let name_parts = name.split(".");
    for part in name_parts {
        let len= part.len() as u8;
        bytes.push(len);
        bytes.extend_from_slice(part.as_bytes());
    }
    bytes.push(0);
    bytes
}

enum RecordType {
    Type_A = 1
}

// enum RecordClass {

// }
fn build_query(domain_name: &str, record_type: RecordType) -> Vec<u8>
{
    let mut bytes: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    let id = rng.gen_range(0..65535) as u16;
    let recursion_desired = (1 << 8) as u16;
    let header = DNSHeader {
        transaction_id: id,
        num_questions: 1,
        num_answers:0,
        num_resources:0,
        flags: recursion_desired,
        num_auths : 0
    };
    let question = DNSQuestion {
        name: encode_dns_name(domain_name),
        type_ : record_type as u16,
        class: 1 // CLASS_IN =1
    };
    bytes.extend_from_slice(&header.encode());
    bytes.extend_from_slice(&question.encode());
    bytes

}
fn main() {
    let header_inst = DNSHeader {
        transaction_id : 0x1314,
        flags : 0,
        num_auths : 0,
        num_resources : 0,
        num_answers: 0,
        num_questions: 1
    };
    let q = build_query("www.example.com", RecordType::Type_A);
    println!("{:?}", header_inst.encode());
    println!("{:?}", encode_dns_name("google.com"));
    println!("{:?}", q);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("erm");
    // 19:56:14.003555 wlp170s0 Out IP 192.168.86.152.43712 > 8.8.8.8.domain: 11225+ A? www.example.com. (33)
    // 19:56:14.025639 wlp170s0 In  IP 8.8.8.8.domain > 192.168.86.152.43712: 11225 4/0/0 CNAME www.example.com-v4.edgesuite.net., CNAME a1422.dscr.akamai.net., A 23.49.5.19, A 23.49.5.33 (143)
    let _ = socket.send_to(&q, "8.8.8.8:53");
    // let mut response = Vec::new();
    let mut buf = [0; 1024];
    let (len, _) = socket.recv_from(& mut buf).expect("yo");
    let mut vec_res = buf.to_vec();
    vec_res.resize(len, 0);
    println!("response: {:?}", vec_res);
    
}
