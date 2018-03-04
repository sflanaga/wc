extern crate quick_xml;

use std::io::prelude::*;

use std::str;

use quick_xml::Reader;
use quick_xml::events::Event;

fn sl2s(v: &[u8]) -> &str {
    return std::str::from_utf8(v).unwrap();
}

fn main() {

    let xml = r#"<tag1 att1 = "test">
                    <tag2><!--Test comment-->Test</tag2>
                    <tag2>
                        Test 2
                    </tag2>
                </tag1>"#;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;

    let mut path = Vec::new();

    //let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
        // for triggering namespaced events, use this instead:
        // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                //println!("{}", e.unescape_and_decode(&reader).unwrap());
                let nstr = String::from(sl2s(e.name()));
                path.push(nstr);
                // println!("name:  {}", sl2s(e.name()) ) ;
                // println!("path:  {}", path.join("/") ) ;
            // for namespaced:
            // Ok((ref namespace_value, Event::Start(ref e)))
                println!("raw e: {:?}", e);
                match e.name() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value)
                                        .collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let t = e.unescape_and_decode(&reader).unwrap();
                println!("{} = {}", path.join("/"), t);
            },
            Ok(Event::End(ref e)) => {
                path.pop();
                //println!("{:?}", txt);
                //txt.truncate(0);
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

}
