<div align="center">

# simpleHTTPd 
Georgia Tech ECE-3600: Final Project, Part 1 - Simple HTTP Web Server demo (with Cat website)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E)
![HTML5](https://img.shields.io/badge/html5-%23E34F26.svg?style=for-the-badge&logo=html5&logoColor=white)

<img src="/pics/clip.jpg" alt="Clip art" width="200" height="200">

#### Members: [Kenneth (Alex) Jenkins](https://alexj.io), Alan Aguilar, Emileigh Elwood, Shyra LaGarde, Kenny Dang, Kenneth Aguilar
#### Professor: [Dr. Chuanyi Ji](https://ece.gatech.edu/directory/chuanyi-ji)
#### Teaching Assistants: [Dulani Nivinya Wijayarathne](mailto:dwijayarathne3@gatech.edu), [Ting-Wei Hsu](mailto:thsu49@gatech.edu)

</div>

___
## 📖 Table of Contents:
* [Project Background](#Project-Background)
* [Why Use Rust?](#Why-Use-Rust)
* [Run The Program](#How-to-run)

---

## 🤔 Project Background 
This project was developed for Georgia Tech's ECE-3600 course to illustrate the construction of a simple yet secure HTTP web server entirely in Rust. The server is designed to host a minimalist cat-themed website and prioritizes secure, memory-safe handling of HTTP requests. By leveraging only the Rust programming language's standard libraries, particularly `std::net` & `std::io`, the server achieves efficient performance with a compact codebase, while Rust's ownership model inherently provides memory safety and reduces common programming pitfalls.

In 2024, memory safety remains a critical issue in software security, accounting for a substantial portion of bugs and vulnerabilities. For instance, the Chromium project, which underpins the Chrome web browser, [reports that around 70% of its severe security issues arise from memory safety problems](https://www.chromium.org/Home/chromium-security/memory-safety/). This project introduces Rust's advantages in network programming, where safety and reliability are essential. Imagine a world with 70% fewer computer bugs — software would be faster, more stable, and far less vulnerable to attacks!

Rust also outperforms garbage-collected languages (like Python), in high-concurrency environments (such as networking), where performance, responsiveness, and efficient resource management are paramount. When the server is running, users can access it locally at `localhost:8000`, where it displays a rotating selection of cat images. This project aligns with Rust’s increasing role in secure systems programming, as highlighted by initiatives like [DARPA’s TRanslating All C TO Rust (TRACTOR) project](https://www.darpa.mil/program/translating-all-c-to-rust), which aims to convert all existing C codebases to Rust.

___

## 🦀 Why Use Rust? 

**Because this code is safe**. It is very limited, but simple (under 40 lines of code) and secure - it also only utilizes Rust's standard libraries.

Using Rust for this type of experiment offers several advantages over older languages like C or C++. 

- **Memory Safety**: Rust provides memory safety without sacrificing performance, thanks to its [ownership](https://doc.rust-lang.org/1.8.0/book/ownership.html) model (also known as the [borrow-checker](https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html)). By default, this helps to entirely prevent common issues like:
  - Null Pointer Dereferencing
  - Buffer Overflows
  - Dangling Pointers
  - Double-Free Errors
  - Use-After-Free Vulnerabilities
  - Uninitialized Memory Access
  - Stack Overflows
  - Data Races in Concurrent Code

- **No Manual Memory Management**: Unlike C/C++, Rust eliminates the need for manual memory management with its built-in system of lifetimes and borrowing, significantly reducing the risk of bugs and vulnerabilities.

- **Standard Library**: Rust’s modern standard library includes tools like `std::net` for networking and `std::io` for file handling, allowing developers to build robust applications more quickly and with greater confidence in their safety and correctness.

We could've implemented this experiment in another memory safe language like [Python](https://docs.python.org/3/library/http.server.html), which is known for its ease of use and high developer productivity. Python's simple syntax and dynamic nature make it a popular choice for many projects, especially for rapid prototyping and high-level application development. 

However, as an interpreted language, Python tends to suffer from performance limitations, particularly in high-concurrency environments where handling thousands of requests per second is crucial — which is common in networking tasks like TCP/IP communication.
___

If you're interested in learning more about programming with Rust specifically for networking applications, we recommend checking out the following resources:

- The [Rust Networking Explainer](https://www.rust-lang.org/what/networking) provides an overview of Rust's networking capabilities, including its design principles and use cases.
- The [Rust Programming Language Handbook](https://doc.rust-lang.org/std/net/index.html) serves as a comprehensive guide, offering in-depth explanations of Rust’s features, best practices, and practical examples to help you get started with network programming.

These resources provide a strong foundation in Rust networking concepts and offer practical guidance for implementation. Although Rust is relatively new, having only reached stable release in 2015, it’s quickly gaining traction in industry and defense sectors, [exemplified by DARPA's TRanslating All C TO Rust (TRACTOR) project](https://www.darpa.mil/program/translating-all-c-to-rust).

> [!IMPORTANT]
> If you're interested in learning how a basic HTTP server works in Rust, check out the `src` folder!
> The `main.rs` file contains detailed documentation and comments explaining each component, making it
> a great starting point for building your own TCP/IP implementation. 

___

## 💻 How to Run:
0. [Download the Rust compiler for your Operating System](https://www.rust-lang.org/).

1. Clone this Git repository, or [download the zip](https://github.gatech.edu/kjenkins60/simpleHTTPd/archive/refs/heads/main.zip) and decompress it.
> [!TIP]
> If you already have [git](https://git-scm.com) downloaded, run `git clone https://github.gatech.edu/kjenkins60/simpleHTTPd.git`

2. Open a terminal or command prompt instance, and use the `cd` command to enter the main directory of the downloaded simpleHTTPd folder.

3. Compile for your Operating System.
> [!NOTE]
> For most Operating Systems, you can simply run `cargo build` to compile.

4. Run the program.
> [!NOTE]
> For most Operating Systems, you can simply run `cargo run` to run.

5. Navigate to `localhost:8000` in the web browser of your choosing, and enjoy some cool cat pics!

> [!WARNING]
> This project is intended to function as a demonstration web server for an academic setting and is not designed for deployment in any serious applications.
> For a secure, production ready and easy to configure web server - check out the open source [Caddy project](https://caddyserver.com/).
___
### 🤓 See Also:
- [Cat Template Website Git Repo](https://github.com/rockenman1234/CatTemplateWebsite)
- [DARPA TRACTOR Project](https://www.darpa.mil/program/translating-all-c-to-rust)
- [RFC 9112 (HTTP 1.1) Protocol](https://httpwg.org/specs/rfc9112.html)
