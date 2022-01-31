from diskutil import blk_dev
from tools import dep_check
from ui import gtk_main


def main():
    print("seed - The AcaicaLinux Installer...")
    print("")
    print("[*] Checking for dependencies...")    
    dep_check.check()

    print("[*] Dependency check completed.")
    print("[*] Launching Window...")
    gtk_main.init()

    print("[*] Exiting with code 0.")
    exit(0)
    

if(__name__ == "__main__"):
    main()
