# Description: Advent of Code Day 2 Problem 1
levels =[]
with open(r"C:\Users\Shawf\OneDrive\Desktop\test.txt","r") as reader:
    reports = reader.readlines()
    
for i in reports:
    temp = i.split(" ")
    for j in range(len(temp)):
        temp[j] = int(temp[(j)])
    levels.append(temp)

def check_increase(levels):
    count = 0
    for i in levels:
        flag = False
        for j in range(len(i)-1):
            if i[j] < i[j+1] and i[j+1] - i[j] >= 1 and i[j+1] - i[j] <= 3:
                flag = True
            else:
                flag = False
                break
        if(flag):
            count += 1
            print(i)
    return count

def check_decrease(levels):
    count = 0
    for i in levels:
        flag = False
        for j in range(len(i)-1):
            if i[j] > i[j+1] and i[j] - i[j+1] >= 1 and i[j] - i[j+1] <= 3:
                flag = True
            else:
                flag = False
                break
        if(flag):
            count += 1
            print(i)
    return count
print(check_increase(levels) + check_decrease(levels))

#Advent of Code Day 2 Problem 2
# Description: Advent of Code Day 2 Problem 1
FILENAME = "test.txt"
MIN_DIFFERENCE = 1
MAX_DIFFERENCE = 3

def parse_input(data):
    return [[int(num) for num in row.split()] for row in data]

def is_safe(level):
    differences = [second - first for first, second in zip(level, level[1:])]
    if any(not (MIN_DIFFERENCE <= abs(diff) <= MAX_DIFFERENCE) for diff in differences):
        return False
    trends = {diff // abs(diff) for diff in differences}
    return len(trends) == 1

def is_safe_with_removal(level):
    for i, num in enumerate(level):
        new_level = level[:i] + level[i + 1 :]
        if is_safe(new_level):
            return True
    return False

def check_increase_damped(levels):
    count = 0
    for level in levels:
        if is_safe(level) or is_safe_with_removal(level):
            count += 1
    return count

def check_decrease_damped(levels):
    count = 0
    for level in levels:
        if is_safe(level) or is_safe_with_removal(level):
            count += 1
    return count

def solve(levels):
    return check_increase_damped(levels) + check_decrease_damped(levels)


print(solve(levels))
