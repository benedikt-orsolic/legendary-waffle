/*
 * File: HangmanCanvas.java
 * ------------------------
 * This file keeps track of the Hangman display.
 */

import java.awt.Font;
import java.awt.Label;

import acm.graphics.*;

public class HangmanCanvas extends GCanvas {

/** Resets the display so that only the scaffold appears */
	public void reset() {
		
		/*	First clear the screen. */
		removeAll();
		currentGuesses = MAX_GUESSES;
		
		/*	Get upper left corner, top of the scaffold ass reference.
		 * 	Subtract font size to make room for two lines under.*/
		int refX = (getWidth() - 2*BEAM_LENGTH) / 2;
		int refY = (getHeight() - SCAFFOLD_HEIGHT)/2 - 2*WORD_FONT_SIZE;
		
		/* 	Now we add scaffold. 	*/
		drawScaffold(refX, refY);
		/* 	Now we add beam.	*/
		drawBeam(refX, refY);
		/*	Finally add rope.	*/
		drawRope(refX, refY);
		
		
		/*	Create GLabel for hidden word. */
		refY = getHeight() - 4*WORD_FONT_SIZE;
		hidden = new GLabel("");
		hidden.setLocation(refX, refY);
		hidden.setFont(new Font("Serif", Font.PLAIN, 14));
		add(hidden);
		
		
		incorrectGuess = new GLabel("");
		incorrectGuess.setLocation(refX, refY + 2*WORD_FONT_SIZE);
		incorrectGuess.setFont(new Font("Serif", Font.PLAIN, 14));
		add(incorrectGuess);
		
	}/*	End reset()	*/
	
	
	/* 	Draws scaffold on the canvas*/
	private void drawScaffold(int refX, int refY) {
		int startX = refX;
		int startY = refY;
		int endX = refX;
		int endY = refY + SCAFFOLD_HEIGHT;
		
		GLine line = new GLine(startX, startY, endX, endY);
		add(line);
	}/*	End drawScaffold()	*/
	
	
	/*	Draws beam on the screen.	*/
	private void drawBeam(int refX, int refY) {
		int startX = refX;
		int startY = refY;
		int endX = refX + BEAM_LENGTH;
		int endY = refY;
		
		GLine line = new GLine(startX, startY, endX, endY);
		add(line);
	}/*	End drawBeam()	*/
	
	
	/*	Draws rope on the screen.	*/
	private void drawRope(int refX, int refY) {
		int startX = refX + BEAM_LENGTH;
		int startY = refY;
		int endX = refX + BEAM_LENGTH;
		int endY = refY + ROPE_LENGTH;
		
		GLine line = new GLine(startX, startY, endX, endY);
		add(line);
	}/*	End drawRope()	*/
	

	/** Updates the word on the screen to correspond to the current
	 * state of the game.  The argument string shows what letters have
	 * been guessed so far; unguessed letters are indicated by hyphens.
	 */
	public void displayWord(String word) {
		
		hidden.setLabel(word);
	}

/**
 * Updates the display to correspond to an incorrect guess by the
 * user.  Calling this method causes the next body part to appear
 * on the scaffold and adds the letter to the list of incorrect
 * guesses that appears at the bottom of the window.
 */
	public void noteIncorrectGuess(char letter) {
		
		/*	Get upper left corner, top of the scaffold ass reference.
		 * 	Subtract font size to make room for two lines under.*/
		int refX = (getWidth() - 2*BEAM_LENGTH) / 2;
		int refY = (getHeight() - SCAFFOLD_HEIGHT)/2 - 2*WORD_FONT_SIZE;
		
		incorrectGuess.setLabel(incorrectGuess.getLabel() + letter);
		currentGuesses--;
		
		switch(currentGuesses) {
		case 7:
			drawHead(refX, refY);
			break;
		case 6:
			drawBody(refX, refY);
			break;
		case 5:
			drawLeftArm(refX, refY);
			break;
		case 4:
			drawRightArm(refX, refY);
			break;
		case 3:
			drawLeftLeg(refX, refY);
			break;
		case 2:
			drawRightLeg(refX, refY);
			break;
		case 1:
			drawLeftFoot(refX, refY);
			break;
		case 0:
			drawRightFoot(refX, refY);
			break;
		}
	}
	
	private void drawHead(int refX, int refY) {
		int x = refX + BEAM_LENGTH - HEAD_RADIUS;
		int y = refY + ROPE_LENGTH;
		
		GOval head = new GOval(x, y, 2*HEAD_RADIUS, 2*HEAD_RADIUS);
		add(head);
	}//End drawHead(int refX, int refY)
	private void drawBody(int refX, int refY) {
		int startX = refX + BEAM_LENGTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS;
		int endX = refX + BEAM_LENGTH;
		int endY = startY + BODY_LENGTH;
		
		GLine body = new GLine(startX, startY, endX, endY);
		add(body);
	}//End drawBody(int refX, int refY)
	private void drawLeftArm(int refX, int refY) {
		/*	Shift reference to left elbow.	*/
		int startX = refX + BEAM_LENGTH - UPPER_ARM_LENGTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + ARM_OFFSET_FROM_HEAD;
		
		int endX = startX + UPPER_ARM_LENGTH;
		int endY = startY;
		GLine upperArm = new GLine(startX, startY, endX, endY);
		add(upperArm);
		
		endX = startX;
		endY = startY + LOWER_ARM_LENGTH;
		GLine lowerArm = new GLine(startX, startY, endX, endY);
		add(lowerArm);
	}//End drawLeftArm(int refX, int refY)
	private void drawRightArm(int refX, int refY) {
		/*	Shift reference to right elbow.	*/
		int startX = refX + BEAM_LENGTH + UPPER_ARM_LENGTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + ARM_OFFSET_FROM_HEAD;
		
		int endX = startX - UPPER_ARM_LENGTH;
		int endY = startY;
		GLine upperArm = new GLine(startX, startY, endX, endY);
		add(upperArm);
		
		endX = startX;
		endY = startY + LOWER_ARM_LENGTH;
		GLine lowerArm = new GLine(startX, startY, endX, endY);
		add(lowerArm);
	}//End drawRightArm(int refX, int refY)
	private void drawLeftLeg(int refX, int refY) {
		/*	Set start coordinates to left hip.	*/
		int startX = refX + BEAM_LENGTH - HIP_WIDTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + BODY_LENGTH;
		
		int endX = startX + HIP_WIDTH;
		int endY = startY;
		GLine hip = new GLine(startX, startY, endX, endY);
		add(hip);
		
		endX = startX;
		endY = startY + LEG_LENGTH;
		GLine leg = new GLine(startX, startY, endX, endY);
		add(leg);
	}//End drawLeftLeg(int refX, int refY)
	private void drawRightLeg(int refX, int refY) {
		/*	Set start coordinates to left hip.	*/
		int startX = refX + BEAM_LENGTH + HIP_WIDTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + BODY_LENGTH;
		
		int endX = startX - HIP_WIDTH;
		int endY = startY;
		GLine hip = new GLine(startX, startY, endX, endY);
		add(hip);
		
		endX = startX;
		endY = startY + LEG_LENGTH;
		GLine leg = new GLine(startX, startY, endX, endY);
		add(leg);
	}//End drawRightLeg(int refX, int refY)
	private void drawLeftFoot(int refX, int refY) {
		int startX = refX + BEAM_LENGTH - HIP_WIDTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + BODY_LENGTH + LEG_LENGTH;
		
		int endX = startX - FOOT_LENGTH;
		int endY = startY;
		GLine foot = new GLine(startX, startY, endX, endY);
		add(foot);
	}//End drawLeftFoot(int refX, int refY)
	private void drawRightFoot(int refX, int refY) {
		int startX = refX + BEAM_LENGTH + HIP_WIDTH;
		int startY = refY + ROPE_LENGTH + 2*HEAD_RADIUS + BODY_LENGTH + LEG_LENGTH;
		
		int endX = startX + FOOT_LENGTH;
		int endY = startY;
		GLine foot = new GLine(startX, startY, endX, endY);
		add(foot);
	}//End drawRightFoot(int refX, int refY)
	
/* Constants for the simple version of the picture (in pixels) */
	private static final int SCAFFOLD_HEIGHT = 360;
	private static final int BEAM_LENGTH = 144;
	private static final int ROPE_LENGTH = 18;
	private static final int HEAD_RADIUS = 36;
	private static final int BODY_LENGTH = 144;
	private static final int ARM_OFFSET_FROM_HEAD = 28;
	private static final int UPPER_ARM_LENGTH = 72;
	private static final int LOWER_ARM_LENGTH = 44;
	private static final int HIP_WIDTH = 36;
	private static final int LEG_LENGTH = 108;
	private static final int FOOT_LENGTH = 28;
	
	private static final int WORD_FONT_SIZE = 25;
	private static final int MAX_GUESSES = 8;
	private int currentGuesses = 0;
	
	public final int WIDTH =  (int) (1.5 * 2*BEAM_LENGTH);
	public final int HEIGHT = (int) (1.5 * (SCAFFOLD_HEIGHT + 2*WORD_FONT_SIZE));
	
	GLabel hidden;	//Stores word with guessed letters.
	GLabel incorrectGuess;

}
