import os
import sys
from colorama import Fore

pkg = ""
ver = 0
pkgs = open("/usr/share/elements/pkgs", 'a')


def pkgscheck():
    if os.path.exists("/usr/share/elements/pkgs"):
        pass
    else:
        file = open("/usr/share/elements/pkgs", 'w')
        file.close()


def install_pkg():
    # Package Check 2
    pkgvalid = os.system("ls ~/.lmt-repo/ | grep " + pkg + "> /dev/null")
    if pkgvalid == 256:
        print(Fore.RED + pkg + " is not a valid package." + Fore.WHITE)
    else:
        print("Are you sure you want to install " + pkg + "?")

        def prompt():
            x = str(input(Fore.GREEN + "Y" + Fore.WHITE + "/" + Fore.RED + "n" + ' ' + Fore.WHITE))
            if x in ['y']:
                os.system("bash ~/.lmt-repo/" + pkg + "/build")
                print("----------------------------")
                print("Installed " + pkg + " successfully")
                pkgs.write(" " + pkg)
            elif x in ['n']:
                sys.exit()
            elif x in ['cry']:
                print(";-;")
                prompt()
            else:
                print(Fore.RED + '"' + x + '"' + " is not a valid command." + Fore.WHITE)
                prompt()

        prompt()
