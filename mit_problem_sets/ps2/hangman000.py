# Problem Set 2, hangman.py
# Name: 
# Collaborators:
# Time spent:

# Hangman Game
# -----------------------------------
# Helper code
# You don't need to understand this helper code,
# but you will have to know how to use the functions
# (so be sure to read the docstrings!)

import random
import string

WORDLIST_FILENAME = "words.txt"
GUESSES_LEFT = 6
WARNINGS_LEFT = 3





























def load_words():
	"""
	Returns a list of valid words. Words are strings of lowercase letters.
	
	Depending on the size of the word list, this function may
	take a while to finish.
	"""
	print("Loading word list from file...")
	# inFile: file
	inFile = open(WORDLIST_FILENAME, 'r')
	# line: string
	line = inFile.readline()
	# wordlist: list of strings
	wordlist = line.split()
	print("  ", len(wordlist), "words loaded.")
	return wordlist
#End load_words






























def choose_word(wordlist):
	"""
	wordlist (list): list of words (strings)
	
	Returns a word from wordlist at random
	"""
	return random.choice(wordlist)

# end of helper code

# -----------------------------------






























# Load the list of words into the variable wordlist
# so that it can be accessed from anywhere in the program
wordlist = load_words()






























def get_guessed_word(secret_word, letters_guessed):
	'''
	secret_word: string, the word the user is guessing
	letters_guessed: list (of letters), which letters have been guessed so far
	returns: string, comprised of letters, underscores (_), and spaces that represents
	  which letters in secret_word have been guessed so far.
	'''
	
	
	
	#Data
	guessed_word = ""
	
	
	
	
	
	#Algorithm
	for i in secret_word:
		if i in letters_guessed:
			guessed_word = guessed_word + i
		else:
			guessed_word = guessed_word + "_"
	#End for
	
	
	
	
	
	#Output
	return guessed_word
#End get_guessed_word






























def get_available_letters(letters_guessed):
	'''
	letters_guessed: list (of letters), which letters have been guessed so far
	returns: string (of letters), comprised of letters that represents which letters have not
	  yet been guessed.
	'''
	
	
	
	
	#Data
	ascii_list = string.ascii_lowercase
	available_letters = ""
	
	
	
	
	#Proces
	for i in ascii_list:
		
		if i not in letters_guessed:
			available_letters = available_letters + i
	#End for
	
	
	
	
	#Return
	return available_letters
#End get_available_letters






























def get_user_input(available_letters):
	"""Asks user to enter a letter
	   returns int if letter is not in available_letters 
	   prints message to notify user what was enterd. """
	
	
	
	
	#Data
	user_input = ""
	
	
	
	
	#Algorithm
	print("Enter your guess: ")
	
	
	
	user_input = input()
	user_input = str.lower(user_input)
	
	
	
	if len(user_input) != 1:
		print("Only astronauts can enter more then one letter!")
		return 0
	
	
	
	
	if user_input not in string.ascii_lowercase:
		print("Only wizards can use non alphabet characters!")
		return 0
	
	
	
	if not user_input in available_letters:
		print("Only dragons may reuse letters!")
		return 0
	
	
	
	
	
	
	
	
	
	
	#Output
	return user_input
#End get_user_input





























def unique_letters(string):
	"""
		Returns number of unique letters in a string
	"""
	
	
	
	count = 0
	unique_list = ""
	
	
	
	for i in string:
		if not i in unique_list:
			count += 1
			unique_list = unique_list + i
	#End for
	
	
	
	return count
#End unique_list





























def hangman(secret_word):
	'''
	secret_word: string, the secret word to guess.
	
	Starts up an interactive game of Hangman.
	
	* At the start of the game, let the user know how many 
	  letters the secret_word contains and how many guesses s/he starts with.
	  
	* The user should start with 6 guesses

	* Before each round, you should display to the user how many guesses
	  s/he has left and the letters that the user has not yet guessed.
	
	* Ask the user to supply one guess per round. Remember to make
	  sure that the user puts in a letter!
	
	* The user should receive feedback immediately after each guess 
	  about whether their guess appears in the computer's word.

	* After each guess, you should display to the user the 
	  partially guessed word so far.
	
	Follows the other limitations detailed in the problem write-up.
	'''
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	Data
	#
	#	
	############################################################
	
	
	#What user needs to guess 
	secret_word = secret_word
	#Letters user tried to guess
	letters_guessed = ""
	#Letters reviled in word sofar, with "_" for letters not guessed
	guessed_word = get_guessed_word(secret_word, letters_guessed)
	available_letters = get_available_letters(letters_guessed)
	guessed_letter = ""
	
	#Number of guesses left
	guesses_left = GUESSES_LEFT
	#Number of warnings before user starts to lose guesses
	warnings_left = WARNINGS_LEFT
	#Stores current user input
	user_input = ""
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	User interaction and data procesing
	#
	#
	############################################################
	
	
	
	while ( "_" in guessed_word and guesses_left > 0 ):
		
		#Prmpt the user and get input
		print("You have " + str(guesses_left) + " guesses left.")
		available_letters = get_available_letters(letters_guessed)
		print("Available letters: " + available_letters )
		
		
		
		
		
		
		
		while True:
			guessed_letter = get_user_input(available_letters)
			
			
			
			if type(guessed_letter) == int:
				if warnings_left > 0:
					warnings_left -= 1
				else:
					guesses_left -= 1
				
				print("You have " + str(guesses_left) + " guesses and warnings " + str(warnings_left) + "!")
			
			
			
			if type(guessed_letter) == str or guesses_left == 0:
				break
		#End while
		if guesses_left == 0:
			break
		
		
		
		
		
		
		
		#Proces user input
		letters_guessed = letters_guessed + guessed_letter
		guessed_word = get_guessed_word(secret_word, letters_guessed)
		
		if guessed_letter in secret_word:
			print("Good guess: " + guessed_word)
		else:
			print("Oops, you missed: " + guessed_word)
			if guessed_letter in "aeiou":
				guesses_left -= 2
			else:
				guesses_left -= 1
		
		print("---------------")
	#End while
	
	
	if "_" not in guessed_word and guesses_left > 0:
		total_score = guesses_left * unique_letters(secret_word)
		print("You won with the score of " + str(total_score) + "!")
	else:
		print("You loose! Secret word was: " + secret_word)



# When you've completed your hangman function, scroll down to the bottom
# of the file and uncomment the first two lines to test
#(hint: you might want to pick your own
# secret_word while you're doing your own testing)


# -----------------------------------






























def match_with_gaps(my_word, other_word):
	'''
	my_word: string with _ characters, current guess of secret word
	other_word: string, regular English word
	returns: boolean, True if all the actual letters of my_word match the 
		corresponding letters of other_word, or the letter is the special symbol
		_ , and my_word and other_word are of the same length;
		False otherwise: 
	'''
	# FILL IN YOUR CODE HERE AND DELETE "pass"
	pass






























def show_possible_matches(my_word):
	'''
	my_word: string with _ characters, current guess of secret word
	returns: nothing, but should print out every word in wordlist that matches my_word
			 Keep in mind that in hangman when a letter is guessed, all the positions
			 at which that letter occurs in the secret word are revealed.
			 Therefore, the hidden letter(_ ) cannot be one of the letters in the word
			 that has already been revealed.

	'''
	# FILL IN YOUR CODE HERE AND DELETE "pass"
	pass






























def hangman_with_hints(secret_word):
	'''
	secret_word: string, the secret word to guess.
	
	Starts up an interactive game of Hangman.
	
	* At the start of the game, let the user know how many 
	  letters the secret_word contains and how many guesses s/he starts with.
	  
	* The user should start with 6 guesses
	
	* Before each round, you should display to the user how many guesses
	  s/he has left and the letters that the user has not yet guessed.
	
	* Ask the user to supply one guess per round. Make sure to check that the user guesses a letter
	  
	* The user should receive feedback immediately after each guess 
	  about whether their guess appears in the computer's word.

	* After each guess, you should display to the user the 
	  partially guessed word so far.
	  
	* If the guess is the symbol *, print out all words in wordlist that
	  matches the current guessed word. 
	
	Follows the other limitations detailed in the problem write-up.
	'''
	# FILL IN YOUR CODE HERE AND DELETE "pass"
	pass



# When you've completed your hangman_with_hint function, comment the two similar
# lines above that were used to run the hangman function, and then uncomment
# these two lines and run this file to test!
# Hint: You might want to pick your own secret_word while you're testing.


if __name__ == "__main__":
	
	
	#print( get_guessed_word("orange", "ae") )
	#print( get_available_letters("abcdeijklmnopqrstuvwxyz") )
	#print( get_user_input("abc") )
	hangman("apple")
	print("Program exit sucess")
	
	# pass
	
	# To test part 2, comment out the pass line above and
	# uncomment the following two lines.
	
	#secret_word = choose_word(wordlist)
	#hangman(secret_word)
	
###############
	
	# To test part 3 re-comment out the above lines and 
	# uncomment the following two lines. 
	
	#secret_word = choose_word(wordlist)
	#hangman_with_hints(secret_word)
	
	
	
	
	
	
	
