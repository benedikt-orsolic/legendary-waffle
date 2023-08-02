/*
 * File: Yahtzee.java
 * ------------------
 * This program will eventually play the Yahtzee game.
 * 
 * 
 * ********************************************************************************************************************
 * 	
 * WARNING:
 * 	
 * 
 * 	scoreboard array starts from 1, not 0
 * 
 * ********************************************************************************************************************
 * TODO:
 * 	--> No known issues
 * 	
 */

import acm.io.*;
import acm.program.*;
import acm.util.*;

public class Yahtzee extends GraphicsProgram implements YahtzeeConstants {
	
	public static void main(String[] args) {
		new Yahtzee().start(args);
	}
	
	public void run() {
		setSize(APPLICATION_WIDTH, APPLICATION_HEIGHT);
		pause(5);
		
		
		IODialog dialog = getDialog();
		nPlayers = dialog.readInt("Enter number of players");
		playerNames = new String[nPlayers];
		for (int i = 1; i <= nPlayers; i++) {
			playerNames[i - 1] = dialog.readLine("Enter name for player " + i);
		}
		display = new YahtzeeDisplay(getGCanvas(), playerNames);
		
		//Make array by 1 bigger so no need to think about conversion
		scoreboard = new int[nPlayers + 1][N_CATEGORIES + 1];
		for(int i = 0; i < nPlayers + 1; i++) {
			for (int j = 0; j < N_CATEGORIES + 1; j++) {
				scoreboard[i][j] = -1;
			}
		}
		
		
		playGame();
	}

	private void playGame() {
		/* Since player has to chose a category on every turn
		 * number of rounds is equal to number of turns. */
		for(int i = 0; i < N_SCORING_CATEGORIES; i++) {
			playRound();
		}
	}
	
	private void playRound() {
		/*	In a round each player plays once.	*/
		for(int i = 1; i <= nPlayers; i++) {
			
			playersTurn(i);
		}
	}
	
	private void playersTurn(int player) {
		/*	Prompt a user of a turn.	*/
		display.printMessage("It's " + playerNames[player-1] + "'s turn.");
		/*	Keep track of dice value after tree roles so we can update score board.	*/
		int[] dice = new int[N_DICE];
		
		/*	Wait for a player to click roll. It doesn't accept 0 as a player.	*/
		display.waitForPlayerToClickRoll(player);
		
		/*	We let player to make three roles.	*/
		for(int i = N_ROLES; i > 0; i--) {
			
			display.printMessage("You have " + i + " rols left.");
			
			
			/*	Get selected dice.	*/
			boolean[] selectedDice = getSelectedDice(i);
			
			/*	Now we can role selected dice.	*/
			dice = getDiceValues(selectedDice, dice);
			
			/*	Display dice on the screen.	*/
			display.displayDice(dice);
		}
		
		/*	Now player needs to select category and we need to update scoreboard.	*/
		display.printMessage("Select category.");
		updateScoreBoard(dice, player);
	}
	
	private boolean[] getSelectedDice(int roll) {
		/*	Initialize all dice to be selected */
		boolean[] selected = {true, true, true, true, true};
		
		/*	Now, if it's first turn; roll == N_ROLES we do nothing, 
		 * 	otherwise we set selected to false if dice is not selected.	*/
		
		if (roll != N_ROLES) {


			display.waitForPlayerToSelectDice();
			for (int i = 0; i < N_DICE; i++) {
				
				if(false == display.isDieSelected(i)) selected[i] = false;
			}
		}
		
		return selected;
	}
	
	private int[] getDiceValues(boolean[] selected, int[] dice) {
		/* Just update dice values on index where selected is true. */
		for(int i = 0; i < N_DICE; i++) {
			
			if (selected[i] == true) dice[i] = rgen.nextInt(1, 6);
		}
		return dice;
	}
	
	private void updateScoreBoard(int[] dice, int player) {
		int category;
		while (true) {
			category = display.waitForPlayerToSelectCategory();
			//System.out.println("Category:" + category + "; score:" + scoreboard[player][category] + "; player:" + player);
			if (scoreboard[player][category] == -1) break;
			display.printMessage("Invalid choice, tray diferent category!");
			
		}
		
		/*	Updates scoreboard array.	*/
		updateScore(dice, player, category);
		
		/*	Update display.	*/
		for(int i = 1; i <= TOTAL; i++) {
			if (scoreboard[player][i] != -1) {
				display.updateScorecard(i, player, scoreboard[player][i]);
			}
		}
		
		updateUpperLower(player);
	}
	
	private void updateScore(int[] dice, int player, int category) {
		int score;
		
		switch (category) {
		case ONES:
			score = getUpper(dice, ONES);
			break;
		case TWOS:
			score = getUpper(dice, TWOS);
			break;
		case THREES:
			score = getUpper(dice, THREES);
			break;
		case FOURS:
			score = getUpper(dice, FOURS);
			break;
		case FIVES:
			score = getUpper(dice, FIVES);
			break;
		case SIXES:
			score = getUpper(dice, SIXES);
			break;
		case THREE_OF_A_KIND:
			score = nOfKind(dice, THREE_OF_A_KIND);
			break;
		case FOUR_OF_A_KIND:
			score = nOfKind(dice, FOUR_OF_A_KIND);
			break;
		case FULL_HOUSE:
			score = fullHouse(dice);
			break;
		case SMALL_STRAIGHT:
			score = strait(dice, SMALL_STRAIGHT);
			break;
		case LARGE_STRAIGHT:
			score = strait(dice, LARGE_STRAIGHT);
			break;
		case YAHTZEE:
			score = nOfKind(dice, YAHTZEE);
			break;
		case CHANCE:
			score = chance(dice);
			break;
		default:
			score = 0;
		}
		
		scoreboard[player][category] = score;
	}
	
	private void updateUpperLower(int player) {
		/*	Update upper score.	*/
		scoreboard[player][UPPER_SCORE] = 0;
		for (int i = 1; i < UPPER_SCORE; i++) {
			if (scoreboard[player][i] != -1) {
				scoreboard[player][UPPER_SCORE] += scoreboard[player][i];
			} else {
				/*	Not all categories are filled.	*/
				scoreboard[player][UPPER_SCORE] = -1;
				break;
			}
			
		}
		
		/*	Update lower score.	
		 * 	Starting category is THREE_OF_A_KIND 
		 * 	but categories count from 1 and indexes are from 0.	*/
		scoreboard[player][LOWER_SCORE] = 0;
		for (int i = THREE_OF_A_KIND; i < LOWER_SCORE; i++) {
			if (scoreboard[player][i] != -1) {
				scoreboard[player][LOWER_SCORE] += scoreboard[player][i];
			} else {
				/*	Not all categories are filled.	*/
				scoreboard[player][LOWER_SCORE] = -1;
				break;
			}
		}
		
		/*	If upper score is larger then 64 bonus is 35.	*/
		if (scoreboard[player][UPPER_SCORE] > 63) scoreboard[player][UPPER_BONUS] = 35;
		
		/*	Update TOTAL.*/
		scoreboard[player][TOTAL] = 0;
		for (int i = 0; i < TOTAL; i++) {
			if (scoreboard[player][i] != -1) {
				scoreboard[player][TOTAL] += scoreboard[player][i];
			}
		}
	}
	
	private int getUpper(int[] dice, int category) {
		int scoreCount = 0;
		for(int i = 0; i < N_DICE; i++) {
			if(dice[i] == category) scoreCount += dice[i];
		}
		return scoreCount;
	}
	
	private int nOfKind(int[] dice, int category) {
		int howMany = 0;
		int score = 0;
		if (category == THREE_OF_A_KIND) howMany = 3;
		if (category == FOUR_OF_A_KIND) howMany = 4;
		if (category == YAHTZEE) howMany = N_DICE;
		
		int current = 1;
		int count = 0;
		/*	First loop trough all dice values. 1-6	*/
		for(current = 1; current <= 6; current++) {
			count = 0;
			/*	Now count dice with value i.	*/
			for (int j = 0; j <  N_DICE; j++) {
				if (dice[j] == current) count++;
			}
			if(count >= howMany) break;
		}
		
		if (category == THREE_OF_A_KIND) score = count*current;
		if (category == FOUR_OF_A_KIND) score = count*current;
		if (category == YAHTZEE && count == 5) score = 50;
		
		return score;
	}
	
	private int fullHouse(int[] dice) {
		int score = 0;
		
		int current = 1;
		int count = 0;
		/*	First loop trough all dice values. 1-6	*/
		for(current = 1; current <= 6; current++) {
			count = 0;
			/*	Now count dice with value i.	*/
			for (int j = 0; j <  N_DICE; j++) {
				if (dice[j] == current) count++;
			}
			if(count >= 3) break;
		}
		
		/*	Now we can check is count is = 3, 
		 * 	if it is then we need to confirm that other two dice are same value.	*/
		int firstLeft = 0;
		if (count == 3) {
			
			for(int i = 0; i < N_DICE; i++) {
				if (dice[i] != current && firstLeft == 0) firstLeft = dice[i];
				if (firstLeft != 0 && firstLeft == dice[i]) {
					score = 25;
				}
			}
		}
		
		return score;
	}
	
	private int strait(int[] dice, int category) {
		int score = 0;
		int howMany = 0;
		if (category == SMALL_STRAIGHT) howMany = 4;
		if (category == LARGE_STRAIGHT) howMany = 5;
		
		/*	Count starts from one.
		 * 	There is always at least one dice with sequential values.
		 * 	Think of: array of size 1
		 */
		int count = 1;
		dice = sort(dice);
		
		for(int i = 0; i < N_DICE - 1; i++) {
			if(dice[i] == dice[i+1] - 1) {
				count++;
			}
			else count = 1;
		}
		
		if (category == SMALL_STRAIGHT && count >= howMany) score = 30;
		if (category == LARGE_STRAIGHT && count == howMany) score = 40;
		
		return score;
	}

	private int chance(int[] dice) {
		int score = 0;
		for(int i = 0; i < N_DICE; i++) {
			score += dice[i];
		}
		return score;
	}
	
	private int[] sort(int[] dice) {
		int temp;
		for(int i = 0; i < N_DICE; i++) {
			for(int j = i + 1; j < N_DICE; j++) {
				if(dice[i] > dice[j]) {
					temp = dice[i];
					dice[i] = dice[j];
					dice[j] = temp;
				}
			}
		}
		return dice;
	}

	
	/* Private instance variables */
	private int nPlayers;
	private String[] playerNames;
	private int[][] scoreboard;
	private YahtzeeDisplay display;
	private RandomGenerator rgen = new RandomGenerator();

}
