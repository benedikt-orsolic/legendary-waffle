# Problem Set 4B
# Name: <your name here>
# Collaborators:
# Time Spent: x:xx

import string


# HELPER CODE #
from typing import Dict


def load_words(file_name):
    """
    file_name (string): the name of the file containing 
    the list of words to load    
    
    Returns: a list of valid words. Words are strings of lowercase letters.
    
    Depending on the size of the word list, this function may
    take a while to finish.
    """

    # print("Loading word list from file...")
    # inFile: file
    in_file = open(file_name, 'r')

    # wordlist: list of strings
    wordlist = []
    for line in in_file:
        wordlist.extend([word.lower() for word in line.split(' ')])
    # print("  ", len(wordlist), "words loaded.")
    return wordlist


def is_word(word_list, word):
    """
    Determines if word is a valid word, ignoring
    capitalization and punctuation

    word_list (list): list of words in the dictionary.
    word (string): a possible word.
    
    Returns: True if word is in word_list, False otherwise

    Example:
    >>> is_word(word_list, 'bat') returns
    True
    >>> is_word(word_list, 'asdf') returns
    False
    """
    word = word.lower()
    word = word.strip(" !@#$%^&*()-_+={}[]|\:;'<>?,./\"")
    return word in word_list


def get_story_string():
    """
    Returns: a story in encrypted text.
    """
    f = open("story.txt", "r")
    story = str(f.read())
    f.close()
    return story


# END HELPER CODE #


WORD_LIST_FILENAME = 'words.txt'


class Message(object):

    message_text = ""

    valid_words = list()

    def __init__(self, text):
        """
        Initializes a Message object
                
        text (string): the message's text

        a Message object has two attributes:
            self.message_text (string, determined by input text)
            self.valid_words (list, determined using helper function load_words)
        """

        self.message_text = text
        self.valid_words = load_words(WORD_LIST_FILENAME)

    def get_message_text(self):
        """
        Used to safely access self.message_text outside of the class
        
        Returns: self.message_text
        """

        return self.message_text

    def get_valid_words(self):
        """
        Used to safely access a copy of self.valid_words outside of the class.
        This helps you avoid accidentally mutating class attributes.
        
        Returns: a COPY of self.valid_words
        """

        return self.valid_words.copy()

    def build_shift_dict(self, shift):
        """
        Creates a dictionary that can be used to apply a cipher to a letter.
        The dictionary maps every uppercase and lowercase letter to a
        character shifted down the alphabet by the input shift. The dictionary
        should have 52 keys of all the uppercase letters and all the lowercase
        letters only.

        shift (integer): the amount by which to shift every letter of the
        alphabet. 0 <= shift < 26

        Returns: a dictionary mapping a letter (string) to
                 another letter (string).
        """

        shift = shift % 26
        shift_dict: Dict[str, str] = {}

        for i in range(len(string.ascii_lowercase)):
            key_index = (i + shift) % 26
            key = string.ascii_lowercase[key_index]
            shift_dict[string.ascii_lowercase[i]] = key

        for i in range(len(string.ascii_uppercase)):
            key_index = (i + shift) % 26
            key = string.ascii_uppercase[key_index]
            shift_dict[string.ascii_uppercase[i]] = key

        return shift_dict

    def apply_shift(self, shift):
        """
        Applies the Caesar Cipher to self.message_text with the input shift.
        Creates a new string that is self.message_text shifted down the
        alphabet by some number of characters determined by the input shift        
        
        shift (integer): the shift with which to encrypt the message.
        0 <= shift < 26

        Returns: the message text (string) in which every character is shifted
             down the alphabet by the input shift
        """

        shift_dict = self.build_shift_dict(shift)
        shifted_message_text = ""

        for i in self.message_text:
            if i in string.ascii_letters:
                shifted_message_text = shifted_message_text + shift_dict[i]
            else:
                shifted_message_text = shifted_message_text + i

        return shifted_message_text


class PlaintextMessage(Message):

    shift = 0
    encryption_dict = {}
    message_text_encrypted = ""

    def __init__(self, text, shift):
        """
        Initializes a PlaintextMessage object        
        
        text (string): the message's text
        shift (integer): the shift associated with this message

        A PlaintextMessage object inherits from Message and has five attributes:
            self.message_text (string, determined by input text)
            self.valid_words (list, determined using helper function load_words)
            self.shift (integer, determined by input shift)
            self.encryption_dict (dictionary, built using shift)
            self.message_text_encrypted (string, created using shift)

        """

        Message.__init__(self, text)
        self.shift = shift
        self.encryption_dict = Message.build_shift_dict(self.shift)
        self.message_text_encrypted = Message.apply_shift(self, shift)

    def get_shift(self):
        """"
        Used to safely access self.shift outside of the class
        
        Returns: self.shift
        """

        return self.shift

    def get_encryption_dict(self):
        """"
        Used to safely access a copy self.encryption_dict outside of the class
        
        Returns: a COPY of self.encryption_dict
        """

        return self.encryption_dict.copy()

    def get_message_text_encrypted(self):
        """"
        Used to safely access self.message_text_encrypted outside of the class
        
        Returns: self.message_text_encrypted
        """

        return self.message_text_encrypted

    def change_shift(self, shift):
        """"
        Changes self.shift of the PlaintextMessage and updates other 
        attributes determined by shift.        
        
        shift (integer): the new shift that should be associated with this message.
        0 <= shift < 26

        Returns: nothing
        """

        self.shift = shift
        self.encryption_dict = Message.build_shift_dict(self.shift)
        self.message_text_encrypted = Message.apply_shift(self.shift)


class CiphertextMessage(Message):

    def __init__(self, text):
        """"
        Initializes a CiphertextMessage object
                
        text (string): the message's text

        a CiphertextMessage object has two attributes:
            self.message_text (string, determined by input text)
            self.valid_words (list, determined using helper function load_words)
        """

        Message.__init__(self, text)

    def decrypt_message(self):
        """"
        Decrypt self.message_text by trying every possible shift value
        and find the "best" one. We will define "best" as the shift that
        creates the maximum number of real words when we use apply_shift(shift)
        on the message text. If s is the original shift value used to encrypt
        the message, then we would expect 26 - s to be the best shift value 
        for decrypting it.

        Note: if multiple shifts are equally good such that they all create 
        the maximum number of valid words, you may choose any of those shifts 
        (and their corresponding decrypted messages) to return

        Returns: a tuple of the best shift value used to decrypt the message
        and the decrypted message text using that shift value
        """

        best_shift = 0
        best_count = 0
        word_list = Message.get_valid_words(self)

        for n in range(26):

            current_shift_message = Message.apply_shift(self, n).split()
            count = 0

            for word in current_shift_message:
                if is_word(word_list, word):
                    count += 1

            if count > best_count:
                best_count = count
                best_shift = n

        return (best_shift, Message.apply_shift(self, best_shift))


if __name__ == '__main__':

    #    #Example test case (PlaintextMessage)
    #    plaintext = PlaintextMessage('hello', 2)
    #    print('Expected Output: jgnnq')
    #    print('Actual Output:', plaintext.get_message_text_encrypted())
    #
    #    #Example test case (CiphertextMessage)
    #    ciphertext = CiphertextMessage('jgnnq')
    #    print('Expected Output:', (24, 'hello'))
    #    print('Actual Output:', ciphertext.decrypt_message())

    message = Message("abc")
    given = ["hello", "honey", "test", "EXAMPLE", "PRINT", "STORY", "BeSt", "HeLlO", "MoNiToR"]
    expected = ["jgnnq", "jqpga", "vguv", "GZCORNG", "RTKPV", "UVQTA", "DgUv", "JgNnQ", "OqPkVqT"]

    for i in range(len(given)):
        message = Message(given[i])
        if message.apply_shift(2) != expected[i]:
            print("FAIL! Expected: '" + expected[i] + "' But got: '" + message.apply_shift(2))

    #TODO: best shift value and unencrypted story not working
    
    message = CiphertextMessage(get_story_string())
    print(message.decrypt_message())
