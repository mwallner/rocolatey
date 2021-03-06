extern crate clap;
use clap::{App, Arg, SubCommand};

use rocolatey_lib::roco::local::{
    get_local_bad_packages_text, get_local_packages_text, get_sources_text,
};
use rocolatey_lib::roco::remote::{get_outdated_packages, update_package_index};

#[tokio::main]
async fn main() {
    let matches = App::new("Rocolatey")
        .version("0.5.4")
        .author("Manfred Wallner <schusterfredl@mwallner.net>")
        .about("provides a basic interface for rocolatey-lib")
        .subcommand(
            SubCommand::with_name("list")
                .about("list local installed packages")
                .arg(
                    Arg::with_name("limitoutput")
                        .short("r")
                        .long("limitoutput")
                        .help("limit the output to essential information"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("be verbose"),
                ),
        )
        .subcommand(
            SubCommand::with_name("bad")
                .about("list packages in lib-bad/")
                .arg(
                    Arg::with_name("limitoutput")
                        .short("r")
                        .long("limitoutput")
                        .help("limit the output to essential information"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("be verbose"),
                ),
        )
        .subcommand(
            SubCommand::with_name("outdated")
                .about("Returns a list of outdated packages.")
                .arg(
                    Arg::with_name("ignore-pinned")
                        .long("ignore-pinned")
                        .help("ignore any pinned packages"),
                )
                .arg(
                    Arg::with_name("ignore-unfound")
                        .long("ignore-unfound")
                        .help("ignore any unfound packages"),
                )
                .arg(
                    Arg::with_name("limitoutput")
                        .short("r")
                        .long("limitoutput")
                        .help("limit the output to essential information"),
                )
                .arg(
                    Arg::with_name("prerelease")
                        .short("p")
                        .long("pre")
                        .help("include prerelease versions"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("be verbose"),
                ),
        )
        .subcommand(
            SubCommand::with_name("source")
                .about("list choco sources")
                .arg(
                    Arg::with_name("limitoutput")
                        .short("r")
                        .long("limitoutput")
                        .help("limit the output to essential information"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("be verbose"),
                ),
        )
        /*.subcommand(
            SubCommand::with_name("index")
                .about("crate package index")
                .arg(
                    Arg::with_name("limitoutput")
                        .short("r")
                        .help("limit the output to essential information"),
                )
                .arg(
                    Arg::with_name("prerelease")
                        .short("pre")
                        .help("include prerelease versions"),
                ),
        )*/
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        rocolatey_lib::set_verbose_mode(matches.is_present("verbose"));
        let r = matches.is_present("limitoutput");
        print!("{}", get_local_packages_text(r));
    } else if let Some(matches) = matches.subcommand_matches("bad") {
        rocolatey_lib::set_verbose_mode(matches.is_present("verbose"));
        let r = matches.is_present("limitoutput");
        print!("{}", get_local_bad_packages_text(r));
    } else if let Some(matches) = matches.subcommand_matches("index") {
        rocolatey_lib::set_verbose_mode(matches.is_present("verbose"));
        let r = matches.is_present("limitoutput");
        let pre = matches.is_present("prerelease");
        println!("{}", update_package_index(r, pre).await);
    } else if let Some(matches) = matches.subcommand_matches("outdated") {
        rocolatey_lib::set_verbose_mode(matches.is_present("verbose"));
        let r = matches.is_present("limitoutput");
        let pre = matches.is_present("prerelease");
        let ignore_pinned = matches.is_present("ignore-pinned");
        let ignore_unfound = matches.is_present("ignore-unfound");
        print!("{}", get_outdated_packages(r, pre, ignore_pinned, ignore_unfound).await);
    } else if let Some(matches) = matches.subcommand_matches("source") {
        rocolatey_lib::set_verbose_mode(matches.is_present("verbose"));
        let r = matches.is_present("limitoutput");
        print!("{}", get_sources_text(r));
    }
}
