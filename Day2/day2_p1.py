#!/usr/bin/env python3
#coding: utf-8


def unpack():
    '''
    Read file, unpack values
    '''
    with open("input.txt", "r") as inp:
        inp = inp.read()
    inp = inp.split("\n")
    strats = []

    for i in inp:
        if i == "":
            pass
        else:
            i = i.split(" ")
            strats.append((i[0], i[1]))
    print(strats)
    return strats


def compute_line(line: tuple):
    '''
    Compute one round of RPS
    '''

    table = ["A", "B", "C"]
    corresp = { "X" : "A", "Y" : "B", "Z" : "C" }
    pts = 0

    #trouver mon signe dans table
    # Vérifier égalité de signes, sinon
    # Si mon signe est un cran à droit de mon adv. je gagne
    # Compter les points
    me = line[1]
    opp = line[0]
    me = corresp[me]
    print(me, opp)

    if me == opp:
        pts += 3
    elif table.index(me) == (table.index(opp) + 1):
        #win
        pts += 6
    elif (table.index(me)) == 0 and (table.index(opp) == 2):
        #win
        pts += 6

    pts += (table.index(me) + 1)
    print(f"{opp} - {me} : {pts} pts")
    return pts


def main():
    strats = unpack()
    pts = 0

    for line in strats:
        pts += compute_line(line)
        
    print(pts)



if __name__ == "__main__":
    main()
