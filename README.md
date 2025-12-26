# SceneReleaseParser

A Rust library for parsing scene release names into simpler, reusable data.

This library parses scene release names and splits the data into smaller, simpler, human readable and therefore more reusable data.

The applied rules are mostly based on studying the existing collection of Scene rules and other release examples from a PreDB, since a lot of releases are not named correctly (specially older ones).

The approach was to implement an algorithm that can really parse a variety of scene releases from all decades.

## Installation

### Using mise for Tool Management

This project uses [mise](https://mise.jdx.dev/) for managing the Rust toolchain.

1. **Install mise** (if not already installed):
   ```bash
   # macOS (Homebrew)
   brew install mise
   
   # Or using the installation script
   curl https://mise.run | sh
   ```

2. **Activate mise in your shell**:
   
   Add this to your `~/.zshrc` (for zsh) or `~/.bashrc` (for bash):
   ```bash
   eval "$(mise activate zsh)"  # for zsh
   # or
   eval "$(mise activate bash)"  # for bash
   ```
   
   Then reload your shell:
   ```bash
   source ~/.zshrc  # or source ~/.bashrc
   ```
   
   Or if mise was installed via the script:
   ```bash
   eval "$(~/.local/bin/mise activate zsh)"
   ```

3. **Install Rust through mise**:
   ```bash
   cd scene_release
   mise install
   ```

   This will install the Rust toolchain as specified in `.mise.toml`.

4. **Activate the project environment**:
   ```bash
   # mise will automatically activate when you cd into the project directory
   # Or manually activate:
   mise use
   ```

5. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```

Alternatively, you can use the provided setup script:
```bash
./setup.sh
```

### Adding to Your Project

Add this to your `Cargo.toml`:

```toml
[dependencies]
scene_release = { path = "../scene_release" }
# or from crates.io when published:
# scene_release = "0.1.0"
```

## Usage

### Basic Example

```rust
use scene_release::ReleaseParser;

fn main() {
    let parser = ReleaseParser::new("tv");
    let release_name = "24.S02E02.9.00.Uhr.bis.10.00.Uhr.German.DL.TV.Dubbed.DVDRip.SVCD.READ.NFO-c0nFuSed";
    let parsed = parser.parse(release_name);

    println!("Title: {}", parsed.title);
    println!("Season: {:?}", parsed.season);
    println!("Episode: {:?}", parsed.episode);
    println!("Group: {}", parsed.group);
    println!("Source: {}", parsed.source);
    println!("Format: {}", parsed.format);
    
    // Access specific fields
    if let Some(season) = parsed.get("season") {
        println!("Season: {}", season);
    }
}
```

### Output Structure

The parser returns a `ParsedRelease` struct with the following fields:

- `release`: The original release name
- `title`: The main title
- `title_extra`: Additional title information
- `episode_title`: Episode title (for TV shows)
- `group`: The release group name
- `year`: Optional year
- `date`: Optional date (for date-based episodes)
- `season`: Optional season number (for TV shows)
- `episode`: Optional episode number (for TV shows)
- `disc`: Optional disc number
- `flags`: Vector of flags (PROPER, REPACK, READNFO, ANiME, 3D, 10bit, etc.)
- `source`: Source type (DVDRip, WEB-DL, HDTV, BluRay, Remux, etc.)
- `format`: Format (x264, x265, h264, h265, HEVC, AVC, H.264, H.265, etc.)
- `resolution`: Resolution (1080p, 720p, 2160p, etc.)
- `audio`: Audio format (AC3, DTS, AAC, TrueHD, DTS-HD MA, EAC3 Atmos, etc.)
- `hdr`: HDR format (HDR10, DV HDR10, HDR10Plus, DV HDR10Plus)
- `streaming_provider`: Streaming provider (AMZN, NF, CR, SKST, TV2, MAX, etc.)
- `tmdb_id`: Optional TMDB ID
- `tvdb_id`: Optional TVDB ID
- `imdb_id`: Optional IMDB ID
- `edition`: Optional edition information (e.g., "Ultimate Extended Edition")
- `device`: Device (XBOX, PS3, etc.)
- `os`: Operating system (Windows, Linux, etc.)
- `version`: Version number
- `language`: HashMap of language codes to language names
- `type`: Release type (tv, movie, etc.)

### Example Output

```rust
ParsedRelease {
    release: "24.S02E02.9.00.Uhr.bis.10.00.Uhr.German.DL.TV.Dubbed.DVDRip.SVCD.READ.NFO-c0nFuSed",
    title: "24",
    title_extra: "",
    episode_title: "9 00 Uhr bis 10 00 Uhr",
    group: "c0nFuSed",
    year: None,
    date: None,
    season: Some(2),
    episode: Some(2),
    disc: None,
    flags: ["READNFO", "TV Dubbed"],
    source: "DVDRip",
    format: "SVCD",
    resolution: "",
    audio: "",
    hdr: "",
    streaming_provider: "",
    tmdb_id: None,
    tvdb_id: None,
    imdb_id: None,
    edition: None,
    device: "",
    os: "",
    version: "",
    language: {"de": "German"},
    type: "tv",
}
```

### New Format Examples

The parser supports modern release formats with metadata in brackets and IDs:

```rust
use scene_release::ReleaseParser;

// Movie with TMDB ID and technical specs
let parser = ReleaseParser::new("movie");
let release = "12.12 The Day (2023) {tmdb-919207} [Remux-1080p][TrueHD 5.1][AVC]-HBO";
let parsed = parser.parse(release);

println!("Title: {}", parsed.title);  // "12 12 The Day"
println!("Year: {:?}", parsed.year);  // Some(2023)
println!("TMDB ID: {:?}", parsed.tmdb_id);  // Some("919207")
println!("Source: {}", parsed.source);  // "Remux"
println!("Audio: {}", parsed.audio);  // "TrueHD 5.1"
println!("Format: {}", parsed.format);  // "AVC"

// TV show with episode title and HDR
let parser = ReleaseParser::new("tv");
let release = "Arrow (2012) - S05E04 - Penance [Bluray-1080p Remux][DTS-HD MA 5.1][AVC]-EPSiLON";
let parsed = parser.parse(release);

println!("Title: {}", parsed.title);  // "Arrow"
println!("Episode Title: {}", parsed.episode_title);  // "Penance"
println!("Season: {:?}", parsed.season);  // Some(5)
println!("Episode: {:?}", parsed.episode);  // Some(4)
println!("HDR: {}", parsed.hdr);  // ""
println!("Audio: {}", parsed.audio);  // "DTS-HD MA 5.1"

// TV show with TVDB ID and DV HDR10
let release = "Seinfeld (1989) {tvdb-79169} - S01E01 - The Seinfeld Chronicles [Bluray-2160p Remux Proper][DV HDR10][DTS-HD MA 5.1][HEVC]-NEWMAN";
let parsed = parser.parse(release);

println!("TVDB ID: {:?}", parsed.tvdb_id);  // Some("79169")
println!("HDR: {}", parsed.hdr);  // "DV HDR10"
println!("Flags: {:?}", parsed.flags);  // ["PROPER"]
```

### Path Parsing

The parser can parse full file paths, automatically detecting series/movie directories and season numbers:

```rust
use scene_release::ReleaseParser;

let parser = ReleaseParser::new("tv");
let path = "/tv/GBRB - Joy Pops Laugh Pops (2025) {tvdb-468780}/Season 01/GBRB - Joy Pops Laugh Pops (2025) - S01E09 - Episode 9 [WEBDL-1080p][AAC 2.0][h264]-JKCT.mkv";

if let Some(path_info) = parser.parse_path(path) {
    // Series directory information
    if let Some(directory) = &path_info.directory {
        println!("Series: {}", directory.title);
        println!("Year: {:?}", directory.year);
        println!("TVDB ID: {:?}", directory.tvdb_id);
    }
    
    // Season number
    if let Some(season) = path_info.season {
        println!("Season: {}", season);
    }
    
    // File information
    println!("File Title: {}", path_info.file.title);
    println!("Episode Title: {}", path_info.file.episode_title);
    println!("Episode: {:?}", path_info.file.episode);
}

// Movie path parsing
let parser = ReleaseParser::new("movie");
let path = "/movies/Vanilla Sky (2001) {tmdb-1903}/Vanilla Sky (2001) {tmdb-1903} [Remux-2160p Proper][DV HDR10][DTS-HD MA 5.1][HEVC]-FraMeSToR.mkv";

if let Some(path_info) = parser.parse_path(path) {
    if let Some(directory) = &path_info.directory {
        println!("Movie: {}", directory.title);
        println!("TMDB ID: {:?}", directory.tmdb_id);
    }
    println!("File Resolution: {}", path_info.file.resolution);
    println!("File HDR: {}", path_info.file.hdr);
}
```

### Directory Parsing

You can also parse directory names separately:

```rust
use scene_release::ReleaseParser;

// Parse series directory
let parser = ReleaseParser::new("series");
let dir = "The Series Title! (2010) {tvdb-79169}";
let parsed = parser.parse_series_directory(dir);
println!("Series: {}", parsed.title);
println!("TVDB ID: {:?}", parsed.tvdb_id);

// Parse movie directory
let parser = ReleaseParser::new("movie");
let dir = "The Movie Title (2010) {imdb-tt0066921}";
let parsed = parser.parse_movie_directory(dir);
println!("Movie: {}", parsed.title);
println!("IMDB ID: {:?}", parsed.imdb_id);

// Parse season directory
let parser = ReleaseParser::new("tv");
let season = parser.parse_season_directory("Season 01");
println!("Season: {:?}", season);  // Some(1)
```

## Examples

Run the example:

```bash
cargo run --example basic
```

This will demonstrate parsing various release formats including:
- Original PHP project example
- Old format movies
- New format movies with TMDB IDs and technical specs
- TV shows with episode titles
- TV shows with TVDB IDs
- Path parsing for TV shows and movies (Unix and Windows)
- Older release formats
- Streaming provider examples
- Anime and special formats

See `examples/basic.rs` for comprehensive examples of all supported formats.

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

All tests should pass:
```bash
running 56 tests
test result: ok. 56 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The test suite includes comprehensive coverage for:
- Basic TV show and movie parsing
- New format releases with IDs (TMDB, TVDB, IMDB)
- Technical specifications in brackets
- Episode titles and multiple episodes
- HDR formats
- Streaming providers
- Path parsing (Unix and Windows)
- Directory parsing
- Anime formats
- Older release formats

## Supported Formats

### TV Show Patterns
- `S01E01`, `S1E1` (standard format)
- `S01E01-E02` (multiple episodes)
- `1x01`, `10x05` (alternative format)
- `Season 1 Episode 1` (verbose format)
- `S5 - 02` (anime format)
- `- 001 -` or `- 001-003 -` (episode number format)
- `[119]` (episode number in brackets)
- `- 2013-10-30 -` (date-based episodes)

### Sources
Based on [Wikipedia's list of pirated movie release types](https://en.wikipedia.org/wiki/Pirated_movie_release_types):

**DVD Sources:**
- DVDRip, DVD-Rip, DvDrip, DVDR, DVD5, DVD9, DVD-R

**Web Sources:**
- WEB-DL, WEBRip, Web Rip, Web Download, WEB, WEBDL
- AMZN WEBDL, MA WEBDL, NF WEB-DL, CR WEB-DL

**TV Sources:**
- HDTV, PDTV, DSR, SATRip, TVRip, iNTERNAL HDTV, INTERNAL

**Blu-ray Sources:**
- BluRay, Bluray, BDRip, BRRip, BD
- Remux (including in brackets: `[Remux-1080p]`)

**Other Sources:**
- VHSRip, R5, TC, TS, CAM, SCR
- HDCAM, TELESYNC, TELECINE
- Workprint, WP, PPV Rip, PPVRip, DDC
- VOD Rip, VODRip, HC HD Rip, HCHDRip
- Web Capture, HDRip, DCP, Theatre, Theater

### Formats
- SVCD, VCD, XviD, DivX
- x264, x265, h264, h265, HEVC, AVC, H.264, H.265
- MPEG2, MPEG4

### Resolutions
- 1080p, 720p, 480p, 2160p (4K)
- Supports resolutions in brackets: `[Remux-1080p]`, `[WEBDL-2160p]`
- Supports resolutions in parentheses: `(1080p)`

### Audio Formats
- AC3, DTS, AAC, MP3
- TrueHD, DTS-HD MA, EAC3 Atmos
- Supports with channel info: `[TrueHD 5.1]`, `[DTS-HD MA 5.1]`, `[EAC3 Atmos 5.1]`
- Supports without brackets: `DTS 5.1`, `AC3`, `AAC2.0`

### HDR Formats
- HDR10
- DV HDR10 (Dolby Vision HDR10)
- HDR10Plus
- DV HDR10Plus

### Languages
- Full names: German, English, French, Spanish, Italian, Portuguese, NORDiC, SWEDiSH, NORWEGiAN
- Language codes in brackets: `[DE]`, `[JA]`, `[Eng.Hard.Sub]`
- Country codes in parentheses: `(CA)`
- Multi/MULTI (Multilingual)
- MultiSub, Multi-Subs flags

### Flags
Based on [Wikipedia's list of pirated movie release types](https://en.wikipedia.org/wiki/Pirated_movie_release_types):

**Release Quality Flags:**
- PROPER, REPACK, RERIP, NUKED, DUPE
- READNFO, READ.NFO, NFOFIX
- INTERNAL, iNTERNAL

**Audio/Subtitle Flags:**
- TV Dubbed, Dubbed, DUBBED
- Subbed, SUBBED
- Hard Sub, HardSub
- MultiSub, Multi-Subs

**Edition Flags:**
- Uncut, Director's Cut, Extended
- Limited, Limited Edition
- Special Edition, Collector's Edition
- Ultimate Edition

**Video Quality Flags:**
- IMAX, IMAX HYBRID
- 3D, 10bit
- REMASTERED

**Anime Flags:**
- ANiME

**Other Flags:**
- RETAIL, COMPLETE, FESTIVAL, STV

### IDs and Metadata
- **TMDB IDs**: `{tmdb-919207}`, `[tmdb-919207]`, `[tmdbid-919207]`
- **TVDB IDs**: `{tvdb-79169}`, `[tvdb-79169]`, `[tvdbid-79169]`
- **IMDB IDs**: `{imdb-tt0066921}`, `[imdb-tt0066921]`, `[imdbid-tt0066921]`
- **Edition**: `{edition-Ultimate Extended Edition}`, `[U-Edition]`
- **Year**: `(2023)`, `[2006]`, or standalone `2023`

### Streaming Providers
Based on [Wikipedia's list of pirated movie release types](https://en.wikipedia.org/wiki/Pirated_movie_release_types):

The parser supports **200+ streaming providers** including:

**Major International Services:**
- AMZN, Amazon, Prime Video, Prime
- NF, Netflix
- CR, Crunchyroll
- HBO, HBO Max, HMAX, MAX, Max
- Hulu, HULU
- Disney+, DSNP, DisneyPlus, Disney
- Paramount+, PMTP, Paramount Plus, Paramount
- Peacock, PCOK
- Apple TV+, ATVP, AppleTV, Apple
- Funimation, FUNI, FUNi
- MA (Movies Anywhere)
- Discovery+, DSCP, Discovery Plus, Discovery
- ESPN+, ESPN Plus, ESPN
- Showtime, SHO, Showtime Anytime
- STARZ, STZ, Starz
- Cinemax, CMAX
- MGM+, EPIX
- AMC+

**Regional Services (Sample):**
- **Australia:** 9NOW, STAN, BNGE, KAYO, TEN
- **Canada:** CBC, CRAV, CLBI, CHGD, CTV, GLBL, KNOW, SNET, WNET
- **UK/Ireland:** BBC, BBC iPlayer (iP), ITV, ITVX, ALL4, MY5, NOW, UKTV, RTE, TV3, TV4
- **Europe:** ARD, ZDF (Germany), FTV, PLUZ, TOU (France), RTP, RTPPLAY, OPTO (Portugal), SVT, TV4, CMOR, VIAP (Nordic), NRK (Norway)
- **Asia:** iQIYI, WTCH, TVING, ODK (South Korea), HTSR, HS, HPLAY, JC, SAINA, SNXT, SS, TK, MMAX (India), FPT (Vietnam), CRKI (Bangladesh)
- **Latin America:** GLOB, GLBO (Brazil), ETTV (Argentina), AO
- **Middle East:** OSN, APPS

**Japanese Services (Complete List):**
- **Broadcast Networks:** ABC, ABMA, AT-X, ATX, CX (Fuji TV), EX (TV Asahi), MX (Tokyo MX), NTV (Nippon TV), TBS, TX (TV TOKYO), YTV
- **NHK:** NHKG (NHK General TV), NHKE (NHK Education TV), BSP, NHK-BSP (NHK BS Premium)
- **BS Channels:** BS4, BS5, EX-BS, BS-EX, BS6, BS7, BSJ, BS-TX, BS8, BS-Fuji, BS11, BS12
- **CS Channels:** CS-Fuji ONE, CS3, EX-CS1, CS-EX1, CSA
- **Streaming Services:** UNXT, U-NEXT, FOD, DMM, WAKA (Wakanim), WOWOW
- **Anime Services:** ADN, ANIMAX, AO, Baha, B-Global, Bstation, CR (Crunchyroll), FUNi, HIDIVE, HIDI
- **Other:** KBC, M-ON!

**Additional Services:**
- YouTube, YT, YouTube Premium, YouTube Red, YouTube Movies, YouTube TV
- Vimeo, VMEO
- TubiTV, TUBI
- Roku, ROKU
- PlayStation Network, PSN
- Xbox Video, XBOX
- Google Play, GPLAY, PLAY
- iTunes, iT
- Microsoft Store, MS
- And many more...

See the source code for the complete list of 200+ supported providers.

### Technical Specifications in Brackets
The parser supports modern bracket notation for technical specs:
- `[Remux-1080p]` - Source and resolution
- `[TrueHD 5.1]` - Audio format with channels
- `[AVC]` - Video codec
- `[DV HDR10]` - HDR format
- `[Bluray-2160p Remux Proper]` - Combined specs

### Anime Formats
- Brackets at start: `[GM-Team][国漫][仙逆][Renegade Immortal][2023][119][AVC][GB][1080P]`
- Episode numbers in brackets: `[119]`
- Season-episode format: `S5 - 02`
- Multi-language subs: `[MultiSub]`, `(Multi-Subs, ...)`
- Release groups in brackets: `[Erai-raws]`, `[ToonsHub]`, `[SubsPlease]`

### Path Parsing
- Full file paths (Unix and Windows)
- Automatic detection of series/movie directories
- Season directory parsing: `Season 01`, `Season 1`
- Cross-platform path support (normalizes Windows backslashes)

## License

WTFPL - Do What The F*ck You Want To Public License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

