use scene_release::ReleaseParser;

fn main() {
    // Example from the original PHP project
    let parser = ReleaseParser::new("tv");
    let release_name = "24.S02E02.9.00.Uhr.bis.10.00.Uhr.German.DL.TV.Dubbed.DVDRip.SVCD.READ.NFO-c0nFuSed";
    let parsed = parser.parse(release_name);

    println!("Parsed Release:");
    println!("  Release: {}", parsed.release);
    println!("  Title: {}", parsed.title);
    println!("  Episode Title: {}", parsed.episode_title);
    println!("  Group: {}", parsed.group);
    println!("  Season: {:?}", parsed.season);
    println!("  Episode: {:?}", parsed.episode);
    println!("  Source: {}", parsed.source);
    println!("  Format: {}", parsed.format);
    println!("  Flags: {:?}", parsed.flags);
    println!("  Languages: {:?}", parsed.language);
    println!("  Type: {}", parsed.release_type);

    // Access specific fields using get()
    println!("\nUsing get() method:");
    println!("  Season: {:?}", parsed.get("season"));
    println!("  Episode: {:?}", parsed.get("episode"));
    println!("  Source: {:?}", parsed.get("source"));
    println!("  Format: {:?}", parsed.get("format"));

    // Movie example (old format)
    println!("\n--- Movie Example (Old Format) ---");
    let movie_parser = ReleaseParser::new("movie");
    let movie_name = "The.Matrix.1999.1080p.BluRay.x264-GROUP";
    let movie_parsed = movie_parser.parse(movie_name);

    println!("  Title: {}", movie_parsed.title);
    println!("  Year: {:?}", movie_parsed.year);
    println!("  Resolution: {}", movie_parsed.resolution);
    println!("  Source: {}", movie_parsed.source);
    println!("  Format: {}", movie_parsed.format);
    println!("  Group: {}", movie_parsed.group);

    // Movie example (new format with TMDB and brackets)
    println!("\n--- Movie Example (New Format) ---");
    let new_movie = "12.12 The Day (2023) {tmdb-919207} [Remux-1080p][TrueHD 5.1][AVC]-HBO";
    let new_movie_parsed = movie_parser.parse(new_movie);

    println!("  Title: {}", new_movie_parsed.title);
    println!("  Year: {:?}", new_movie_parsed.year);
    println!("  TMDB ID: {:?}", new_movie_parsed.tmdb_id);
    println!("  Resolution: {}", new_movie_parsed.resolution);
    println!("  Source: {}", new_movie_parsed.source);
    println!("  Audio: {}", new_movie_parsed.audio);
    println!("  Format: {}", new_movie_parsed.format);
    println!("  HDR: {}", new_movie_parsed.hdr);
    println!("  Group: {}", new_movie_parsed.group);

    // TV show example (new format with episode title)
    println!("\n--- TV Show Example (New Format) ---");
    let new_tv = "Arrow (2012) - S05E04 - Penance [Bluray-1080p Remux][DTS-HD MA 5.1][AVC]-EPSiLON";
    let new_tv_parsed = parser.parse(new_tv);

    println!("  Title: {}", new_tv_parsed.title);
    println!("  Year: {:?}", new_tv_parsed.year);
    println!("  Season: {:?}", new_tv_parsed.season);
    println!("  Episode: {:?}", new_tv_parsed.episode);
    println!("  Episode Title: {}", new_tv_parsed.episode_title);
    println!("  Resolution: {}", new_tv_parsed.resolution);
    println!("  Source: {}", new_tv_parsed.source);
    println!("  Audio: {}", new_tv_parsed.audio);
    println!("  Format: {}", new_tv_parsed.format);
    println!("  Group: {}", new_tv_parsed.group);

    // TV show example with TVDB ID
    println!("\n--- TV Show Example (With TVDB ID) ---");
    let tvdb_tv = "Seinfeld (1989) {tvdb-79169} - S01E01 - The Seinfeld Chronicles [Bluray-2160p Remux Proper][DV HDR10][DTS-HD MA 5.1][HEVC]-NEWMAN";
    let tvdb_tv_parsed = parser.parse(tvdb_tv);

    println!("  Title: {}", tvdb_tv_parsed.title);
    println!("  Year: {:?}", tvdb_tv_parsed.year);
    println!("  TVDB ID: {:?}", tvdb_tv_parsed.tvdb_id);
    println!("  Season: {:?}", tvdb_tv_parsed.season);
    println!("  Episode: {:?}", tvdb_tv_parsed.episode);
    println!("  Episode Title: {}", tvdb_tv_parsed.episode_title);
    println!("  Resolution: {}", tvdb_tv_parsed.resolution);
    println!("  Source: {}", tvdb_tv_parsed.source);
    println!("  HDR: {}", tvdb_tv_parsed.hdr);
    println!("  Audio: {}", tvdb_tv_parsed.audio);
    println!("  Format: {}", tvdb_tv_parsed.format);
    println!("  Flags: {:?}", tvdb_tv_parsed.flags);
    println!("  Group: {}", tvdb_tv_parsed.group);
    
    // Access TVDB ID using get() method
    if let Some(tvdb_id) = tvdb_tv_parsed.get("tvdb_id") {
        println!("  TVDB ID (via get()): {}", tvdb_id);
    }

    // Path parsing examples
    println!("\n--- Path Parsing Example (TV Show) ---");
    let tv_parser = ReleaseParser::new("tv");
    let tv_path = "/mnt/ttt/shows/The Series Title! (2010)/Season 01/The Series Title! (2010) - S01E01-E03 - Episode Title [AMZN WEBDL-1080p Proper][DV HDR10][DTS 5.1][x264]-RlsGrp.mkv";
    
    if let Some(path_info) = tv_parser.parse_path(tv_path) {
        println!("  Full Path: {}", path_info.full_path);
        
        if let Some(directory) = &path_info.directory {
            println!("  Series Directory:");
            println!("    Title: {}", directory.title);
            println!("    Year: {:?}", directory.year);
        }
        
        if let Some(season) = path_info.season {
            println!("  Season: {}", season);
        }
        
        println!("  File:");
        println!("    Title: {}", path_info.file.title);
        println!("    Episode Title: {}", path_info.file.episode_title);
        println!("    Season: {:?}", path_info.file.season);
        println!("    Episode: {:?}", path_info.file.episode);
        println!("    Episodes: {:?}", path_info.file.episodes);
        println!("    Resolution: {}", path_info.file.resolution);
        println!("    Streaming Provider: {}", path_info.file.streaming_provider);
        println!("    Source: {}", path_info.file.source);
        println!("    HDR: {}", path_info.file.hdr);
        println!("    Audio: {}", path_info.file.audio);
        println!("    Format: {}", path_info.file.format);
        println!("    Group: {}", path_info.file.group);
    }

    println!("\n--- Path Parsing Example (Movie) ---");
    let movie_parser = ReleaseParser::new("movie");
    let movie_path = "/mnt/ttt/shows/The Movie Title (2010) {tmdb-1520211}/The Movie Title (2010) [imdbid-tt0106145] - {edition-Ultimate Extended Edition} [Surround Sound x264][Bluray-1080p Remux Proper][3D][DTS 5.1][DE][10bit][AVC]-RlsGrp.mkv";
    
    if let Some(path_info) = movie_parser.parse_path(movie_path) {
        println!("  Full Path: {}", path_info.full_path);
        
        if let Some(directory) = &path_info.directory {
            println!("  Movie Directory:");
            println!("    Title: {}", directory.title);
            println!("    Year: {:?}", directory.year);
            if let Some(tmdb_id) = &directory.tmdb_id {
                println!("    TMDB ID: {}", tmdb_id);
            }
        }
        
        println!("  File:");
        println!("    Title: {}", path_info.file.title);
        if let Some(year) = path_info.file.year {
            println!("    Year: Some({})", year);
        } else {
            println!("    Year: None");
        }
        if let Some(edition) = &path_info.file.edition {
            println!("    Edition: {}", edition);
        }
        if let Some(imdb_id) = &path_info.file.imdb_id {
            println!("    IMDB ID: {}", imdb_id);
        }
        println!("    Resolution: {}", path_info.file.resolution);
        println!("    Source: {}", path_info.file.source);
        println!("    HDR: {}", path_info.file.hdr);
        println!("    Audio: {}", path_info.file.audio);
        println!("    Format: {}", path_info.file.format);
        println!("    Flags: {:?}", path_info.file.flags);
        println!("    Languages: {:?}", path_info.file.language);
        println!("    Group: {}", path_info.file.group);
    }

    // Additional path parsing examples
    println!("\n--- Path Parsing Example (Movie with TMDB) ---");
    let movie_path2 = "/movies/Vanilla Sky (2001) {tmdb-1903}/Vanilla Sky (2001) {tmdb-1903} [Remux-2160p Proper][DV HDR10][DTS-HD MA 5.1][HEVC]-FraMeSToR.mkv";
    
    if let Some(path_info) = movie_parser.parse_path(movie_path2) {
        println!("  Full Path: {}", path_info.full_path);
        
        if let Some(directory) = &path_info.directory {
            println!("  Movie Directory:");
            println!("    Title: {}", directory.title);
            println!("    Year: {:?}", directory.year);
            if let Some(tmdb_id) = &directory.tmdb_id {
                println!("    TMDB ID: {}", tmdb_id);
            }
        }
        
        println!("  File:");
        println!("    Title: {}", path_info.file.title);
        println!("    Year: {:?}", path_info.file.year);
        if let Some(tmdb_id) = &path_info.file.tmdb_id {
            println!("    TMDB ID: {}", tmdb_id);
        }
        println!("    Resolution: {}", path_info.file.resolution);
        println!("    Source: {}", path_info.file.source);
        println!("    HDR: {}", path_info.file.hdr);
        println!("    Audio: {}", path_info.file.audio);
        println!("    Format: {}", path_info.file.format);
        println!("    Flags: {:?}", path_info.file.flags);
        println!("    Group: {}", path_info.file.group);
    }

    println!("\n--- Path Parsing Example (TV Show with TVDB) ---");
    let tv_path2 = "/tv/GBRB - Joy Pops Laugh Pops (2025) {tvdb-468780}/Season 01/GBRB - Joy Pops Laugh Pops (2025) - S01E09 - Episode 9 [WEBDL-1080p][AAC 2.0][h264]-JKCT.mkv";
    
    if let Some(path_info) = tv_parser.parse_path(tv_path2) {
        println!("  Full Path: {}", path_info.full_path);
        
        if let Some(directory) = &path_info.directory {
            println!("  Series Directory:");
            println!("    Title: {}", directory.title);
            println!("    Year: {:?}", directory.year);
            if let Some(tvdb_id) = &directory.tvdb_id {
                println!("    TVDB ID: {}", tvdb_id);
            }
        }
        
        if let Some(season) = path_info.season {
            println!("  Season: {}", season);
        }
        
        println!("  File:");
        println!("    Title: {}", path_info.file.title);
        println!("    Episode Title: {}", path_info.file.episode_title);
        println!("    Season: {:?}", path_info.file.season);
        println!("    Episode: {:?}", path_info.file.episode);
        println!("    Resolution: {}", path_info.file.resolution);
        println!("    Source: {}", path_info.file.source);
        println!("    Audio: {}", path_info.file.audio);
        println!("    Format: {}", path_info.file.format);
        println!("    Group: {}", path_info.file.group);
    }

    println!("\n--- Path Parsing Example (Windows Path) ---");
    let windows_path = r"C:\tv\GBRB - Joy Pops Laugh Pops (2025) {tvdb-468780}\Season 01\GBRB - Joy Pops Laugh Pops (2025) - S01E10 - Episode 10 [WEBDL-1080p][AAC 2.0][h264]-JKCT.mkv";
    
    if let Some(path_info) = tv_parser.parse_path(windows_path) {
        println!("  Full Path: {}", path_info.full_path);
        
        if let Some(directory) = &path_info.directory {
            println!("  Series Directory:");
            println!("    Title: {}", directory.title);
            println!("    Year: {:?}", directory.year);
            if let Some(tvdb_id) = &directory.tvdb_id {
                println!("    TVDB ID: {}", tvdb_id);
            }
        }
        
        if let Some(season) = path_info.season {
            println!("  Season: {}", season);
        }
        
        println!("  File:");
        println!("    Title: {}", path_info.file.title);
        println!("    Episode Title: {}", path_info.file.episode_title);
        println!("    Season: {:?}", path_info.file.season);
        println!("    Episode: {:?}", path_info.file.episode);
        println!("    Episodes: {:?}", path_info.file.episodes);
        println!("    Resolution: {}", path_info.file.resolution);
        println!("    Source: {}", path_info.file.source);
        println!("    Audio: {}", path_info.file.audio);
        println!("    Format: {}", path_info.file.format);
        println!("    Group: {}", path_info.file.group);
    }

    // Older format examples
    println!("\n--- Older Format Examples ---");
    
    println!("\n--- Example 1: Year in Square Brackets ---");
    let old_format1 = "Letters.From.Iwo.Jima[2006]DvDrip[Eng.Hard.Sub]-aXXo";
    let parsed1 = movie_parser.parse(old_format1);
    
    println!("  Release: {}", parsed1.release);
    println!("  Title: {}", parsed1.title);
    println!("  Year: {:?}", parsed1.year);
    println!("  Source: {}", parsed1.source);
    println!("  Flags: {:?}", parsed1.flags);
    println!("  Languages: {:?}", parsed1.language);
    println!("  Group: {}", parsed1.group);

    println!("\n--- Example 2: U-Edition ---");
    let old_format2 = "You.Dont.Mess.With.The.Zohan[2008][U-Edition]DvDrip-aXXo";
    let parsed2 = movie_parser.parse(old_format2);
    
    println!("  Release: {}", parsed2.release);
    println!("  Title: {}", parsed2.title);
    println!("  Year: {:?}", parsed2.year);
    if let Some(edition) = &parsed2.edition {
        println!("  Edition: {}", edition);
    }
    println!("  Source: {}", parsed2.source);
    println!("  Group: {}", parsed2.group);

    println!("\n--- Example 3: Standard Old Format ---");
    let old_format3 = "Zero.Man.vs.The.Half.Virgin.2012.DVDRip.x264.AC3.WahDee";
    let parsed3 = movie_parser.parse(old_format3);
    
    println!("  Release: {}", parsed3.release);
    println!("  Title: {}", parsed3.title);
    println!("  Year: {:?}", parsed3.year);
    println!("  Source: {}", parsed3.source);
    println!("  Format: {}", parsed3.format);
    println!("  Audio: {}", parsed3.audio);
    println!("  Group: {}", parsed3.group);

    // Streaming provider examples
    println!("\n--- Streaming Provider Examples ---");
    
    println!("\n--- Example 1: SKST Provider ---");
    let streaming1 = "The.Christmas.Doctor.2020.NORDiC.1080p.SKST.WEB-DL.H.264-NORViNE";
    let parsed1 = movie_parser.parse(streaming1);
    
    println!("  Release: {}", parsed1.release);
    println!("  Title: {}", parsed1.title);
    println!("  Year: {:?}", parsed1.year);
    println!("  Resolution: {}", parsed1.resolution);
    println!("  Streaming Provider: {}", parsed1.streaming_provider);
    println!("  Source: {}", parsed1.source);
    println!("  Format: {}", parsed1.format);
    println!("  Languages: {:?}", parsed1.language);
    println!("  Group: {}", parsed1.group);

    println!("\n--- Example 2: MAX Provider (TV Show) ---");
    let streaming2 = "Gransbevakarna.Sverige.S06E01.SWEDiSH.1080p.MAX.WEB-DL.H.265-VARiOUS";
    let parsed2 = parser.parse(streaming2);
    
    println!("  Release: {}", parsed2.release);
    println!("  Title: {}", parsed2.title);
    println!("  Season: {:?}", parsed2.season);
    println!("  Episode: {:?}", parsed2.episode);
    println!("  Resolution: {}", parsed2.resolution);
    println!("  Streaming Provider: {}", parsed2.streaming_provider);
    println!("  Source: {}", parsed2.source);
    println!("  Format: {}", parsed2.format);
    println!("  Languages: {:?}", parsed2.language);
    println!("  Group: {}", parsed2.group);

    println!("\n--- Example 3: TV2 Provider ---");
    let streaming3 = "Hotellet.S01E17.NORWEGiAN.1080p.TV2.WEB-DL.H.264-NORViNE";
    let parsed3 = parser.parse(streaming3);
    
    println!("  Release: {}", parsed3.release);
    println!("  Title: {}", parsed3.title);
    println!("  Season: {:?}", parsed3.season);
    println!("  Episode: {:?}", parsed3.episode);
    println!("  Resolution: {}", parsed3.resolution);
    println!("  Streaming Provider: {}", parsed3.streaming_provider);
    println!("  Source: {}", parsed3.source);
    println!("  Format: {}", parsed3.format);
    println!("  Languages: {:?}", parsed3.language);
    println!("  Group: {}", parsed3.group);

    // Anime and special format examples
    println!("\n--- Anime and Special Format Examples ---");
    
    println!("\n--- Example 1: German Movie with ANiME-style naming ---");
    let anime1 = "Kinder.des.Zorns.Runaway.2018.German.DL.1080P.BluRay.AVC-MRW";
    let parsed1 = movie_parser.parse(anime1);
    
    println!("  Release: {}", parsed1.release);
    println!("  Title: {}", parsed1.title);
    println!("  Year: {:?}", parsed1.year);
    println!("  Resolution: {}", parsed1.resolution);
    println!("  Source: {}", parsed1.source);
    println!("  Format: {}", parsed1.format);
    println!("  Languages: {:?}", parsed1.language);
    println!("  Group: {}", parsed1.group);

    println!("\n--- Example 2: ANiME TV Show ---");
    let anime2 = "Ranma.1.2.2024.S02E11.GERMAN.ANiME.WEBRiP.x264-AVTOMAT";
    let parsed2 = parser.parse(anime2);
    
    println!("  Release: {}", parsed2.release);
    println!("  Title: {}", parsed2.title);
    println!("  Season: {:?}", parsed2.season);
    println!("  Episode: {:?}", parsed2.episode);
    println!("  Year: {:?}", parsed2.year);
    println!("  Resolution: {}", parsed2.resolution);
    println!("  Source: {}", parsed2.source);
    println!("  Format: {}", parsed2.format);
    println!("  Languages: {:?}", parsed2.language);
    println!("  Flags: {:?}", parsed2.flags);
    println!("  Group: {}", parsed2.group);

    println!("\n--- Example 3: ANiME TV Show with Episode Title ---");
    let anime3 = "Kizuna.no.Allele.S02E10.Unsere.unbekannte.Groesse.German.2023.ANiME.DL.1080p.BluRay.x264-STARS";
    let parsed3 = parser.parse(anime3);
    
    println!("  Release: {}", parsed3.release);
    println!("  Title: {}", parsed3.title);
    println!("  Episode Title: {}", parsed3.episode_title);
    println!("  Season: {:?}", parsed3.season);
    println!("  Episode: {:?}", parsed3.episode);
    println!("  Year: {:?}", parsed3.year);
    println!("  Resolution: {}", parsed3.resolution);
    println!("  Source: {}", parsed3.source);
    println!("  Format: {}", parsed3.format);
    println!("  Languages: {:?}", parsed3.language);
    println!("  Flags: {:?}", parsed3.flags);
    println!("  Group: {}", parsed3.group);

    println!("\n--- Example 4: Chinese Anime Format with Brackets ---");
    let anime4 = "[GM-Team][国漫][仙逆][Renegade Immortal][2023][119][AVC][GB][1080P]";
    let parsed4 = parser.parse(anime4);
    
    println!("  Release: {}", parsed4.release);
    println!("  Title: {}", parsed4.title);
    println!("  Year: {:?}", parsed4.year);
    println!("  Season: {:?}", parsed4.season);
    println!("  Episode: {:?}", parsed4.episode);
    println!("  Resolution: {}", parsed4.resolution);
    println!("  Format: {}", parsed4.format);
    println!("  Languages: {:?}", parsed4.language);
    println!("  Group: {}", parsed4.group);

    println!("\n--- Example 5: Anime with Country Code and MultiSub ---");
    let anime5 = "[Erai-raws] Xian Wang de Richang Shenghuo 5 - 01 (CA) [720p CR WEB-DL AVC AAC][MultiSub][2B267646]";
    let parsed5 = parser.parse(anime5);
    
    println!("  Release: {}", parsed5.release);
    println!("  Title: {}", parsed5.title);
    println!("  Season: {:?}", parsed5.season);
    println!("  Episode: {:?}", parsed5.episode);
    println!("  Resolution: {}", parsed5.resolution);
    println!("  Streaming Provider: {}", parsed5.streaming_provider);
    println!("  Source: {}", parsed5.source);
    println!("  Format: {}", parsed5.format);
    println!("  Audio: {}", parsed5.audio);
    println!("  Languages: {:?}", parsed5.language);
    println!("  Flags: {:?}", parsed5.flags);
    println!("  Group: {}", parsed5.group);

    println!("\n--- Example 6: Anime with NF Provider and Multi-Subs ---");
    let anime6 = "[ToonsHub] Pray Speak What Has Happened S01E09 1080p NF WEB-DL AAC2.0 H.264 (Multi-Subs, Moshimo Kono Yo ga Butai nara, Gakuya wa Doko ni Aru Darou)";
    let parsed6 = parser.parse(anime6);
    
    println!("  Release: {}", parsed6.release);
    println!("  Title: {}", parsed6.title);
    println!("  Season: {:?}", parsed6.season);
    println!("  Episode: {:?}", parsed6.episode);
    println!("  Resolution: {}", parsed6.resolution);
    println!("  Streaming Provider: {}", parsed6.streaming_provider);
    println!("  Source: {}", parsed6.source);
    println!("  Format: {}", parsed6.format);
    println!("  Audio: {}", parsed6.audio);
    println!("  Languages: {:?}", parsed6.language);
    println!("  Flags: {:?}", parsed6.flags);
    println!("  Group: {}", parsed6.group);

    println!("\n--- Example 7: SubsPlease Anime Format ---");
    let anime7 = "[SubsPlease] The Daily Life of the Immortal King S5 - 02 (1080p) [66856162].mkv";
    let parsed7 = parser.parse(anime7);
    
    println!("  Release: {}", parsed7.release);
    println!("  Title: {}", parsed7.title);
    println!("  Season: {:?}", parsed7.season);
    println!("  Episode: {:?}", parsed7.episode);
    println!("  Resolution: {}", parsed7.resolution);
    println!("  Group: {}", parsed7.group);

    // Additional examples
    println!("\n--- Example 8: TV Show with High Season Number ---");
    let tv_high_season = "Pinoy Big Brother Celebrity Collab Edition S13E44 1080p WEB-DL AAC x264-RSG";
    let parsed8 = parser.parse(tv_high_season);
    
    println!("  Release: {}", parsed8.release);
    println!("  Title: {}", parsed8.title);
    println!("  Season: {:?}", parsed8.season);
    println!("  Episode: {:?}", parsed8.episode);
    println!("  Resolution: {}", parsed8.resolution);
    println!("  Source: {}", parsed8.source);
    println!("  Audio: {}", parsed8.audio);
    println!("  Format: {}", parsed8.format);
    println!("  Group: {}", parsed8.group);

    println!("\n--- Example 9: Movie with WAVVE Streaming Provider ---");
    let wavve_movie = "Doctor (2012) 1080p WAVVE WEB-DL AAC H.264-GNom";
    let parsed9 = movie_parser.parse(wavve_movie);
    
    println!("  Release: {}", parsed9.release);
    println!("  Title: {}", parsed9.title);
    println!("  Year: {:?}", parsed9.year);
    println!("  Resolution: {}", parsed9.resolution);
    println!("  Streaming Provider: {}", parsed9.streaming_provider);
    println!("  Source: {}", parsed9.source);
    println!("  Audio: {}", parsed9.audio);
    println!("  Format: {}", parsed9.format);
    println!("  Group: {}", parsed9.group);

    println!("\n--- Example 10: Movie with DDP Audio Format ---");
    let ddp_movie1 = "Sharks.of.the.Corn.2021.1080p.AMZN.WEB-DL.DDP2.0.H.264-SQS";
    let parsed10 = movie_parser.parse(ddp_movie1);
    
    println!("  Release: {}", parsed10.release);
    println!("  Title: {}", parsed10.title);
    println!("  Year: {:?}", parsed10.year);
    println!("  Resolution: {}", parsed10.resolution);
    println!("  Streaming Provider: {}", parsed10.streaming_provider);
    println!("  Source: {}", parsed10.source);
    println!("  Audio: {}", parsed10.audio);
    println!("  Format: {}", parsed10.format);
    println!("  Group: {}", parsed10.group);

    println!("\n--- Example 11: Movie with DDP5.1 Audio ---");
    let ddp_movie2 = "Detective Kien: The Headless Horror 2025 1080p AMZN WEB-DL DDP5.1 H.264-playWEB";
    let parsed11 = movie_parser.parse(ddp_movie2);
    
    println!("  Release: {}", parsed11.release);
    println!("  Title: {}", parsed11.title);
    println!("  Year: {:?}", parsed11.year);
    println!("  Resolution: {}", parsed11.resolution);
    println!("  Streaming Provider: {}", parsed11.streaming_provider);
    println!("  Source: {}", parsed11.source);
    println!("  Audio: {}", parsed11.audio);
    println!("  Format: {}", parsed11.format);
    println!("  Group: {}", parsed11.group);

    println!("\n--- Example 12: TV Show with Group in Brackets ---");
    let tv_bracket_group = "Digimon Beatbreak (2025) S01E11 (1080p CR WEB-DL H264 AAC 2.0) [AnoZu]";
    let parsed12 = parser.parse(tv_bracket_group);
    
    println!("  Release: {}", parsed12.release);
    println!("  Title: {}", parsed12.title);
    println!("  Year: {:?}", parsed12.year);
    println!("  Season: {:?}", parsed12.season);
    println!("  Episode: {:?}", parsed12.episode);
    println!("  Resolution: {}", parsed12.resolution);
    println!("  Streaming Provider: {}", parsed12.streaming_provider);
    println!("  Source: {}", parsed12.source);
    println!("  Format: {}", parsed12.format);
    println!("  Audio: {}", parsed12.audio);
    println!("  Group: {}", parsed12.group);

    println!("\n--- Example 13: Episode-Only Format with Group in Brackets ---");
    let episode_only = "[FSP DN] Tales of Herding Gods Episode 61 1080p HEVC AAC";
    let parsed13 = parser.parse(episode_only);
    
    println!("  Release: {}", parsed13.release);
    println!("  Title: {}", parsed13.title);
    println!("  Season: {:?}", parsed13.season);
    println!("  Episode: {:?}", parsed13.episode);
    println!("  Episodes: {:?}", parsed13.episodes);
    println!("  Resolution: {}", parsed13.resolution);
    println!("  Format: {}", parsed13.format);
    println!("  Audio: {}", parsed13.audio);
    println!("  Group: {}", parsed13.group);

    println!("\n--- Example 14: Episode-Only Format E01E02 ---");
    let e01e02 = "Mondo.Senza.Fine.E01E02.iTALiAN.HDTV.x264-HWD";
    let parsed14 = parser.parse(e01e02);
    
    println!("  Release: {}", parsed14.release);
    println!("  Title: {}", parsed14.title);
    println!("  Season: {:?}", parsed14.season);
    println!("  Episode: {:?}", parsed14.episode);
    println!("  Episodes: {:?}", parsed14.episodes);
    println!("  Source: {}", parsed14.source);
    println!("  Format: {}", parsed14.format);
    println!("  Languages: {:?}", parsed14.language);
    println!("  Group: {}", parsed14.group);

    println!("\n--- Example 15: The Simpsons with MULTI ---");
    let simpsons = "The.Simpsons.S37E05.MULTI.1080p.WEB.H264-HiggsBoson";
    let parsed15 = parser.parse(simpsons);
    
    println!("  Release: {}", parsed15.release);
    println!("  Title: {}", parsed15.title);
    println!("  Season: {:?}", parsed15.season);
    println!("  Episode: {:?}", parsed15.episode);
    println!("  Episodes: {:?}", parsed15.episodes);
    println!("  Resolution: {}", parsed15.resolution);
    println!("  Source: {}", parsed15.source);
    println!("  Format: {}", parsed15.format);
    println!("  Languages: {:?}", parsed15.language);
    println!("  Group: {}", parsed15.group);

    println!("\n--- Example 16: Running Man E780 (Episode-Only) ---");
    let running_man = "Running Man E780 1080p VIU WEB-DL AAC 2.0 H.264-MMR";
    let parsed16 = parser.parse(running_man);
    
    println!("  Release: {}", parsed16.release);
    println!("  Title: {}", parsed16.title);
    println!("  Season: {:?}", parsed16.season);
    println!("  Episode: {:?}", parsed16.episode);
    println!("  Resolution: {}", parsed16.resolution);
    println!("  Streaming Provider: {}", parsed16.streaming_provider);
    println!("  Source: {}", parsed16.source);
    println!("  Audio: {}", parsed16.audio);
    println!("  Format: {}", parsed16.format);
    println!("  Group: {}", parsed16.group);

    println!("\n--- Example 17: Running Man E780 with Episode Title ---");
    let running_man_title = "Running.Man.E780.This.is.the.Romance.of.It.Continues.1080p.VIU.WEB-DL.H264.AAC-MMR";
    let parsed17 = parser.parse(running_man_title);
    
    println!("  Release: {}", parsed17.release);
    println!("  Title: {}", parsed17.title);
    println!("  Season: {:?}", parsed17.season);
    println!("  Episode: {:?}", parsed17.episode);
    println!("  Episode Title: {}", parsed17.episode_title);
    println!("  Resolution: {}", parsed17.resolution);
    println!("  Streaming Provider: {}", parsed17.streaming_provider);
    println!("  Source: {}", parsed17.source);
    println!("  Format: {}", parsed17.format);
    println!("  Audio: {}", parsed17.audio);
    println!("  Group: {}", parsed17.group);
}

