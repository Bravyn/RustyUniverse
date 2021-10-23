import os
global logfile

def logfile():
    print("Creating a logfile")
    try:
        if not "logfile.txt" in os.listdir():
            logfile = open("logfile.txt", "wt")
        print("Logfile already exists") 
        logfile = open("logfile.txt", "wt")
        return logfile
    except:
        print("Error creating logfile.txt")
    
logfile = logfile()

def hello():
    if any(name.endswith('.rs') for 
    name in os.listdir()):
        print("Rust source files present in this directory.", file=logfile)
        print("Here they are: {}".format([i for i in os.listdir() if i.endswith(".rs")]), file=logfile)
        print("Opening...", file=logfile)
        try:
            rs = open("main.rs","r")
        except:
            print("Could  not open", file=logfile)
        print("------reading file------")
        try:
            data = rs.read()
            rs.close()
        except:
            print("Could not read the file")
        
        
        print("File contents: ", data[:9])
        for i, s in enumerate(data[:]):
            if i == "b":
                print(i)
                
hello()