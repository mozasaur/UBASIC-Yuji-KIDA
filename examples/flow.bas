10 PRINT "Start"
20 GOSUB 100
30 PRINT "Back from subroutine"
40 GOTO 200
50 PRINT "This will not be printed"
100 PRINT "In subroutine"
110 RETURN
200 PRINT "End"
210 END 