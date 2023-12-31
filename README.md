
# Video to ASCII Rust

This is a Rust program that converts an video into ASCII video.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- OpenCV (https://opencv.org/releases/)

## Usage

1. Clone the repository:

    ```shell
    git clone https://github.com/iamvasanth07/video-to-ascii-rust.git
    ```

2. Navigate to the project directory:

    ```shell
    cd video-to-ascii-rust
    ```

3. Build the project:

    ```shell
    cargo build --release
    ```

4. Run the program with an video file as an argument:

    ```shell
    cargo run --release -- <video>
    ```

    Replace `<video>` with the path to your video file.

## Example

Suppose you have an video named 'example.mp4'. To convert it into ASCII video, you would run:
```shell
cargo run --release -- example.mp4
```