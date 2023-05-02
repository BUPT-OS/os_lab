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

class Rule:
    def __init__(self,regex,line_limit=-1):
        self.regex = regex
        self.line_limit = line_limit
    def match(self,s:str) ->bool:
        return self.regex.match(s)
    def consume(self) ->bool:
        # 检查是否耗尽行数控制
        if self.line_limit < 0:
            return False
        else:
            self.line_limit -= 1
            return self.line_limit == 0
rules = [
    Rule(re.compile(r"error\[[A-Za-z0-9]*\]:.*")),# rust error
    Rule(re.compile(r"error:.*")),                # rust error
    Rule(re.compile(r'.*\.c:\d+:\d+: error:.*'),4), # rust/helper error
    Rule(re.compile(r'ld.lld: error: .*'))        # ld error
]
end_rules = [
    re.compile("\n"),
    re.compile(".*error generated.")
]
print_error = False
cur_rule = None
with open("compile.txt", "r") as file:  # open the file "compile.txt" in read mode
    for line in file:  # iterate through each line in the file
        if not print_error:
            for rule in rules:
                if rule.match(line):
                    print_error = True
                    cur_rule = rule
                    break
        if print_error:
            print(line,end='')
            if cur_rule.consume():
                print_error = False
                cur_rule = None
                continue
            for rule in end_rules:
                if rule.match(line):
                    print_error = False
                    cur_rule = None
                    break
print("compile failed")
import os
os._exit(-1)