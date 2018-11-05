################################################################################
#
#	Author: Benedikt Orsolic
#	
#	Purpose: Exercise
#	
#	Usage: User is prompted
#	
#	Known issues: None known
#	
#	To do: None planed
#	
#	Date created: 
#
#	Date last edited: 
#	
################################################################################


def main():
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	Data structure
	#
	#
	############################################################
	
	
	#Number of months required to get enough for down payment
	month_count = 36
	
	
	#Cost of a home
	total_cost = 1000000.0
	#Portion of cost for down payment decimal (%/100)
	portion_down_payment = 0.25
	#Currently saved amount in dollars $
	current_savings = 0.0
	
		
	#Annual investment return decimal (%/100)
	r = 0.04
	annual_salary = 0.0
	#Portion of a salary saved in decimal (%/100)
	portion_saved = 1.00
	#How much salary increases every 6 months decimal (%/100)
	semi_annual_rais = 0.07
	#By how much down payment can be of in $
	down_payment_margin = 100.00
	
	#Number of steps in bi-sectional search
	steps = 0
	
	#Tracks if it is possible to get down payment in 36 months
	possible_down_payment = True
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	User input
	#
	#
	############################################################
	
	
	annual_salary    = float( input("Enter starting annual salary: ") )
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	Algorithm for time required for down payment
	#
	#
	############################################################
	
	
	#Variables for algorithm only
	
	
	#By how much are current savings off
	off_by = 0.0
	annual_salary_temp = annual_salary
	#Points in search in decimal ( %/100 )
	low = 0.00
	high = 1.00
	
	
	
	
	
	#Add savings monthly and count months
	while( True ):
		
		
		#Find middle 
		portion_saved = (high - low)/2.0 + low
		steps += 1
		annual_salary_temp = annual_salary
		current_savings = 0.0
		
		
		
		
		
		for month in range( 1, month_count+1 ):
			
			
			if month % 6 == 0:
				annual_salary_temp += annual_salary * semi_annual_rais
			#End if
			
			
			#Add investment return
			current_savings += current_savings*r/12.0
			#Add monthly savings
			current_savings += annual_salary_temp/12.0*portion_saved
			
			
		#End for
		
		
		
		
		
		#Positive result needs higher portion_saved
		#Negative result needs lower portion_saved 
		off_by = total_cost * portion_down_payment - current_savings
		
		
		
		
		
		#If within a margin stop
		if abs(off_by)<= down_payment_margin:
			break
		#If it reaches ~100% stop
		if portion_saved > 0.9999:
			possible_down_payment = False
			break
		#Adjust top and values for savings in search
		if off_by > 0:
			low = portion_saved
		if off_by < 0:
			high = portion_saved
		
		
	#End while
	
	
	
	
	
	
	
	############################################################
	#
	#	Block:	Program output
	#
	#
	############################################################
	
	
	#Output to the console
	if possible_down_payment:
		print( "Portion down payment for 36 months: ", portion_saved*100, "%" )
	else:
		print("It is not possible to make a down payment in {}\
		 months with salary of: ".format(month_count, annual_salary))
	
	
	print( "Number of steps: ", steps )
#End main





























#Starts from main after script loads
if __name__ == "__main__":
	main()
#End if
