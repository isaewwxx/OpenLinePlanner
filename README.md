<div id="top"></div>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![CI/CD][ci-shield]][ci-url]
[![Code Coverage][coverage-shield]][coverage-url]

<br />
<div align="center">
  <a href="https://github.com/TheNewCivilian/OpenLinePlanner">
    <img src="doc/logo.svg" alt="Logo" width="80" height="80">
  </a>

  <h2 align="center">OpenLinePlanner</h3>

  <p align="center">
    Fast and Easy public transport network prototyping with modern web technologies
    <br />
    <a href="https://openlineplanner.com/"><strong>Check out the Demo »</strong></a>
    <br />
    <br />
    <a href="https://github.com/TheNewCivilian/OpenLinePlanner/issues">Report Bug</a>
    ·
    <a href="https://github.com/TheNewCivilian/OpenLinePlanner/issues">Request Feature</a>
    ·
    <a href="mailto:hi@xatellite.space?subject=%5BOpenlineplanner%5D">Send Feedback</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
        <li><a href="#key-features">Key Features</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#calculation-methods">Calculation Methods</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#backend-setup">Backend Setup</a></li>
        <li><a href="#frontend-setup">Frontend Setup</a></li>
        <li><a href="#development">Development</a></li>
      </ul>
    </li>
    <li><a href="#testing">Testing</a></li>
    <li><a href="#deployment">Deployment</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

## About The Project

![OpenLinePlanner Screen Shot][product-screenshot]

OpenLinePlanner is a modern, high-performance web application for prototyping public transport networks. It provides an intuitive interface for drawing transportation lines, analyzing station coverage, and optimizing station placement.

### Key Features

- 🚇 **Interactive Map Interface** - Draw and edit transportation lines with real-time feedback
- 📊 **Advanced Analytics** - Analyze coverage areas and passenger demand
- 🎯 **Optimal Station Placement** - AI-powered station location optimization
- 📱 **Responsive Design** - Works seamlessly on desktop and mobile devices
- 🔄 **Real-time Updates** - Live updates and collaborative features
- 📄 **Export Capabilities** - Export maps and analysis as PDF reports
- 🎨 **Customizable Styling** - Dark/light themes and customizable line colors
- 🔒 **Type Safety** - Full TypeScript support for better development experience

### Built With

**Frontend:**
- [![Vue][Vue.js]][Vue-url] - Progressive JavaScript framework
- [![TypeScript][TypeScript]][TypeScript-url] - Type-safe JavaScript
- [![Vite][Vite]][Vite-url] - Fast build tool and dev server
- [![Pinia][Pinia]][Pinia-url] - State management
- [![Mapbox GL][Mapbox]][Mapbox-url] - Interactive maps

**Backend:**
- [![Rust][Rust]][Rust-url] - High-performance systems programming
- [![Actix Web][Actix]][Actix-url] - Fast web framework
- [![Serde][Serde]][Serde-url] - Serialization framework
- [![Geo][Geo]][Geo-url] - Geospatial data processing

**DevOps:**
- [![Docker][Docker]][Docker-url] - Containerization
- [![GitHub Actions][GitHub Actions]][GitHub Actions-url] - CI/CD
- [![Vitest][Vitest]][Vitest-url] - Unit testing
- [![ESLint][ESLint]][ESLint-url] - Code linting

<p align="right">(<a href="#top">back to top</a>)</p>

## Usage

OpenLinePlanner allows you to:

- 🗺️ **Draw schematic transportation lines** on interactive maps
- 🏷️ **Name lines and stations** with custom identifiers
- 🎨 **Customize line colors** and styling
- 📊 **Analyze coverage areas** of stations with detailed metrics
- 📄 **Export maps and analysis** as professional PDF reports
- 🤖 **Automatically locate new stations** based on predicted demand
- 🔄 **Real-time collaboration** with team members

### Calculation Methods

The application supports multiple calculation methods for coverage analysis:

**Absolute Method (Default):**
- Shows the absolute number of residences in each station's coverage area
- Every person within the influence radius (default: 500m) is weighted equally
- Result: Direct count of potential passengers

**Relative Method:**
- Takes distance between station and residence into account
- Uses inverse square root distance weighting: `1 / sqrt(distance)`
- Result: Distance-weighted passenger potential

```javascript
// Calculation methods:
absolute: 1;
relative: 1 / sqrt(distance);
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Getting Started

### Prerequisites

- **Node.js** 18.x or higher
- **Rust** 1.70+ and Cargo
- **Git** for version control
- **Docker** (optional, for containerized deployment)

### Backend Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/TheNewCivilian/OpenLinePlanner.git
   cd OpenLinePlanner/openlineplanner-backend
   ```

2. **Install Rust dependencies:**
   ```bash
   cargo build --release
   ```

3. **Configure the application:**
   ```bash
   cp Config.toml.example Config.toml
   # Edit Config.toml with your data file paths
   ```

4. **Add data files:**
   - Download population data from [OpenPopulationEstimator](https://github.com/TheNewCivilian/OpenPopulationEstimator)
   - Download OSM data from [Protomaps](https://app.protomaps.com/downloads/osm)
   - Place files in the `data/` directory

5. **Install and run:**
   ```bash
   cargo install --path .
   openlineplanner-backend
   ```

### Frontend Setup

1. **Navigate to frontend directory:**
   ```bash
   cd ../openlineplanner
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Configure environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Start development server:**
   ```bash
   npm run dev
   ```

### Development

**Available Scripts:**
```bash
# Development
npm run dev              # Start development server
npm run build           # Build for production
npm run preview         # Preview production build

# Code Quality
npm run lint            # Run ESLint
npm run type-check      # Run TypeScript checks
npm run format          # Format code with Prettier

# Testing
npm run test            # Run unit tests
npm run test:ui         # Run tests with UI
npm run test:coverage   # Generate coverage report
```

**Backend Development:**
```bash
# Code Quality
cargo fmt               # Format code
cargo clippy           # Run linter
cargo test             # Run tests

# Performance
cargo build --release  # Optimized build
cargo bench            # Run benchmarks
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Testing

The project includes comprehensive testing infrastructure:

**Frontend Tests:**
- Unit tests with Vitest
- Component testing with Vue Test Utils
- E2E testing capabilities
- Coverage reporting

**Backend Tests:**
- Unit tests with Rust's built-in test framework
- Integration tests
- Performance benchmarks
- Code coverage with tarpaulin

**Running Tests:**
```bash
# Frontend
npm run test
npm run test:coverage

# Backend
cargo test
cargo test --release
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Deployment

### Docker Deployment

**Frontend:**
```bash
docker build -t openlineplanner-frontend .
docker run -p 3000:80 openlineplanner-frontend
```

**Backend:**
```bash
docker build -t openlineplanner-backend .
docker run -p 8080:8080 openlineplanner-backend
```

### Production Deployment

1. **Build the applications:**
   ```bash
   # Frontend
   npm run build
   
   # Backend
   cargo build --release
   ```

2. **Configure environment variables**
3. **Set up reverse proxy (nginx recommended)**
4. **Configure SSL certificates**
5. **Set up monitoring and logging**

<p align="right">(<a href="#top">back to top</a>)</p>

## Contributing

We welcome contributions! Please follow these steps:

1. **Fork the project**
2. **Create a feature branch** (`git checkout -b feature/AmazingFeature`)
3. **Make your changes** following our coding standards
4. **Run tests** to ensure everything works
5. **Commit your changes** (`git commit -m 'Add some AmazingFeature'`)
6. **Push to the branch** (`git push origin feature/AmazingFeature`)
7. **Open a Pull Request**

**Development Guidelines:**
- Follow the existing code style
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting

<p align="right">(<a href="#top">back to top</a>)</p>

## License

Distributed under the GNU GPL V3 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

## Contact

- **Email:** [hi@xatellite.space](mailto:hi@xatellite.space)
- **Project Link:** [https://github.com/TheNewCivilian/OpenLinePlanner](https://github.com/TheNewCivilian/OpenLinePlanner)
- **Documentation:** [https://openlineplanner.com/docs](https://openlineplanner.com/docs)

## Contributors

- [TheNewCivilian](https://github.com/TheNewCivilian) - Lead Developer
- [zdmx](https://github.com/zandemax) - Contributor

## Acknowledgments

- [README Template](https://github.com/othneildrew/Best-README-Template) by @OthneilDrew
- [Choose an Open Source License](https://choosealicense.com) by @ChooseaLicense
- [Img Shields](https://shields.io) by @ShieldIO

This project was created as part of the interdisciplinary project of the master class Rail Technology and Management of Railway Systems @FH-St.Pölten.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[contributors-url]: https://github.com/TheNewCivilian/OpenLinePlanner/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[forks-url]: https://github.com/TheNewCivilian/OpenLinePlanner/network/members
[stars-shield]: https://img.shields.io/github/stars/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[stars-url]: https://github.com/TheNewCivilian/OpenLinePlanner/stargazers
[issues-shield]: https://img.shields.io/github/issues/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[issues-url]: https://github.com/TheNewCivilian/OpenLinePlanner/issues
[license-shield]: https://img.shields.io/github/license/TheNewCivilian/OpenLinePlanner.svg?style=for-the-badge
[license-url]: https://github.com/TheNewCivilian/OpenLinePlanner/blob/master/LICENSE.txt
[ci-shield]: https://img.shields.io/github/actions/workflow/status/TheNewCivilian/OpenLinePlanner/ci.yml?style=for-the-badge
[ci-url]: https://github.com/TheNewCivilian/OpenLinePlanner/actions
[coverage-shield]: https://img.shields.io/codecov/c/github/TheNewCivilian/OpenLinePlanner?style=for-the-badge
[coverage-url]: https://codecov.io/gh/TheNewCivilian/OpenLinePlanner
[product-screenshot]: ./doc/images/plain.png
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[TypeScript]: https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white
[TypeScript-url]: https://www.typescriptlang.org/
[Vite]: https://img.shields.io/badge/Vite-646CFF?style=for-the-badge&logo=Vite&logoColor=white
[Vite-url]: https://vitejs.dev/
[Pinia]: https://img.shields.io/badge/Pinia-FFD02F?style=for-the-badge&logo=pinia&logoColor=black
[Pinia-url]: https://pinia.vuejs.org/
[Mapbox]: https://img.shields.io/badge/Mapbox-000000?style=for-the-badge&logo=mapbox&logoColor=white
[Mapbox-url]: https://www.mapbox.com/
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Actix]: https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=actix&logoColor=white
[Actix-url]: https://actix.rs/
[Serde]: https://img.shields.io/badge/Serde-000000?style=for-the-badge&logo=serde&logoColor=white
[Serde-url]: https://serde.rs/
[Geo]: https://img.shields.io/badge/Geo-000000?style=for-the-badge&logo=geo&logoColor=white
[Geo-url]: https://docs.rs/geo/
[Docker]: https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=Docker&logoColor=white
[Docker-url]: https://www.docker.com/
[GitHub Actions]: https://img.shields.io/badge/GitHub_Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white
[GitHub Actions-url]: https://github.com/features/actions
[Vitest]: https://img.shields.io/badge/Vitest-6E9F18?style=for-the-badge&logo=vitest&logoColor=white
[Vitest-url]: https://vitest.dev/
[ESLint]: https://img.shields.io/badge/ESLint-4B32C3?style=for-the-badge&logo=eslint&logoColor=white
[ESLint-url]: https://eslint.org/
