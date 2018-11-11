/*
 * File: HangmanLexicon.java
 * -------------------------
 * This file contains a stub implementation of the HangmanLexicon
 * class that you will reimplement for Part III of the assignment.
 */

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;

import acm.util.*;

public class HangmanLexicon {

/** Returns the number of words in the lexicon. */
	private int getWordCount() {
		try {
			BufferedReader rd = new BufferedReader(new FileReader("words"));
			while(true) {
				String line = rd.readLine();
				if(line == null)break;
				word.add(line);
			}
			rd.close();
		} catch (Exception ex) {
			throw new ErrorException(ex);
		}
		
		int numOfWords = word.size();
		return numOfWords;
	}

/** Returns the word at the specified index. */
	public String getWord() {
		int wordCount = getWordCount();
		
		RandomGenerator rgen = new RandomGenerator();
		int rndWordID = rgen.nextInt(0, wordCount-1);
		return word.get(rndWordID);
		
	}
	
	private ArrayList<String> word = new ArrayList<String>();
}
