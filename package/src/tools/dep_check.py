import os

def check():
    does_file_exist("/usr/bin/blkid")
    

    #more deps...

def does_file_exist(name):
    if(os.path.exists(name)):
        print("[*] Dependency {} found!".format(name))
    else:
        print("[!] Dependency {} missing.".format(name))
        print("[!] Exiting with code -1")
        exit(-1)


