/*
 * File: Hangman.java
 * ------------------
 * This program will eventually play the Hangman game from
 * Assignment #4.
 */

import acm.graphics.*;
import acm.program.*;
import acm.util.*;

import java.awt.*;

public class Hangman extends ConsoleProgram {
	
	public static void main(String args[] ) {
		new Hangman().start();
	}
	
	
	/*	Prepare canvas and window for the game.	*/
	public void init() {
		
		
    	
		/*	Initialize canvas*/
		canvas = new HangmanCanvas();
		add(canvas);
		
		/*	Set window size and reset canvas.	*/
		setSize(canvas.WIDTH*2, canvas.HEIGHT);
		pause(5);
		
		canvas.reset();
		
	}
	
	
	/*******************************************************************************************************************
	 * 	
	 * Main run method, runs the game.
	 * 	
	 * ****************************************************************************************************************/
    public void run() {
    	
    	/*---------->	Local variables	<----------*/
		HangmanLexicon newWord = new HangmanLexicon();
		
		
		String word = newWord.getWord();
		
		String hidden = getDashes(word.length());	//What user sees, dashes replaced with correct guesses
		String incorrectGuess = "";
		
		
    	/*---------->	Welcome user and start the game.	<----------*/
		println("Welcome to Hangma.");
		int guessesLeft = N_GUESSES;
		boolean win = false;
		
		
		/*---------->	Loop until user runs out of guesses or until word is guessed.	<----------*/
		while( guessesLeft > 0 && !win) {
			
			/*	Prompt user of the word and number off guesses left.	*/
			println("The word no looks like this: " + hidden);
			canvas.displayWord(hidden);
			println("You have " + guessesLeft + " guesses left.");
			
			
			
			/*	Loop variables, character user typed in as a guess and boolean to track 
			 * 	has user made a correct guess.	*/
			char guess = readChar("Your guess: ");
			boolean wrongGuess = true;
			
			
			/*	Checks the word for the character guess that user typed in
			 * 	sets wrongGuess to false so we know user made a correct guess.	*/
			for(int j = 0; j < word.length(); j++) {
				
				char ch = word.charAt(j);
				if (ch == guess) {
					hidden = hidden.substring(0, j) + ch + hidden.substring(j+1);
					wrongGuess = false;
				}
				
			}/*_*End for*_*/
			
			
			/*	Tell the user was his guess correct.
			 * 	Decrement number of guesses left and update list of incorrect guesses.	*/
			if(wrongGuess) {
				println("There are no " + guess + "'s in the word.");
				incorrectGuess = incorrectGuess + guess;
				canvas.noteIncorrectGuess(guess);
				guessesLeft--;
			} else {
				println("That guess is corret.");
			}/*_* End if * */
			
			/*	Sets win to true if there are no dashes left, word was guessed.	*/
			if(numOfDashes(hidden) == 0) {
				win = true;
			}
			
			
		}//End while
		
		/*	Prompt user of win or loose*/
		if(win) {
			println("You guessed the word: " + word);
			println("You win.");
		} else {
			println("The word was: " + word);
			println("You loose.");
		}
		
		
	}//End run
    
    
    /* Return upper case version of users input 
     * or asks for new input until its valid.	*/
    private char readChar(String prompt) {
    	String line;
    	
    	while (true) {
    		line = readLine(prompt);
    		if (line.length() > 1) {
    			println("Invalid input. Try again.");
    		} else {
    			break;
    		}
    	}
    	
    	line = line.toUpperCase();
    	char ch = line.charAt(0);
    	
    	return ch;
    }//End readChar
    
    
    /*	Returns string made of dashes to be displayed
     * 	as a word user needs to guess. */
    private String getDashes(int length) {
    	String result = "";
    	for (int i = 0; i < length; i++) {
    		result = result + "-";
    	}
    	
    	return result;
    }//End get dashes
    
    
    /*	Returns number of dashes left in the string.	*/
    private int numOfDashes(String str) {
    	int result = 0;
    	for(int i = 0; i < str.length(); i++) {
    		if (str.charAt(i) == '-') result++;
    	}
    	
    	return result;
    }
    
    
    /*********** INSTANCE VARIABLES ***********/
    
    private HangmanCanvas canvas;
    
    /*********** CONSTANTS ***********/
    private static final int N_GUESSES = 8;
}
