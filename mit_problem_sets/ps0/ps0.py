"""Use tabs, not space 4 times!!!!!"""

import math






























def main():
	"""	Asks for input of 2 numbers and prints:
	X raisd to power of Y,
	base 2 logarithm of X."""
	
	x = int( input("Enter number x: "))
	y = int( input("Enter number y: "))
	
	result = x**y
	print( "X**Y = ", result )
	
	result = log2( x )
	print( "log2( x ) = ", result )
#Enf main





























#Starts the program at main when scirpt loads
if __name__ == "__main__":
	main()






























