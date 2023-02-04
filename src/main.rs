use std::{env, io};

use std::io::{stdout, Write};
use nix::unistd::getuid;
use crate::imut_api::enterrw;
use crate::vpl::{add_pkg_to_db, compare_old_to_new, debug_add_pkg_to_pkglist, install_tar, list_packages, download_pkglist, remove_tar, search_package};

mod vpl; // import VPLIB
mod imut_api; // Immutability API

fn main() {
    // vpl::download_pkglist();
    //
    // install_tar("neofetch", "", false, false);
    //
    // remove_tar("neofetch");
    //
    //
    // if list_packages().contains("neofetch") {
    //     println!("Package was not removed");
    // } else {
    //     println!("Package was removed");
    // }
    //
    // return;
    let mut args_mod: Vec<String> = env::args().collect(); // args_mod that can be modified
    let imut_args: Vec<String> = env::args().collect(); // immutable args_mod for other things

    if imut_args.len() >= 2 {
        let command = &imut_args[1].to_lowercase();

        if command == "install"
            || command == "in"
            || command == "remove"
            || command == "rm"
            || command == "upgrade"
            || command == "up"
            || command == "search"
            || command == "se"
        {
            if vpl::check_option("snapshots") {
                vpl::new_snapshot("pre", &imut_args[1]);
            }

            if !getuid().to_string().eq("0") {
                println!("You must be root to use this command!");
                std::process::exit(1);
            }

        } else if command.eq("help") || command.eq( "help") {
            help(0);
        } else {
            println!("Invalid operation: {}", command);
            std::process::exit(1);
        }
    } else {
        println!("At least one 2 arguments are required(1 found)");
        std::process::exit(1);
    }

    let command = &imut_args[1].to_lowercase(); // redeclare in main

    if imut_args.len() >= 3 {
        for i in 2..args_mod.len() {
            if args_mod[i].is_empty() {
                // Throw error if "" is passed as argument
                println!("Error: Unknown error.");
                std::process::exit(512); // Error 512 for invalid arguments
            }

            if args_mod[i].contains(' ') {
                // Throw error if package name contains space
                println!("Error: Package name cannot be empty.");
                std::process::exit(512);
            }

            if args_mod[i].contains('.') || args_mod[i].contains('/') {
                println!("Error: Package name cannot contain '{}'", args_mod[i]);
                std::process::exit(512);
            }

            if !vpl::check_option("remove_protected")
                && command.eq("remove")
                && [
                "elements",
                "gnome-core",
                "gnome",
                "linux",
                "xbps",
                "mutter",
                "kernel",
            ] // kernel - nitrogen os's kernel
                .contains(&&*args_mod[i])
            {
                println!(
                    "Cannot remove '{0}': Package is required by system.",
                    args_mod[i]
                );
                std::process::exit(128);
            }

			if vpl::get_pkg_version(args_mod[i].as_str()).is_empty() {
            	println!("Couldn't find package {} in repository", args_mod[i])
            }
        }
    } else if command == "upgrade" || command == "up" {
        let mut prompt = true;
        while prompt {
            print!("Are you sure you want to upgrade all packages? [Y/n] ");
            stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" || input.is_empty() {
                prompt = false;
            } else if input == "n" || input == "no" {
                std::process::exit(0);
            } else {
                println!("Invalid input: {}", input);
            }
        }

        vpl::upgrade_system();
    } else {
        println!("At least one 3 arguments are required(2 found)");
        std::process::exit(1);
    }

    args_mod.remove(0);
    args_mod.remove(0); // remove non-important arguments(will be saved in imut_args)

    if imut_args[2].eq("search") {
        if search_package(&args_mod[1]) {
            println!(
                "Package: {0} was found in Elements' repository.",
                &args_mod[0]
            );
            println!("Use 'lmt install {0}' to install it.", &args_mod[0])
        } else {
            println!("Couldn't find '{0}' .", &args_mod[0])
        }
    }

    args_mod.dedup(); // remove duplicates

    if command.eq("install") || command.eq("in") && args_mod.len() == 1 {
        println!("Installing {0:?}", args_mod.join(" "));
    } else if command.eq("remove") || command.eq("rm") && args_mod.len() == 1 {
        println!("Removing: {0:?}", args_mod.join(" "));
    } else if command.eq("upgrade") || command.eq("up") && args_mod.len() == 1 {
        println!("Upgrading: {0:?}", args_mod.join(" "));
    } else if command.eq("install") || command.eq("in") && args_mod.len() != 1 {
        println!(
            "Installing {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    } else if command.eq("remove") || command.eq("rm") && args_mod.len() != 1 {
        println!(
            "Removing {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    } else if command.eq("upgrade") || command.eq("up") && args_mod.len() != 1 {
        println!(
            "Upgrading {0} packages: {1:?}",
            args_mod.len(),
            args_mod.join(" ")
        );
    }

    let mut in_prompt = true;

    while in_prompt {
        print!("Continue? [Y/n] ");
        stdout().flush().unwrap(); // flush stdout

        let mut input = String::new(); // answer to the "Continue?" prompt
        io::stdin().read_line(&mut input).unwrap(); // take input
        input = input.to_lowercase();

        if input.eq("n\n") || input.eq("no\n") {
            // if answer is "n" or "no"
            println!("Aborting.");
            std::process::exit(0);
        } else if !input.eq("y\n") && !input.eq("yes\n") && !input.eq("\n") {
            // if answer is neither "y" nor "yes" nor nothing
            println!("Input Error: Unknown answer.")
        } else {
            in_prompt = false;
        }
    }

    let mut pkgs_done = 0;

	let orig_mode = imut_api::getmode();
  
    if orig_mode {
        enterrw();
    }
    
    while pkgs_done < args_mod.len() {
        if command.eq("install") || command.eq("in") {
            println!(
                "Install package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if install_tar(&args_mod[pkgs_done], "", false, false) == 128 {
                println!("Package already installed. Skipping...");
            };
        } else if command.eq("remove") || command.eq("rm") {
            println!(
                "Removing package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if remove_tar(&args_mod[pkgs_done]) == 128 {
                println!("Package not installed. Skipping...");
            };
        } else if command.eq("upgrade") || command.eq("up") {
            println!(
                "Updating package: {0} {1}/{2}",
                &args_mod[pkgs_done],
                pkgs_done + 1,
                args_mod.len()
            );
            if install_tar(&args_mod[pkgs_done], "", false, true) == 128 {
                println!("Package not installed. Skipping...");
            };
        }

        pkgs_done += 1;

    }

	if orig_mode {
        imut_api::enterro();
    }
}

fn help(exit_code: i32) {
    println!("usage: vpt <action> <package>");
    println!("Use 'man vpt' to check all available commands");
    std::process::exit(exit_code);
}
