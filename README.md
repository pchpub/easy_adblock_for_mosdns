# Easy Adblock for MosDNS

This project provides an easy way to block ads using MosDNS.

## Features

- Simple setup
- Easy to use
- Lightweight and fast

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/pchpub/easy_adblock_for_mosdns.git
    ```

2. Navigate to the project directory:

    ```sh
    cd easy_adblock_for_mosdns
    ```

3. Compile the project:

    ```sh
    cargo build --profile=small
    ```

## Usage

1. Edit the `config.yaml` file to add the filter lists you want to use.

2. Run the project:

    ```sh
    cargo run --release -- config.yaml
    ```

    Or

    ```sh
    ./target/samll/easy_adblock_for_mosdns config.yaml
    ```

3. Use filter lists in the `config.yaml`(mosdns config) file to block ads.

4. Enjoy an ad-free browsing experience.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
