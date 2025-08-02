use std::io::{BufRead, Write};
// BufRead allows us to read input data efficiently by loading chunks into memory,
// with useful features like reading one line at a time.
//
// Write gives us ways to output data, either one piece at a time or all at once.
//
// NOTE: Since we only use Rust's built-in features (no external dependencies),
// this code will work on any computer that can run the Rust compiler.

fn main() {
    // Create a TCP listener bound to, and listening on port 8000.
    //
    // NOTE: This is unique to Rust, in most other languages, you would have to use a 
    // library in order to open a socket and listen on it within the same line.
    let tcp_listener = std::net::TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server running on 127.0.0.1:8000");

    for mut tcp_stream in tcp_listener.incoming().flatten() {

        // Log client IP and port
        let client_addr = tcp_stream.peer_addr().unwrap();
        println!("Incoming connection from {}", client_addr);

        // Create a buffered reader to read the request line and headers
        let mut buffered_reader = std::io::BufReader::new(&mut tcp_stream);
        // NOTE: Rust requires the use of `&mut` for mutable references, ensuring that only one mutable reference 
        // to a value exists at a time to maintain memory safety and prevent data races.

        // Read the request line, this is the first line of the HTTP request
        let mut request_line = String::new();
        buffered_reader.read_line(&mut request_line).unwrap();
        println!("Request line: {}", request_line.trim());

        // Capture raw packet data for the request
        let mut packet_data = Vec::new();
        while let Ok(bytes_read) = buffered_reader.read_until(b'\n', &mut packet_data) {
            if bytes_read == 0 || packet_data.ends_with(b"\r\n\r\n") {
                break; // End of headers
            }
        }

        // Log raw packet data (as hex)
        println!(
            "Raw packet data:\n{}",
            packet_data.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ")
        );

        // Parse the request line to get the HTTP method and requested resource
        match request_line.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", requested_resource, "HTTP/1.1"] => {
                // Read and discard the rest of the request headers.
                //
                // NOTE: This is a very basic implementation of an HTTP server, and does not handle all possible headers.
                //
                // NOTE: As specified in RFC 9112, HTTP headers are required to indicate the purpose of the request.
                // For the purposes of this experiment, we're okay to discard most of the information the browser sent
                // back to us when the connection was established. This is not ideal for all web browsers - but works just
                // fine with any browser written in the last 15 years.
                println!("Parsed GET request for resource: {}", requested_resource);

                // Build the path to the requested resource
                let mut resource_path = std::path::PathBuf::new();
                resource_path.push("catSite"); // Base directory for resources
                resource_path.push(requested_resource.trim_start_matches("/"));

                
                if requested_resource.ends_with('/') {
                    resource_path.push("index.html");
                }

                // Check if the file exists
                if resource_path.exists() {
                    let found_response = b"HTTP/1.1 200 OK\r\n\r\n";
                    tcp_stream.write_all(found_response).unwrap();
                    let file_content = std::fs::read(resource_path).unwrap();
                    tcp_stream.write_all(&file_content).unwrap();

                    // Log details
                    println!("Response: 200 OK, {} bytes sent", file_content.len());
                } else {
                    // Respond with a 404 Not Found message if the resource does not exist
                    let not_found_response = b"HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
                    tcp_stream.write_all(not_found_response).unwrap();

                    // Logdetails
                    println!("Response: 404 Not Found, {} bytes sent", not_found_response.len());
                }
            }
            _ => {
                // Handle invalid requests
                let response = b"HTTP/1.1 400 Bad Request\r\n\r\n400 Bad Request";
                tcp_stream.write_all(response).unwrap();
                println!("Response: 400 Bad Request, {} bytes sent", response.len());
            }
        }
    }
}
