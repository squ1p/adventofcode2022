#!/usr/bin/env python3
#coding: utf-8
from string import ascii_uppercase
import re

with open("input.txt") as inp:
    inp = inp.read()
inp = inp.splitlines()

#Reading the drawing
#drawing = inp[0:8]
#orders = inp[10:]

drawing = inp[:8]
orders = inp[10:]
stacks = {}
letters = list(ascii_uppercase)

#Initialize our dict of lists
for i in range(1, 10):
    stacks[i] = []

s = 1
for line in drawing:
    for char in line:
        if char in letters: 
            stacks[s].append(char)
            s += 1
        elif char == "!":
            s += 1
    s = 1

print(f"Starting with : ")
for stack in stacks:
    stacks[stack].reverse()
    print(f"{stack} ==> {stacks[stack]}")

processed_orders = []
#We got a stack, now read the orders
print("Processing orders")
for line in orders:
    #Getting the numbers in the line
    line = re.sub("\D", "-", line)
    line = re.sub("\-+", "-", line)
    order = line.split("-")
    order.pop(0)
    #print(line)

    #Now have : move order[0] from column order[1] to column order[2]
    processed_orders.append(order)

i = 0
for order in processed_orders:
    nbr = int(order[0])
    src = int(order[1])
    dst = int(order[2])
    print(f"\tProcessing {order}")
    print(f"Src : {stacks[src]}")
    print(f"Dst : {stacks[dst]}")

    table = []

    while nbr > 0:
        if len(stacks[src]) > 0:
            print(f"{nbr} {src} {dst}")
            table.append(stacks[src][-1])
            stacks[src].pop(-1)
            nbr -= 1
        else:
            print(f"Column {src} is empty :(")
            nbr -= 1
    table.reverse()
    for crate in table:
        stacks[dst].append(crate)
    i += 1

response = ""
for stack in stacks:
    print(stack, stacks[stack])



