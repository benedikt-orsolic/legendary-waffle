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
# TODO minor buggs with hint system, currently takes warnings of with n hints still left
# 

import random
import string

WORDLIST_FILENAME = "words.txt"
MAX_GUESSES_LEFT = 6
MAX_WARNINGS_LEFT = 3
MAX_HINTS_LEFT = 2



ERROR_000 = "You can only enter one letter a time! "
ERROR_001 = "You can only enter alphabetical characters! "
ERROR_002 = "You can not reuse characters! "
ERROR_003 = "Why you can't get the answer yourself!?"









def load_words():
	"""
	Returns a list of valid words. Words are strings of lowercase letters..
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










# Load the list of words into the variable wordlist
# so that it can be accessed from anywhere in the program
wordlist = load_words()



# end of helper code

# -----------------------------------











def get_guessed_word(secret_word, letters_guessed):
	'''
	secret_word: string, the word the user is guessing
	letters_guessed: list (of letters), which have been guessed
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










def count_unique(secret_word):
	"""
	Returns int of unique letters.
	"""
	
	
	count = 0
	unique = ""
	
	
	
	for i in secret_word:
		if not i in unique:
			unique = unique + i
			count += 1
		#End if
	#End for
	
	
	
	return count
#End count_unique(secret_word)










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
	
	
	
	
	#What user needs to guess 
	secret_word = choose_word(wordlist)
	#Letters user tried to guess
	letters_guessed = ""
	#Letters reviled in word sofar, with "_" for letters not guessed
	guessed_word = get_guessed_word(secret_word, letters_guessed)
	available_letters = get_available_letters(letters_guessed)
	#Also checkt to be a valid letter
	user_input = ""
	
	#Number of guesses left
	guesses_left = MAX_GUESSES_LEFT
	#Number of warnings left
	warnings_left = MAX_WARNINGS_LEFT
	#Number of hints left
	hints_left = MAX_HINTS_LEFT
	#Input fail
	user_input_fail = False
	
	
	
	print( "Secret word looks like: " + guessed_word )
	print( "You are starting with: " + str( hints_left ))
	
	
	while ( "_" in guessed_word and guesses_left > 0 ):
		
		
		
		while guesses_left > 0:
			
			
			
			#Prmpt the user and get input
			print("You have " + str(guesses_left) + " guesses and " + str(warnings_left) + " warnings left.")
			available_letters = get_available_letters(letters_guessed)
			print("Available letters: " + available_letters )
			
			
			
			user_input = str.lower( input("\n") )
			user_input_fail = False
			#Continue to new line
			print()
			
			
			if user_input == "*":
				if hints_left > 0:
					show_hints(guessed_word )
					hints_left -= 1
					print("Now you have: " + str(hints_left) + " hint left!")
				else:
					print( ERROR_003 )
					user_input_fail = True
			else:
				if len( user_input ) != 1:
					print( ERROR_000, end="" )
					user_input_fail = True
				elif not str.isalpha( user_input ):
					print( ERROR_001, end="" )
					user_input_fail = True
				elif not user_input in available_letters:
					print( ERROR_002, end="" )
					user_input_fail = True
			
			
			#For spacing
			print()
			if user_input_fail:
				if warnings_left > 0:
					warnings_left -= 1
				else:
					guesses_left -= 1
			else:
				break
			
		#End while
		
		
		
		#Proces user input
		letters_guessed = letters_guessed + user_input
		guessed_word = get_guessed_word(secret_word, letters_guessed)
		
		if user_input == "*":
			#Just skip
			continue
		if user_input in secret_word:
			print("Good guess: " + guessed_word)
		else:
			print("Oops, you missed: " + guessed_word)
			if user_input in "aeiou":
				guesses_left -= 2
			else:
				guesses_left -= 1
		
		print("---------------")
	#End while
	
	
	
	if "_" not in guessed_word and guesses_left > 0:
		print("You won with a score of: " + str(guesses_left * count_unique(secret_word)) + "!")
	else:
		print("You loose! The word was: \"" + secret_word + "\"")
#End hangman










def show_hints( guessed_word ):
	"""
	Finds and prints all posible matches letter to letter where "_" can
	be any ltter.
	"""
	
	
	
	result = []
	temp = ""
	add_temp_to_result = True
	count = 0
	
	
	
	for word in wordlist:
		if len( word ) == len( guessed_word ):
			
			
			
			add_temp_to_result = True
			
			
			
			for i in range( len(guessed_word) ):
				if guessed_word[i] != "_" and  guessed_word[i] != word[i]:
					add_temp_to_result = False
					break
				#End if
			#End For
			
			
			
			if add_temp_to_result:
				if count % 13 == 0:
					print()
				print( word, end="  ")
				count += 1
				result.append( word )
			#End if
			
			
			
		#End if
	#End for
	
	print(end="\n\n")
	
	return result
#End show_hints( guessed_word )










if __name__ == "__main__":
	#Run after program loads
	hangman("")
	print("Program exit sucess")
	
	
	
	
	
	
	
	
