import re
class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

rules = [
     re.compile(r"error\[[A-Za-z0-9]*\]:.*"),
    re.compile(r"error:.*"),
    re.compile(r'.*\.c:\d+:\d+: error:.*')
]
end_rules = [
    re.compile("\n"),
    re.compile(".*error generated.")
]
print_error = False
with open("compile.txt", "r") as file:  # open the file "compile.txt" in read mode
    for line in file:  # iterate through each line in the file
        if not print_error:
            for rule in rules:
                if rule.match(line):
                    print_error = True
                    break
        if print_error:
            print(line,end='')
            for rule in end_rules:
                if rule.match(line):
                    print_error = False
                    break