#Challenge 1
list1 = []
list2 = []
sum = 0
with open(r"<path>", "r") as reader:
    temp = reader.readlines()
    for i in range(len(temp)):
        str = temp[i].split(" ")
        list1.append(int(str[0]))
        list2.append(int(str[3]))
list1 = sorted(list1)
list2 = sorted(list2)

for i in range(len(list1)):
    sum += Math.abs(list1[i] - list2[i])
print(sum)

#Challenge 2
list1 = []
list2 = []
sum = 0
with open(r"<path>", "r") as reader:
    temp = reader.readlines()
    for i in range(len(temp)):
        str = temp[i].split(" ")
        list1.append(int(str[0]))
        list2.append(int(str[3]))
list1 = sorted(list1)
list2 = sorted(list2)
freq = {}
for i in range(len(list1)):
    if list1[i] in freq:
        print("Already in dictionary", list1[i])
        continue
    else:
        freq[list1[i]] = 0
    for j in range(len(list2)):
        if list2[j] == list1[i]:
            freq[list1[i]] += 1
            print("Incrementing", list1[i])
for val in list1:
    print(val, freq[val])
    sum += freq[val]*val
print(sum)
