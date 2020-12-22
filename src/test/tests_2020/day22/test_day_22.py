import itertools
import unittest
from collections import deque
import logging

logging.basicConfig(level=logging.INFO)


def deck_to_string(deck_p1, deck_p2):
    deck_string = "".join([str(card) for card in deck_p1])
    deck_string2 = "".join([str(card) for card in deck_p2])
    total_string = deck_string + "," + deck_string2
    return total_string


def play_game(deck_p1, deck_p2, game=1, recursive=True):
    round = 1
    history = set()
    while deck_p2 and deck_p1:
        logging.debug(f'now playing round {round} for game {game}')
        logging.debug(f"player 1 deck: {deck_p1}")
        logging.debug(f"player 2 deck: {deck_p2}")

        # check if we had this configuration. if so player 1 wins
        current_deck_string = deck_to_string(deck_p1, deck_p2)
        if current_deck_string in history:
            logging.debug('Loop detected: player 1 wins!')
            winner = 1
            break
        history.add(current_deck_string)

        # drawing cards
        card_p1 = deck_p1.popleft()
        card_p2 = deck_p2.popleft()
        logging.debug(f'player 1 draws: {card_p1}')
        logging.debug(f'player 2 draws: {card_p2}')

        # check if we should cards or determine winner by high card
        if recursive and card_p1 <= len(deck_p1) and card_p2 <= len(deck_p2):
            logging.debug('time for a recursive game')
            rec_deck_p1 = deque(deque(itertools.islice(deck_p1, 0, card_p1)))
            rec_deck_p2 = deque(deque(itertools.islice(deck_p2, 0, card_p2)))

            winner, deck = play_game(rec_deck_p1, rec_deck_p2, game=game + 1, recursive=recursive)
        elif card_p1 > card_p2:
            winner = 1
        elif card_p2 > card_p1:
            winner = 2

        if winner == 1:
            logging.debug(f'player 1 wins round {round} of game {game}')
            deck_p1.append(card_p1)
            deck_p1.append(card_p2)
        if winner == 2:
            logging.debug(f'player 2 wins round {round} of game {game}')
            deck_p2.append(card_p2)
            deck_p2.append(card_p1)
        round += 1

    # game finished
    if winner == 1:
        logging.debug(f'player 1 wins game {game}')
        return winner, deck_p1
    else:
        logging.debug(f'player 2 wins game {game}')
        return winner, deck_p2


class Day22Tester(unittest.TestCase):

    def test_deque_to_string(self):
        deck_p1 = deque([9, 2, 6, 3, 1])
        deck_p2 = deque([5, 8, 4, 7, 10])
        total_string = deck_to_string(deck_p1, deck_p2)
        self.assertEqual("92631,584710", total_string)

    def test_example_a(self):
        player1 = deque([9, 2, 6, 3, 1])
        player2 = deque([5, 8, 4, 7, 10])
        winner, deck = play_game(player1, player2, recursive=False)
        deck.reverse()
        score = sum([card * value for value, card in enumerate(deck, 1)])
        self.assertEqual(306, score)

    def test_example_b(self):
        player1 = deque([9, 2, 6, 3, 1])
        player2 = deque([5, 8, 4, 7, 10])

        winner, deck = play_game(player1, player2)
        deck.reverse()
        score = sum([card * value for value, card in enumerate(deck, 1)])
        self.assertEqual(291, score)

    def test_a(self):
        player1 = deque(
            [19, 22, 43, 38, 23, 21, 2, 40, 31, 17, 27, 28, 35, 44, 41, 47, 50, 7, 39, 5, 42, 25, 33, 3, 48])
        player2 = deque([16, 24, 36, 6, 34, 11, 8, 30, 26, 15, 9, 10, 14, 1, 12, 4, 32, 13, 18, 46, 37, 29, 20, 45, 49])

        winner, deck = play_game(player1, player2, recursive=False)
        deck.reverse()
        score = sum([card * value for value, card in enumerate(deck, 1)])
        self.assertEqual(33772, score)

    def test_prevent_infinite(self):
        # these cards result in a infinite loop, see that it gets detected
        player1 = deque([43, 19])
        player2 = deque([2, 29, 14])

        winner, deck = play_game(player1, player2)
        print(deck)
        deck.reverse()
        score = sum([card * value for value, card in enumerate(deck, 1)])
        self.assertEqual(105, score)

    def test_b(self):
        player1 = deque(
            [19, 22, 43, 38, 23, 21, 2, 40, 31, 17, 27, 28, 35, 44, 41, 47, 50, 7, 39, 5, 42, 25, 33, 3, 48])
        player2 = deque(
            [16, 24, 36, 6, 34, 11, 8, 30, 26, 15, 9, 10, 14, 1, 12, 4, 32, 13, 18, 46, 37, 29, 20, 45, 49])

        winner, deck = play_game(player1, player2)
        deck.reverse()
        score = sum([card * value for value, card in enumerate(deck, 1)])
        self.assertEqual(35070, score)
