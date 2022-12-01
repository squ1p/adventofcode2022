#!/usr/bin/env python3
#coding: utf-8


with open("input.txt", "r") as inp:
    inp = inp.read()


inp = inp.split("\n\n")
inp.pop(-1)


max_cal = 0

values = []

for elf in inp:
    elf = elf.split("\n")
    total = 0
    for food in elf:
        total += int(food)
    values.append(total)

values.sort(reverse=True)
aaa = sum(values[0:3])
print(f"{aaa}")

print(f"Winner {max_cal}")
