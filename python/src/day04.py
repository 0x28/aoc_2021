#!/usr/bin/env python

def part1(numbers, cards):
    for number in numbers:
        for card in cards:
            mark(card, number)

            if has_won(card):
                return score(number, card)


def part2(numbers, cards):
    for number in numbers:
        iter_cards = cards.copy()
        for card in iter_cards:
            mark(card, number)

            if has_won(card) and len(cards) > 1:
                cards.remove(card)
            elif has_won(card) and len(cards) == 1:
                return score(number, card)

def score(number, card):
    result = 0
    for line in card:
        for cell in line:
            if cell != 'X':
                result += cell

    return result * number

def mark(card, value):
    for y, line in enumerate(card):
        for x, number in enumerate(line):
            if number == value:
                card[y][x] = 'X'

def has_won(card):
    for line in card:
        if all([cell == 'X' for cell in line]):
            return True

    for col in range(0, len(card[0])):
        if all([line[col] == 'X' for line in card]):
            return True

    return False

def parse():
    with open("../input/input04.txt") as f:
        numbers = [int(word) for word in f.readline().split(',')]
        cards = []

        for card in f.read().split("\n\n"):
            new_card = [line.split() for line in card.strip().split('\n')]
            for y, line in enumerate(new_card):
                for x, cell in enumerate(line):
                    new_card[y][x] = int(cell)

            cards.append(new_card)


        return (numbers, cards)

if __name__ == '__main__':
    (numbers, cards) = parse()
    print("part1 =", part1(numbers, cards))
    print("part2 =", part2(numbers, cards))
