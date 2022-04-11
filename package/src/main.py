# Main file
# Author: zimsneexh
# Copyright (c) zimsneexh (The AcaciaLinux project), 2022

from diskutil import blk_dev
from tools import dep_check
from ui import gtk_main
import os
import sys

def main():
    print("seed - the AcaicaLinux installer...")
    print("")
    
    dep_check_skip = False
    args = len(sys.argv)

    if(args == 2):
        # arguments
        if("--nodeps" in sys.argv):
            dep_check_skip = True


    if(dep_check_skip):
        print("[*] Skipped dependency check..")
    else:
        print("[*] Checking for dependencies...")    
        dep_check.check()
        print("[*] Dependency check completed.")

    

    print("[*] Exiting with code 0.")
    exit(0)
    

if(__name__ == "__main__"):
    main()
