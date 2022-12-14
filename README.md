<a name="readme-top"></a>

<div align="center">
  <a href="https://github.com/aaarkid/eta/stargazers"><img src="https://img.shields.io/github/stars/aaarkid/eta.svg?style=flat" alt="Stargazers" /></a>
  <a href="https://crates.io/crates/eta"><img src="https://img.shields.io/crates/v/eta?label=version" alt="Version" /></a>
  <a href="https://codecov.io/gh/aaarkid/eta"><img src="https://img.shields.io/codecov/c/github/aaarkid/eta" alt="Code Coverage" /></a><br>
  <a href="https://github.com/aaarkid/eta/blob/master/LICENSE.txt"><img src="https://img.shields.io/github/license/aaarkid/eta.svg?style=flat" alt="License" /></a>
  <a href="https://linkedin.com/in/arkid-kaleci"><img src="https://img.shields.io/badge/-LinkedIn-black.svg?style=flat&logo=linkedin&colorB=555" alt="LinkedIn" /></a>
</div>

<br />
<div align="center">
  <a href="https://github.com/aaarkid/eta">
    <img src="images/logo.png" alt="Logo" width="210" height="">
  </a>

  <p align="center">
    Rust library for tracking progress on repetive tasks and measuring estimated remaining times.
    <br />
    <a href="https://docs.rs/eta/latest/eta/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/aaarkid/eta">View Demo</a>
    ·
    <a href="https://github.com/aaarkid/eta/issues">Report Bug</a>
    ·
    <a href="https://github.com/aaarkid/eta/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <!-- <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

ETA aims to be a simple, easy to use, and efficient library for tracking progress on repetitive tasks. The main purpose of ETA is to measure remaining time, but it's being develop to include progress tracking and progress bar display too.

### Built With

[![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting started

Add this to your `Cargo.toml`:
```rust
[dependencies]
eta = "0.2.2"
```

Add this to your source code:
```rust
use eta::{ETA, TimeAcc};
```

<!-- USAGE EXAMPLES -->
## Usage

Usage is revolved around the `Eta` Object which implements a number of functions.

Initialize Eta with `new(TasksCount, TimeAccuracy)` function where `TasksCount` is the number of tasks you want to track and `TimeAccuracy` is the accuracy of the time measurement.

`TimeAccuracy` can be one of the following:
* `TimeAcc::SEC`
* `TimeAcc::MILLI`
* `TimeAcc::MICRO`
* `TimeAcc::NANO`

Run `step()` function on every iteration of your task.

```rust
    let mut eta = Eta::new(10, TimeSteps::MILLI);

    for i in 0..10 {
        do_some_function();
        eta.step();
    }
```

_For more examples, please refer to the [Documentation](https://docs.rs/eta/0.0.0/eta/)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [x] Enable to pause and resume time tracking
- [ ] Ability to calculate weight of unequal tasks during development and create more accurate progress and ETA measurement.
- [ ] Create more options for formatting output.
- [ ] Add tolerance against outlier values.

See the [open issues](https://github.com/aaarkid/eta/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributing to this repository is **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Giving the project a star is much appreciated and means a lot! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MPL-2.0 license. See `LICENSE.MD` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

[@aaarkid](https://github.com/aaarkid) - akaleci@jacobs-university.de

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS 

---None yet---

## Acknowledgments


<p align="right">(<a href="#readme-top">back to top</a>)</p>
-->


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&color=red&logoColor=white
[Rust-url]: https://www.rust-lang.org/
