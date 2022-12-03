#!/usr/bin/env python3
#coding: utf-8
from string import ascii_lowercase, ascii_uppercase

def get_input():
    with open("input.txt", "r") as inp:
        return inp.read().splitlines()

def line_compute(line: str):
    '''
    Gets the points for a line
    '''
    #The whole alphabet, lowercase and then uppercase
    letters = list(ascii_lowercase) + list(ascii_uppercase)

    left = slice(0, len(line) // 2)
    right = slice(len(line) // 2, len(line))

    left = list(line[left])
    right = list(line[right])

    for letter in left:
        if letter in right:
            common_letters = letter

    pts = letters.index(common_letters) + 1
    #print(f"{left} ---- {right} =====> {common_letters} : {pts}")
    return pts


def make_groups(lines: list, how_many: int = 3):
    '''
    Takes lines and group them by how_many
    '''

    groups, group = [], []
    nbr_of_groups = len(lines) // how_many
    
    while nbr_of_groups > 0:
        for i in range(0, how_many):
            group.append(list(lines[0]))
            lines.pop(0)
        groups.append(group)
        nbr_of_groups -= 1
        group = []

    return groups


def count_group(group: list):
    '''
    For a group, find a common element and return associated points
    '''
    letters = list(ascii_lowercase) + list(ascii_uppercase)
    pts = 0

    for letter in group[0]:
        if (letter in group[1]) and (letter in group[2]):
            pts = letters.index(letter) + 1

    return pts


def main():
    
    lines = get_input()
    total_points = 0

    #part 1
    for line in lines:
        total_points += line_compute(line)
    print(total_points)

    #part2
    total_points = 0
    groups = make_groups(lines, 3)
    for group in groups:
        total_points += count_group(group)
    print(total_points)


if __name__ == "__main__":
    main()