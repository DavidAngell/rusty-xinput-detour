<div id="top"></div>

<!-- OVERVIEW -->
# Rusty XInput Detour
A program to detour the XInput dll injected into a game. Specifically, this repo uses Rocket League as the example game. It also provides a [function scheduler](xinput_detour_dll/src/function_scheduler.rs) which allows the user to pass delays for when an input should be run on the controller. Chaining such delays allows for th e simple creation of macro buttons on a controller.

<!-- RUNNING EXAMPLE -->
## Running Example
### Prerequisites

You will need Rust and Rocket League to view the example. [Rocket League is free on Epic Games](https://store.epicgames.com/en-US/p/rocket-league) (not sponsored).
* [Rust](https://www.npmjs.com/)
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/DavidAngell/rusty-xinput-detour
   cd rusty-xinput-detour/
   ```
2. Install the nightly toolchain
    ```sh
    rustup toolchain install nightly-x86_64-pc-windows-msvc
    ```
3. Build the DLL
   ```sh
   cd xinput_detour_dll/
   cargo build
   ```
4. Run the injector script (must be in the root directory)
   ```sh
   cd ..
   cargo run
   ```

## Usage
### Editing [main.rs](src/main.rs)
- Change the value of ```EXE_NAME``` to match the name of the exe you want to detour
- IMPORTANT: if you change the lib name in Cargo.toml it will change the name of the genrated dll. You will need to change ```DLL_NAME``` at the top of main.rs to match whatever lib name you choose

### Handling Controller Actions
See examples in [handle_controller_state.rs](xinput_detour_dll/src/handle_controller_state.rs) to get an understanding of how function scheduler works.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this program better, please fork the repo and create a pull request. Thank You.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

David Angell - [@DavidJAngell42](https://twitter.com/DavidJAngell42) - davidjangell42@gmail.com


<p align="right">(<a href="#top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Othneil Drew's README Template](https://github.com/othneildrew/Best-README-Template)
