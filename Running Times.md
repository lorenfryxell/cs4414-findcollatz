#Comparative Running Times for findcollatz.rs 
>(the findcollatz.rs in this repo is referred to as "My Multi-Threaded")

###My Single-Threaded:  n = 700
	Result: 15733191  

	real	1m26.751s  
	user	1m26.107s  
	sys		0m0.101s  


###My Multi-Threaded:  n = 700, max_tasks = 3
	Result: 15733191

	real	0m41.020s
	user	1m59.012s
	sys		0m0.264s


###Provided Multi-Threaded:  n = 700, max_tasks = 7
	Result: 15733189
	real	5m40.841s
	user	9m37.860s
	sys		3m6.018s


###Provided Multi-Threaded:  n = 700, max_tasks = 3
	Result: 15733189
	real	5m40.092s
	user	9m39.877s
	sys		3m25.574s


###My Multi-Threaded:  n = 700, max_tasks = 3 (with comments)
	Process 2 finished FIRST and found valid result for value: 15733191 steps: 704
	Process 0 was told to finish and stopped at value: 21344731 steps: 309
	Process 1 was told to finish and stopped at value: 15733193 steps: 81

	Final Result: 15733191

	real	0m51.067s
	user	2m8.001s
	sys		0m0.156s

###My Multi-Threaded:  n = 700, max_tasks = 7 (with comments)   (Run 1)
	Process 0 finished FIRST and found valid result for value: 23599787 steps: 702
	Process 4 finished FIRST and found valid result for value: 15733191 steps: 704
	Process 2 finished FIRST and found valid result for value: 31466382 steps: 705
	Process 1 was told to finish and stopped at value: 42321995 steps: 328
	Process 5 was told to finish and stopped at value: 24933859 steps: 229
	Process 3 was told to finish and stopped at value: 23599790 steps: 79
	Process 6 was told to finish and stopped at value: 23599793 steps: 79
	
	Final Result: 15733191
	
	real	1m6.955s
	user	3m43.485s
	sys		0m0.378s

###My Multi-Threaded:  n = 700, max_tasks = 7 (with comments)   (Run 2)
	Process 0 finished FIRST and found valid result for value: 23599787 steps: 702
	Process 4 finished FIRST and found valid result for value: 15733191 steps: 704
	Process 3 was told to finish and stopped at value: 27487597 steps: 131
	Process 2 was told to finish and stopped at value: 27545360 steps: 224
	Process 1 was told to finish and stopped at value: 23599788 steps: 79
	Process 5 was told to finish and stopped at value: 23599792 steps: 79
	Process 6 was told to finish and stopped at value: 23599793 steps: 79
	
	Final Result: 15733191
	
	real	1m3.827s
	user	3m7.467s
	sys		0m0.293s