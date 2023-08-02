/*
 * File: Breakout.java
 * -------------------
 * Name:
 * Section Leader:
 * 
 * This file will eventually implement the game of Breakout.
 */

import acm.graphics.*;
import acm.program.*;
import acm.util.*;

import java.applet.*;
import java.awt.*;
import java.awt.event.*;

public class Breakout extends GraphicsProgram {

	/** Width and height of application window in pixels */
	public static final int APPLICATION_WIDTH = 400;
	public static final int APPLICATION_HEIGHT = 600;

	/** Dimensions of game board (usually the same) */
	private static final int WIDTH = APPLICATION_WIDTH;
	//private static final int HEIGHT = APPLICATION_HEIGHT;

	/** Dimensions of the paddle */
	private static final int PADDLE_WIDTH = 60;
	private static final int PADDLE_HEIGHT = 10;

	/** Offset of the paddle up from the bottom */
	private static final int PADDLE_Y_OFFSET = 30;

	/** Number of bricks per row */
	private static final int NBRICKS_PER_ROW = 10;

	/** Number of rows of bricks */
	private static final int NBRICK_ROWS = 10;

	/** Separation between bricks */
	private static final int BRICK_SEP = 4;

	/** Width of a brick */
	private static final int BRICK_WIDTH =
	  (WIDTH - (NBRICKS_PER_ROW - 1) * BRICK_SEP) / NBRICKS_PER_ROW;

	/** Height of a brick */
	private static final int BRICK_HEIGHT = 8;

	/** Radius of the ball in pixels */
	private static final int BALL_RADIUS = 10;

	/** Offset of the top brick row from the top */
	private static final int BRICK_Y_OFFSET = 70;

	/** Number of turns */
	private static final int NTURNS = 3;
	
	private static final int DELAY = 15;//In miliseconds
	
		public static void main(String[] args) {
			new Breakout().start(args);
		}
	

	/* Method: run() */
	/** Runs the Breakout program. */
	public void run() {
		addMouseListeners();
		
		setUp();
		runGame();
	}//End run
	
	
	
	/******************************************************************************************************************/
	/*				RUN GAME																						  */
	/******************************************************************************************************************/
	
	private void runGame() {
		
		//Prompt player of number of turns
		GLabel prompt = new GLabel("", 0, 0);
		prompt.setFont("Dialog-24");
		
		int x = (int) (getWidth()/2 - prompt.getWidth()/2);
		int y = (int) (getHeight()/2 + prompt.getAscent()/2);
		//Move y down by 25px so it doesn't overlap with ball
		y += 25;
		
		
		for(int i = 0; i < NTURNS; i++) {
			makeBall();
			add(ball);
			
			prompt.setLabel("You have " + (NTURNS - i) + " turns left.");
			x = (int) (getWidth()/2 - prompt.getWidth()/2);
			add( prompt, x, y);
			waitForClick();	//Until user is ready to start
			remove(prompt);
			//Remove prompt after user starts playing
			
			
			while (moveBall()) 	{
				pause(DELAY);
				if (nBricksLeft == 0) {
					prompt.setLabel("You win!!");
					x = (int) (getWidth()/2 - prompt.getWidth()/2);
					add(prompt);
					break;
				}
			}//End while
		}//End for
		
		removeAll();
		if( nBricksLeft == 0 ) {
			prompt.setLabel("You win!!");
			x = (int) (getWidth()/2 - prompt.getWidth()/2);
			add(prompt);
		}else{
			prompt.setLabel("You Lose!!");
			x = (int) (getWidth()/2 - prompt.getWidth()/2);
			add(prompt);
		}
	}//End runGame
	
	
	/*	Moves ball and returns true if ball didn't go out of screen.	
	 * 	Decrements brick count if it removes one.	*/
	private boolean moveBall() {
		ball.move(vx, vy);
		
		int ballX = (int) ball.getX();
		int ballY = (int) ball.getY();
		
		GObject colider = null;	//Object ball hits
		
		/*	>>>>>>>>>>	FIND AND REMOVE OBJECT BALL HITS OR BOUNCE IF ITS PADDLE	<<<<<<<<<<	*/
		
		if (colider == null) colider = getElementAt(ballX, ballY);
		if (colider == null) colider = getElementAt(ballX, ballY + BALL_RADIUS);
		if (colider == null) colider = getElementAt(ballX + BALL_RADIUS, ballY);
		if (colider == null) colider = getElementAt(ballX + BALL_RADIUS, ballY + BALL_RADIUS);
		
		
		if (colider != null) {
			/*
			double x = colider.getX();
			double y = colider.getY();
			double height = colider.getHeight();
			double width = colider.getWidth();
			
			//If we hit horizontal surface
			if( ballX > x + (int) (0.1 * BRICK_WIDTH) && ballX + BALL_RADIUS < x + width - (int) (0.1 * BRICK_WIDTH)) {
				vy = -vy;
			} else {
				vx = -vx;
			}
			*/
			
			vy = -vy;
			if(colider != paddle) {
				remove(colider);
				nBricksLeft--;
			}
			
			bounceClip.play();
		}//End if
		
		
		/*	>>>>>>>>>>	BOUNC BALL IF IT HITS WALL OR SEND FALSE	<<<<<<<<<<	*/
		if (ballX + BALL_RADIUS > APPLICATION_WIDTH || ballX < 0) {
			vx = -vx;
			bounceClip.play();
		}
		if (ballY < 0) {
			vx = -vx;
			vy = -vy;
			bounceClip.play();
		}
		if (ballY + BALL_RADIUS > APPLICATION_HEIGHT) {
			remove(ball);
			return false;
		}
		return true;
		
	}//End moveBall
	
	public void mouseMoved(MouseEvent e) {
		int mouseX = e.getX();
		
		double newPaddleX = mouseX - paddle.getWidth()/2;
		paddle.setLocation(newPaddleX, paddle.getY());
		
	}
	/******************************************************************************************************************/
	/*				SETUP BLOCK																						  */
	/******************************************************************************************************************/
	
	private void setUp() {
		
		setSize(APPLICATION_WIDTH, APPLICATION_HEIGHT);
		pause(5);
		//Pause before adding another object to window, it doesn't set size instanteniusly
		drawBricks();
		drawPaddle();
		makeBall();
	}//End setUp
	
	
	private void drawBricks(){
		
		int offsetY = BRICK_Y_OFFSET;
		
		for(int i = 0; i < NBRICK_ROWS; i++) {
			
			//Draw rows
			drawRow(i, offsetY);
			
			/*	Increase offsetY by brick height and brick separation.	*/
			offsetY += BRICK_HEIGHT + BRICK_SEP;
			
		}//End for
		
	}//End drawBricks

	private void drawRow(int row, int offsetY) {
		//Gets number of row its drawing, so it can get the color
		/*	offsetY is wher to put row on y aces, x is calculaed here	*/
		
		int offsetX = (getWidth() - (BRICK_WIDTH + BRICK_SEP) * NBRICKS_PER_ROW + BRICK_SEP)/2 ;
		
		Color brickColor = Color.RED;
		     if (row ==  0 || row == 1) brickColor = Color.RED;
		else if (row ==  2 || row == 3) brickColor = Color.ORANGE;
		else if (row ==  4 || row == 5) brickColor = Color.YELLOW;
		else if (row ==  6 || row == 7) brickColor = Color.GREEN;
		else if (row ==  8 || row == 9) brickColor = Color.CYAN;
		     
		//Add individual bricks
		for (int i = 0; i < NBRICKS_PER_ROW; i++) {
			
			GRect brick = new GRect(BRICK_WIDTH, BRICK_HEIGHT);
			brick.setFilled(true);
			brick.setFillColor(brickColor);
			
			add(brick, offsetX, offsetY);
			
			/*	Increase offsetX by brick width and brick separation*/
			offsetX += BRICK_WIDTH + BRICK_SEP;
			
		}//End for
		
	}//End drawRow
	
	private void drawPaddle() {
		paddle = new GRect(PADDLE_WIDTH, PADDLE_HEIGHT);
		paddle.setFilled(true);
		paddle.setFillColor(Color.BLACK);
		
		int paddleX = (getWidth() - PADDLE_WIDTH)/2;
		int paddleY = getHeight() - PADDLE_Y_OFFSET - PADDLE_HEIGHT;
		
		add(paddle, paddleX, paddleY);
	}//End draw paddle
	
	private void makeBall() {
		ball = new GOval(BALL_RADIUS, BALL_RADIUS);
		ball.setFilled(true);
		ball.setFillColor(Color.BLACK);
		
		int ballX = (getWidth() - BALL_RADIUS)/2;
		int ballY = (getHeight() - BALL_RADIUS)/2;
		
		ball.setLocation(ballX, ballY);
		
		/*	Initialize ball velocity*/
		vx = rgen.nextDouble(1.0, 3.0);
		vy = rgen.nextDouble(1.0, 3.0);
		System.out.println(vx);
		if (rgen.nextBoolean(0.5)) vx = -vx;
		
	}//End drawBall
	
	
	
	/******************************************************************************************************************/
	/*				INSTANCE VARIABLES																				  */
	/******************************************************************************************************************/
	
	GRect paddle;
	GOval ball;
	
	AudioClip bounceClip = MediaTools.loadAudioClip("bounce.au"); 
	
	/*	Ball velocity, how much to move it on x and y access*/
	private RandomGenerator rgen = RandomGenerator.getInstance();
	double vx, vy;
	int nBricksLeft = NBRICK_ROWS * NBRICKS_PER_ROW;
}
