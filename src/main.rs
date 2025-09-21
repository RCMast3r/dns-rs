
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
    fn encode(header: DNSHeader) -> Vec<u8>
    {
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend(header.transaction_id.to_le_bytes());
        byte_vec.extend(header.flags.to_le_bytes());
        byte_vec.extend(header.num_questions.to_le_bytes());
        byte_vec.extend(header.num_answers.to_le_bytes());
        byte_vec.extend(header.num_auths.to_le_bytes());
        byte_vec.extend(header.num_resources.to_le_bytes());
        byte_vec
    }
}


struct DNSQuestion{
    name: Vec<u8>, 
    type_: u16,
    class: u16
}

fn main() {
    println!("Hello, world!");
}
